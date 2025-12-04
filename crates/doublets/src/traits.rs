use crate::{Error, Flow, Index, Link, ReadHandler, Result, WriteHandler};

/// Core trait for doublets storage operations
///
/// Provides low-level CRUD operations on links
pub trait Links<T: Index>: Send + Sync {
  /// Count links matching a query
  ///
  /// Query format: [index?, source?, target?]
  /// Use T::ANY for wildcards
  fn count<const N: usize>(&self, query: [T; N]) -> T;

  /// Create a new link
  ///
  /// Query format: [source?, target?] or []
  /// Empty query creates a point link
  fn create<const N: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N],
    handler: &mut H,
  ) -> Result<Flow, T>;

  /// Iterate over links matching a query
  fn each<const N: usize, H: ReadHandler<T>>(
    &self,
    query: [T; N],
    handler: &mut H,
  ) -> Flow;

  /// Update links matching a query
  ///
  /// Query identifies which links to update
  /// Change specifies new values
  fn update<const N1: usize, const N2: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N1],
    change: [T; N2],
    handler: &mut H,
  ) -> Result<Flow, T>;

  /// Delete links matching a query
  fn delete<const N: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N],
    handler: &mut H,
  ) -> Result<Flow, T>;

  /// Get a specific link by index
  fn get(&self, index: T) -> Option<Link<T>>;
}

/// High-level doublets operations
///
/// Extends Links with convenient methods for common operations
pub trait Doublets<T: Index>: Links<T> {
  /// Count all links in the store
  #[inline]
  fn count_all(&self) -> T {
    self.count([])
  }

  /// Count links matching a specific query
  #[inline]
  fn count_by<const N: usize>(&self, query: [T; N]) -> T {
    self.count(query)
  }

  /// Create a new link and return its index
  fn create_link(&mut self, source: T, target: T) -> Result<T, T> {
    let mut result = T::ZERO;
    self.create(
      [source, target],
      &mut |_before: Link<T>, after: Link<T>| {
        result = after.index;
        Flow::Continue
      },
    )?;
    Ok(result)
  }

  /// Create a point link (source = target = index)
  fn create_point(&mut self) -> Result<T, T> {
    let mut index = T::ZERO;
    self.create([], &mut |_before: Link<T>, after: Link<T>| {
      index = after.index;
      Flow::Continue
    })?;
    self.update(
      [index],
      [index, index, index],
      &mut |_before: Link<T>, _after: Link<T>| Flow::Continue,
    )?;
    Ok(index)
  }

  /// Update a specific link
  fn update_link(&mut self, index: T, source: T, target: T) -> Result<T, T> {
    let mut result = T::ZERO;
    self.update(
      [index],
      [index, source, target],
      &mut |_before: Link<T>, after: Link<T>| {
        result = after.index;
        Flow::Continue
      },
    )?;
    Ok(result)
  }

  /// Delete a specific link
  fn delete_link(&mut self, index: T) -> Result<T, T> {
    let mut result = T::ZERO;
    self.delete([index], &mut |before: Link<T>, _after: Link<T>| {
      result = before.index;
      Flow::Continue
    })?;
    Ok(result)
  }

  /// Search for a link with specific source and target
  fn search(&self, source: T, target: T) -> Option<T> {
    let mut result = None;
    self.each([T::ANY, source, target], &mut |link: Link<T>| {
      result = Some(link.index);
      Flow::Break
    });
    result
  }

  /// Get or create a link with specific source and target
  fn get_or_create(&mut self, source: T, target: T) -> Result<T, T> {
    if let Some(existing) = self.search(source, target) {
      Ok(existing)
    } else {
      self.create_link(source, target)
    }
  }

  /// Count usages of a link (as source or target)
  fn count_usages(&self, index: T) -> Result<T, T> {
    let link = self.get(index).ok_or(Error::NotExists(index))?;

    let mut usage_source = self.count([T::ANY, index, T::ANY]);
    if index == link.source {
      usage_source = usage_source.checked_sub_one().unwrap_or(usage_source);
    }

    let mut usage_target = self.count([T::ANY, T::ANY, index]);
    if index == link.target {
      usage_target = usage_target.checked_sub_one().unwrap_or(usage_target);
    }

    Ok(T::from_usize(usage_source.as_usize() + usage_target.as_usize()))
  }

  /// Check if a link has any usages
  fn has_usages(&self, index: T) -> bool {
    self.count_usages(index).map(|count| !count.is_zero()).unwrap_or(false)
  }

  /// Rebase: replace all occurrences of old with new
  fn rebase(&mut self, old: T, new: T) -> Result<T, T> {
    if old == new {
      return Ok(new);
    }
    let _ = self.get(old).ok_or(Error::NotExists(old))?;

    let mut to_update = Vec::new();
    self.each([T::ANY, old, T::ANY], &mut |link: Link<T>| {
      if link.index != old {
        to_update.push((link.index, new, link.target));
      }
      Flow::Continue
    });
    self.each([T::ANY, T::ANY, old], &mut |link: Link<T>| {
      if link.index != old {
        to_update.push((link.index, link.source, new));
      }
      Flow::Continue
    });

    for (index, source, target) in to_update {
      self.update_link(index, source, target)?;
    }

    Ok(new)
  }

  /// Rebase and then delete the old link
  fn rebase_and_delete(&mut self, old: T, new: T) -> Result<T, T> {
    if old == new {
      Ok(new)
    } else {
      self.rebase(old, new)?;
      self.delete_link(old)
    }
  }

  /// Collect all links into a vector
  fn collect_all(&self) -> Vec<Link<T>> {
    let count = self.count_all().as_usize();
    let mut result = Vec::with_capacity(count);
    self.each([], &mut |link| {
      result.push(link);
      Flow::Continue
    });
    result
  }

  /// Iterate over all links
  fn iter(&self) -> impl Iterator<Item = Link<T>> {
    self.collect_all().into_iter()
  }
}

impl<T: Index, S: Links<T> + ?Sized> Doublets<T> for S {}
