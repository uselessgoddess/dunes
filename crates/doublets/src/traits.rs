use crate::{Error, Flow, Index, Link, ReadHandler, Result, WriteHandler};

/// Core trait for doublets storage operations
///
/// Provides low-level CRUD operations on links
pub trait Links<L: Index>: Send + Sync {
  /// Count links matching a query
  ///
  /// Query format: [index?, source?, target?]
  /// Use L::ANY for wildcards
  fn count<const N: usize>(&self, query: [L; N]) -> L;

  /// Create a new link
  ///
  /// Query format: [source?, target?] or []
  /// Empty query creates a point link
  fn create<const N: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N],
    handler: &mut H,
  ) -> Result<Flow, L>;

  /// Iterate over links matching a query
  fn each<const N: usize, H: ReadHandler<L>>(
    &self,
    query: [L; N],
    handler: &mut H,
  ) -> Flow;

  /// Update links matching a query
  ///
  /// Query identifies which links to update
  /// Change specifies new values
  fn update<const N1: usize, const N2: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N1],
    change: [L; N2],
    handler: &mut H,
  ) -> Result<Flow, L>;

  /// Delete links matching a query
  fn delete<const N: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N],
    handler: &mut H,
  ) -> Result<Flow, L>;

  /// Get a specific link by index
  fn get(&self, index: L) -> Option<Link<L>>;
}

/// High-level doublets operations
///
/// Extends Links with convenient methods for common operations
pub trait Doublets<L: Index>: Links<L> {
  /// Count all links in the store
  #[inline]
  fn count_all(&self) -> L {
    self.count([])
  }

  /// Count links matching a specific query
  #[inline]
  fn count_by<const N: usize>(&self, query: [L; N]) -> L {
    self.count(query)
  }

  /// Create a new link and return its index
  fn create_link(&mut self, source: L, target: L) -> Result<L, L> {
    let mut result = L::ZERO;
    self.create(
      [source, target],
      &mut |_before: Link<L>, after: Link<L>| {
        result = after.index;
        Flow::Continue
      },
    )?;
    Ok(result)
  }

  /// Create a point link (source = target = index)
  fn create_point(&mut self) -> Result<L, L> {
    let mut index = L::ZERO;
    self.create([], &mut |_before: Link<L>, after: Link<L>| {
      index = after.index;
      Flow::Continue
    })?;
    self.update(
      [index],
      [index, index, index],
      &mut |_before: Link<L>, _after: Link<L>| Flow::Continue,
    )?;
    Ok(index)
  }

  /// Update a specific link
  fn update_link(&mut self, index: L, source: L, target: L) -> Result<L, L> {
    let mut result = L::ZERO;
    self.update(
      [index],
      [index, source, target],
      &mut |_before: Link<L>, after: Link<L>| {
        result = after.index;
        Flow::Continue
      },
    )?;
    Ok(result)
  }

  /// Delete a specific link
  fn delete_link(&mut self, index: L) -> Result<L, L> {
    let mut result = L::ZERO;
    self.delete([index], &mut |before: Link<L>, _after: Link<L>| {
      result = before.index;
      Flow::Continue
    })?;
    Ok(result)
  }

  /// Search for a link with specific source and target
  fn search(&self, source: L, target: L) -> Option<L> {
    let mut result = None;
    self.each([L::ANY, source, target], &mut |link: Link<L>| {
      result = Some(link.index);
      Flow::Break
    });
    result
  }

  /// Get or create a link with specific source and target
  fn get_or_create(&mut self, source: L, target: L) -> Result<L, L> {
    if let Some(existing) = self.search(source, target) {
      Ok(existing)
    } else {
      self.create_link(source, target)
    }
  }

  /// Count usages of a link (as source or target)
  fn count_usages(&self, index: L) -> Result<L, L> {
    let link = self.get(index).ok_or(Error::NotExists(index))?;

    let mut usage_source = self.count([L::ANY, index, L::ANY]);
    if index == link.source {
      usage_source = usage_source.checked_sub_one().unwrap_or(usage_source);
    }

    let mut usage_target = self.count([L::ANY, L::ANY, index]);
    if index == link.target {
      usage_target = usage_target.checked_sub_one().unwrap_or(usage_target);
    }

    Ok(L::from_usize(usage_source.as_usize() + usage_target.as_usize()))
  }

  /// Check if a link has any usages
  fn has_usages(&self, index: L) -> bool {
    self.count_usages(index).map(|count| !count.is_zero()).unwrap_or(false)
  }

  /// Rebase: replace all occurrences of old with new
  fn rebase(&mut self, old: L, new: L) -> Result<L, L> {
    if old == new {
      return Ok(new);
    }
    let _ = self.get(old).ok_or(Error::NotExists(old))?;

    let mut to_update = Vec::new();
    self.each([L::ANY, old, L::ANY], &mut |link: Link<L>| {
      if link.index != old {
        to_update.push((link.index, new, link.target));
      }
      Flow::Continue
    });
    self.each([L::ANY, L::ANY, old], &mut |link: Link<L>| {
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
  fn rebase_and_delete(&mut self, old: L, new: L) -> Result<L, L> {
    if old == new {
      Ok(new)
    } else {
      self.rebase(old, new)?;
      self.delete_link(old)
    }
  }

  /// Collect all links into a vector
  fn collect_all(&self) -> Vec<Link<L>> {
    let count = self.count_all().as_usize();
    let mut result = Vec::with_capacity(count);
    self.each([], &mut |link| {
      result.push(link);
      Flow::Continue
    });
    result
  }

  /// Iterate over all links
  fn iter(&self) -> impl Iterator<Item = Link<L>> {
    self.collect_all().into_iter()
  }
}

impl<L: Index, S: Links<L> + ?Sized> Doublets<L> for S {}
