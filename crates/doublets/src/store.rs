use {
  crate::{
    Constants, DoubletsError, Flow, Link, LinkIndex, Links, ReadHandler,
    Result, WriteHandler,
  },
  mem::{Alloc, RawMem},
};

/// Raw link data stored in memory
///
/// Stores source and target for a link. Tree navigation is handled
/// separately. We use a special marker in source to indicate if this
/// link is in the free list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct RawLink {
  source: usize,
  target: usize,
  // Internal flag: if this is usize::MAX, link is in free list
  is_free: usize,
}

unsafe impl bytemuck::Pod for RawLink {}
unsafe impl bytemuck::Zeroable for RawLink {}

/// Doublets store implementation using modern memory management
///
/// This is a simplified implementation that stores links in a flat
/// array. More sophisticated indexing (like the old tree-based
/// approach) can be added later.
pub struct DoubletsStore<T, M = Alloc<RawLink>>
where
  T: LinkIndex,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  mem: M,
  constants: Constants<T>,
  allocated: usize,
  free_count: usize,
  first_free: Option<usize>,
}

impl<T, M> DoubletsStore<T, M>
where
  T: LinkIndex,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  /// Create a new doublets store with default capacity
  pub fn new(mut mem: M) -> Result<Self, T> {
    // Initial allocation - must call .zeroed() to mark memory as initialized
    mem.grow(1024).map_err(|_| DoubletsError::AllocationFailed)?.zeroed();

    Ok(Self {
      mem,
      constants: Constants::new(1024),
      allocated: 1, // Start at 1, reserve 0 for header/special use
      free_count: 0,
      first_free: None,
    })
  }

  /// Get a raw link from memory
  #[inline]
  fn get_raw(&self, index: usize) -> Option<&RawLink> {
    let slice = self.mem.as_slice();
    slice.get(index)
  }

  /// Get a mutable raw link from memory
  #[inline]
  fn get_raw_mut(&mut self, index: usize) -> Option<&mut RawLink> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(index)
  }

  /// Check if a link exists and is not in free list
  fn exists(&self, index: T) -> bool {
    let idx = index.as_usize();
    if index.is_zero() || idx >= self.allocated {
      return false;
    }

    // Check if it's in use (not in free list)
    if let Some(raw) = self.get_raw(idx) {
      raw.is_free != usize::MAX
    } else {
      false
    }
  }

  /// Allocate a new link index
  fn allocate_index(&mut self) -> Result<T, T> {
    // Try to use free list first
    if let Some(free_index) = self.first_free {
      let next_free = if let Some(raw) = self.get_raw(free_index) {
        if raw.source == 0 { None } else { Some(raw.source) }
      } else {
        None
      };

      // Clear the is_free flag when reusing
      if let Some(raw) = self.get_raw_mut(free_index) {
        raw.is_free = 0;
      }

      self.first_free = next_free;
      self.free_count -= 1;
      return Ok(T::from_usize(free_index));
    }

    // Allocate new index
    let index = self.allocated;
    self.allocated += 1;

    // Ensure we have enough memory
    if self.allocated >= self.mem.as_slice().len() {
      let current_len = self.mem.as_slice().len();
      let addition = current_len; // Double the capacity
      self
        .mem
        .grow(addition)
        .map_err(|_| DoubletsError::AllocationFailed)?
        .zeroed();
      self.constants = Constants::new(current_len + addition);
    }

    // Initialize the new link slot
    if let Some(raw) = self.get_raw_mut(index) {
      raw.source = 0;
      raw.target = 0;
      raw.is_free = 0;
    }

    Ok(T::from_usize(index))
  }

  /// Free a link index
  fn free_index(&mut self, index: T) {
    let idx = index.as_usize();
    let next_free = self.first_free.unwrap_or(0);

    if let Some(raw) = self.get_raw_mut(idx) {
      // Add to free list
      raw.source = next_free;
      raw.target = 0;
      raw.is_free = usize::MAX; // Mark as freed
    }

    self.first_free = Some(idx);
    self.free_count += 1;
  }

  /// Count all non-free links
  fn count_total(&self) -> usize {
    self.allocated - self.free_count - 1
  }
}

impl<T, M> Links<T> for DoubletsStore<T, M>
where
  T: LinkIndex,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  fn constants(&self) -> &Constants<T> {
    &self.constants
  }

  fn count(&self, query: &[T]) -> T {
    let any = self.constants.any;

    match query.len() {
      0 => T::from_usize(self.count_total()),
      1 => {
        let index = query[0];
        if index == any {
          T::from_usize(self.count_total())
        } else if self.exists(index) {
          T::from_usize(1)
        } else {
          T::zero()
        }
      }
      _ => {
        let mut count = 0;
        self.each(query, &mut |_| {
          count += 1;
          Flow::Continue
        });
        T::from_usize(count)
      }
    }
  }

  fn create(
    &mut self,
    query: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T> {
    let index = self.allocate_index()?;
    let before = Link::nothing();

    let (source, target) = match query.len() {
      0 => (T::zero(), T::zero()),
      1 => (query[0], query[0]),
      2 => (query[0], query[1]),
      _ => (query[0], query[1]),
    };

    // Set the link data
    if let Some(raw) = self.get_raw_mut(index.as_usize()) {
      raw.source = source.as_usize();
      raw.target = target.as_usize();
      raw.is_free = 0; // Ensure it's not marked as free
    }

    let after = Link::new(index, source, target);
    Ok(handler(before, after))
  }

  fn each(&self, query: &[T], handler: ReadHandler<'_, T>) -> Flow {
    let any = self.constants.any;

    if query.is_empty() {
      // Iterate all links
      for i in 1..self.allocated {
        let index = T::from_usize(i);
        if self.exists(index)
          && let Some(raw) = self.get_raw(i)
        {
          let source = T::from_usize(raw.source);
          let target = T::from_usize(raw.target);
          let link = Link::new(index, source, target);
          if handler(link) == Flow::Break {
            return Flow::Break;
          }
        }
      }
      return Flow::Continue;
    }

    let index_query = query[0];

    if query.len() == 1 {
      if index_query == any {
        return self.each(&[], handler);
      } else if self.exists(index_query)
        && let Some(raw) = self.get_raw(index_query.as_usize())
      {
        let source = T::from_usize(raw.source);
        let target = T::from_usize(raw.target);
        return handler(Link::new(index_query, source, target));
      }
      return Flow::Continue;
    }

    let source = if query.len() >= 2 { query[1] } else { any };
    let target = if query.len() >= 3 { query[2] } else { any };

    // Query format: [index, source, target] where any component can be 'any'
    for i in 1..self.allocated {
      let index = T::from_usize(i);
      if !self.exists(index) {
        continue;
      }

      let raw = match self.get_raw(i) {
        Some(r) => r,
        None => continue,
      };

      let raw_source = T::from_usize(raw.source);
      let raw_target = T::from_usize(raw.target);

      let matches = (index_query == any || index_query == index)
        && (source == any || source == raw_source)
        && (target == any || target == raw_target);

      if matches {
        let link = Link::new(index, raw_source, raw_target);
        if handler(link) == Flow::Break {
          return Flow::Break;
        }
      }
    }

    Flow::Continue
  }

  fn update(
    &mut self,
    query: &[T],
    change: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T> {
    if query.is_empty() || change.is_empty() {
      return Err(DoubletsError::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(DoubletsError::NotExists(index));
    }

    let before = self.get(index).ok_or(DoubletsError::NotExists(index))?;

    let new_source = if change.len() >= 2 { change[1] } else { before.source };
    let new_target = if change.len() >= 3 { change[2] } else { before.target };

    // Update the link
    if let Some(raw) = self.get_raw_mut(index.as_usize()) {
      raw.source = new_source.as_usize();
      raw.target = new_target.as_usize();
    }

    let after = Link::new(index, new_source, new_target);
    Ok(handler(before, after))
  }

  fn delete(
    &mut self,
    query: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T> {
    if query.is_empty() {
      return Err(DoubletsError::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(DoubletsError::NotExists(index));
    }

    let before = self.get(index).ok_or(DoubletsError::NotExists(index))?;

    self.free_index(index);

    let after = Link::nothing();
    Ok(handler(before, after))
  }

  fn get(&self, index: T) -> Option<Link<T>> {
    if !self.exists(index) {
      return None;
    }

    let raw = self.get_raw(index.as_usize())?;
    let source = T::from_usize(raw.source);
    let target = T::from_usize(raw.target);
    Some(Link::new(index, source, target))
  }
}

/// Create a doublets store with heap allocation
pub fn create_heap_store<T>() -> Result<DoubletsStore<T, Alloc<RawLink>>, T>
where
  T: LinkIndex,
{
  DoubletsStore::new(Alloc::new())
}
