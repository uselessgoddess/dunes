use crate::{
  Error, Flow, Index, Link, Links, ReadHandler, Result, WriteHandler,
};

use {
  mem::{Alloc, RawMem},
  trees::{AdaptiveRadix, Node, SizeBalanced, Tree},
};

/// Marker trait for tree strategies that can insert and remove from trees
pub trait TreeStrategy<T: trees::Idx>: Send + Sync {
  /// Insert into tree using this strategy
  fn insert<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>;

  /// Remove from tree using this strategy
  fn remove<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>;
}

/// Size-Balanced Tree strategy marker
pub struct SbtStrategy;

impl<T: trees::Idx> TreeStrategy<T> for SbtStrategy {
  fn insert<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>,
  {
    SizeBalanced::insert_sbt(tree, root, idx)
  }

  fn remove<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>,
  {
    SizeBalanced::remove_sbt(tree, root, idx)
  }
}

/// Adaptive Radix Tree strategy marker
pub struct ArtStrategy;

impl<T: trees::Idx> TreeStrategy<T> for ArtStrategy {
  fn insert<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>,
  {
    AdaptiveRadix::insert_art(tree, root, idx)
  }

  fn remove<Tr>(tree: &mut Tr, root: Option<T>, idx: T) -> Option<T>
  where
    Tr: Tree<T> + SizeBalanced<T> + AdaptiveRadix<T>,
  {
    AdaptiveRadix::remove_art(tree, root, idx)
  }
}

/// Query/change array arity constants for method signatures
const NC_SOURCE: usize = 2; // Change includes source
const NC_TARGET: usize = 3; // Change includes target

/// Raw link data stored in memory with tree navigation
///
/// Stores source, target, and tree index information for efficient
/// searching by source and target using size-balanced trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct RawLink {
  source: usize,
  target: usize,
  /// Tree node for indexing by source
  source_tree: Node<usize>,
  /// Tree node for indexing by target
  target_tree: Node<usize>,
  /// Special marker: usize::MAX if in free list, 0 otherwise
  is_free: usize,
}

unsafe impl bytemuck::Pod for RawLink {}
unsafe impl bytemuck::Zeroable for RawLink {}

/// Helper struct to implement Tree trait for source indexing with
/// configurable strategy
struct SourceTree<'a, M: RawMem<Item = RawLink>, S> {
  mem: &'a mut M,
  _strategy: core::marker::PhantomData<S>,
}

impl<'a, M: RawMem<Item = RawLink>, S> SourceTree<'a, M, S> {
  fn new(mem: &'a mut M) -> Self {
    Self { mem, _strategy: core::marker::PhantomData }
  }
}

impl<'a, M: RawMem<Item = RawLink>, S> Tree<usize> for SourceTree<'a, M, S> {
  fn get(&self, idx: usize) -> Option<Node<usize>> {
    let slice = self.mem.as_slice();
    slice.get(idx).map(|raw| raw.source_tree)
  }

  fn set(&mut self, idx: usize, node: Node<usize>) {
    let slice = self.mem.as_mut_slice();
    if let Some(raw) = slice.get_mut(idx) {
      raw.source_tree = node;
    }
  }

  fn left_mut(&mut self, idx: usize) -> Option<&mut usize> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(idx).and_then(|raw| raw.source_tree.left.as_mut())
  }

  fn right_mut(&mut self, idx: usize) -> Option<&mut usize> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(idx).and_then(|raw| raw.source_tree.right.as_mut())
  }

  fn is_left_of(&self, first: usize, second: usize) -> bool {
    let slice = self.mem.as_slice();
    if let (Some(a), Some(b)) = (slice.get(first), slice.get(second)) {
      // Compare by (source, target) tuple for source tree
      (a.source, a.target) < (b.source, b.target)
    } else {
      first < second
    }
  }
}

// Implement SizeBalanced for all strategies (required by trait bounds)
impl<'a, M: RawMem<Item = RawLink>, S> SizeBalanced<usize>
  for SourceTree<'a, M, S>
{
}

// Implement AdaptiveRadix for all strategies (required by trait bounds)
impl<'a, M: RawMem<Item = RawLink>, S> AdaptiveRadix<usize>
  for SourceTree<'a, M, S>
{
}

/// Helper struct to implement Tree trait for target indexing with
/// configurable strategy
struct TargetTree<'a, M: RawMem<Item = RawLink>, S> {
  mem: &'a mut M,
  _strategy: core::marker::PhantomData<S>,
}

impl<'a, M: RawMem<Item = RawLink>, S> TargetTree<'a, M, S> {
  fn new(mem: &'a mut M) -> Self {
    Self { mem, _strategy: core::marker::PhantomData }
  }
}

impl<'a, M: RawMem<Item = RawLink>, S> Tree<usize> for TargetTree<'a, M, S> {
  fn get(&self, idx: usize) -> Option<Node<usize>> {
    let slice = self.mem.as_slice();
    slice.get(idx).map(|raw| raw.target_tree)
  }

  fn set(&mut self, idx: usize, node: Node<usize>) {
    let slice = self.mem.as_mut_slice();
    if let Some(raw) = slice.get_mut(idx) {
      raw.target_tree = node;
    }
  }

  fn left_mut(&mut self, idx: usize) -> Option<&mut usize> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(idx).and_then(|raw| raw.target_tree.left.as_mut())
  }

  fn right_mut(&mut self, idx: usize) -> Option<&mut usize> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(idx).and_then(|raw| raw.target_tree.right.as_mut())
  }

  fn is_left_of(&self, first: usize, second: usize) -> bool {
    let slice = self.mem.as_slice();
    if let (Some(a), Some(b)) = (slice.get(first), slice.get(second)) {
      // Compare by (target, source) tuple for target tree
      (a.target, a.source) < (b.target, b.source)
    } else {
      first < second
    }
  }
}

// Implement SizeBalanced for all strategies (required by trait bounds)
impl<'a, M: RawMem<Item = RawLink>, S> SizeBalanced<usize>
  for TargetTree<'a, M, S>
{
}

// Implement AdaptiveRadix for all strategies (required by trait bounds)
impl<'a, M: RawMem<Item = RawLink>, S> AdaptiveRadix<usize>
  for TargetTree<'a, M, S>
{
}

/// Doublets store implementation using tree-based indexing
///
/// Generic over tree strategies for both source and target indexing.
///
/// # Type Parameters
/// * `T` - Index type (usually usize)
/// * `M` - Memory backend (default: heap allocation)
/// * `SourceStrategy` - Tree strategy for source indexing
///   (SbtStrategy or ArtStrategy)
/// * `TargetStrategy` - Tree strategy for target indexing
///   (SbtStrategy or ArtStrategy)
///
/// # Examples
/// ```
/// use doublets::{SbtStrategy, ArtStrategy, create_heap_store_with_strategies};
///
/// // Create a store with SBT for both source and target trees
/// let mut sbt_store =
///   create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()
///     .unwrap();
///
/// // Create a store with mixed strategies
/// let mut mixed_store =
///   create_heap_store_with_strategies::<usize, SbtStrategy, ArtStrategy>()
///     .unwrap();
/// ```
pub struct Store<
  T,
  M = Alloc<RawLink>,
  SourceStrategy = SbtStrategy,
  TargetStrategy = SbtStrategy,
> where
  T: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
  SourceStrategy: TreeStrategy<usize>,
  TargetStrategy: TreeStrategy<usize>,
{
  mem: M,
  allocated: usize,
  free_count: usize,
  first_free: Option<usize>,
  /// Root of tree indexing links by source
  source_root: Option<usize>,
  /// Root of tree indexing links by target
  target_root: Option<usize>,
  _phantom: core::marker::PhantomData<(T, SourceStrategy, TargetStrategy)>,
}

impl<T, M, SourceStrategy, TargetStrategy>
  Store<T, M, SourceStrategy, TargetStrategy>
where
  T: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
  SourceStrategy: TreeStrategy<usize>,
  TargetStrategy: TreeStrategy<usize>,
{
  /// Create a new doublets store with default capacity
  pub fn new(mut mem: M) -> Result<Self, T> {
    mem.grow(1024).map_err(|_| Error::AllocationFailed)?.zeroed();

    Ok(Self {
      mem,
      allocated: 1,
      free_count: 0,
      first_free: None,
      source_root: None,
      target_root: None,
      _phantom: core::marker::PhantomData,
    })
  }

  /// Get a raw link from memory
  #[inline]
  fn repr_at(&self, index: usize) -> Option<&RawLink> {
    let slice = self.mem.as_slice();
    slice.get(index)
  }

  /// Get a mutable raw link from memory
  #[inline]
  fn repr_mut_at(&mut self, index: usize) -> Option<&mut RawLink> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(index)
  }

  /// Check if a link exists and is not in free list
  fn exists(&self, index: T) -> bool {
    let idx = index.as_usize();
    if index.is_zero() || idx >= self.allocated {
      return false;
    }

    if let Some(raw) = self.repr_at(idx) {
      raw.is_free != usize::MAX
    } else {
      false
    }
  }

  /// Allocate a new link index
  fn allocate_index(&mut self) -> Result<T, T> {
    if let Some(free_index) = self.first_free {
      let next_free = if let Some(raw) = self.repr_at(free_index) {
        if raw.source == 0 { None } else { Some(raw.source) }
      } else {
        None
      };

      if let Some(raw) = self.repr_mut_at(free_index) {
        raw.is_free = 0;
      }

      self.first_free = next_free;
      self.free_count -= 1;
      return Ok(T::from_usize(free_index));
    }

    let index = self.allocated;
    self.allocated += 1;

    if self.allocated >= self.mem.as_slice().len() {
      let current_len = self.mem.as_slice().len();
      let addition = current_len;
      self.mem.grow(addition).map_err(|_| Error::AllocationFailed)?.zeroed();
    }

    if let Some(raw) = self.repr_mut_at(index) {
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

    if let Some(raw) = self.repr_mut_at(idx) {
      raw.source = next_free;
      raw.target = 0;
      raw.is_free = usize::MAX;
      // Clear tree nodes
      raw.source_tree = Node::default();
      raw.target_tree = Node::default();
    }

    self.first_free = Some(idx);
    self.free_count += 1;
  }

  /// Attach a link to the source tree
  fn attach_to_source_tree(&mut self, index: usize)
  where
    for<'a> SourceTree<'a, M, SourceStrategy>:
      SizeBalanced<usize> + AdaptiveRadix<usize>,
  {
    let mut tree = SourceTree::<M, SourceStrategy>::new(&mut self.mem);
    self.source_root =
      SourceStrategy::insert(&mut tree, self.source_root, index);
  }

  /// Detach a link from the source tree
  fn detach_from_source_tree(&mut self, index: usize)
  where
    for<'a> SourceTree<'a, M, SourceStrategy>:
      SizeBalanced<usize> + AdaptiveRadix<usize>,
  {
    let mut tree = SourceTree::<M, SourceStrategy>::new(&mut self.mem);
    self.source_root =
      SourceStrategy::remove(&mut tree, self.source_root, index);

    // Clear the node's tree pointers after removal
    if let Some(raw) = self.repr_mut_at(index) {
      raw.source_tree = Node::default();
    }
  }

  /// Attach a link to the target tree
  fn attach_to_target_tree(&mut self, index: usize)
  where
    for<'a> TargetTree<'a, M, TargetStrategy>:
      SizeBalanced<usize> + AdaptiveRadix<usize>,
  {
    let mut tree = TargetTree::<M, TargetStrategy>::new(&mut self.mem);
    self.target_root =
      TargetStrategy::insert(&mut tree, self.target_root, index);
  }

  /// Detach a link from the target tree
  fn detach_from_target_tree(&mut self, index: usize)
  where
    for<'a> TargetTree<'a, M, TargetStrategy>:
      SizeBalanced<usize> + AdaptiveRadix<usize>,
  {
    let mut tree = TargetTree::<M, TargetStrategy>::new(&mut self.mem);
    self.target_root =
      TargetStrategy::remove(&mut tree, self.target_root, index);

    // Clear the node's tree pointers after removal
    if let Some(raw) = self.repr_mut_at(index) {
      raw.target_tree = Node::default();
    }
  }

  /// Search for a link with exact source and target in source tree
  fn search_in_source_tree(
    &self,
    source: usize,
    target: usize,
  ) -> Option<usize> {
    let mut current = self.source_root?;
    let slice = self.mem.as_slice();

    loop {
      let raw = slice.get(current)?;

      match (source, target).cmp(&(raw.source, raw.target)) {
        core::cmp::Ordering::Equal => return Some(current),
        core::cmp::Ordering::Less => {
          current = raw.source_tree.left?;
        }
        core::cmp::Ordering::Greater => {
          current = raw.source_tree.right?;
        }
      }
    }
  }

  /// Traverse source tree calling handler for all links with matching source
  #[allow(dead_code)]
  fn each_by_source<H: ReadHandler<T>>(
    &self,
    source: usize,
    handler: &mut H,
  ) -> Flow {
    self.traverse_source_tree(self.source_root, source, usize::MAX, handler)
  }

  /// Traverse target tree calling handler for all links with matching target
  #[allow(dead_code)]
  fn each_by_target<H: ReadHandler<T>>(
    &self,
    target: usize,
    handler: &mut H,
  ) -> Flow {
    self.traverse_target_tree(self.target_root, target, usize::MAX, handler)
  }

  /// Recursively traverse source tree for links with matching source
  #[allow(dead_code)]
  fn traverse_source_tree<H: ReadHandler<T>>(
    &self,
    current: Option<usize>,
    source: usize,
    target: usize,
    handler: &mut H,
  ) -> Flow {
    let idx = match current {
      Some(i) => i,
      None => return Flow::Continue,
    };

    let slice = self.mem.as_slice();
    let raw = match slice.get(idx) {
      Some(r) => r,
      None => return Flow::Continue,
    };

    // When searching by source with wildcard target
    if target == usize::MAX {
      // Do an in-order traversal, visiting only nodes where source matches
      // Since tree is ordered by (source, target), matching nodes are
      // contiguous in in-order traversal

      // If current node's source < search source, go right
      if raw.source < source {
        return self.traverse_source_tree(
          raw.source_tree.right,
          source,
          target,
          handler,
        );
      }

      // If current node's source > search source, go left
      if raw.source > source {
        return self.traverse_source_tree(
          raw.source_tree.left,
          source,
          target,
          handler,
        );
      }

      // Current node's source == search source, traverse both subtrees
      if self.traverse_source_tree(
        raw.source_tree.left,
        source,
        target,
        handler,
      ) == Flow::Break
      {
        return Flow::Break;
      }

      // Check current node
      let link = Link::new(
        T::from_usize(idx),
        T::from_usize(raw.source),
        T::from_usize(raw.target),
      );
      if handler.handle(link) == Flow::Break {
        return Flow::Break;
      }

      // Continue to right subtree
      return self.traverse_source_tree(
        raw.source_tree.right,
        source,
        target,
        handler,
      );
    } else {
      // Exact (source, target) search - can prune efficiently
      // Traverse left subtree if it might contain matches
      if (source, target) < (raw.source, raw.target)
        && self.traverse_source_tree(
          raw.source_tree.left,
          source,
          target,
          handler,
        ) == Flow::Break
      {
        return Flow::Break;
      }

      // Check current node
      if raw.source == source && raw.target == target {
        let link = Link::new(
          T::from_usize(idx),
          T::from_usize(raw.source),
          T::from_usize(raw.target),
        );
        if handler.handle(link) == Flow::Break {
          return Flow::Break;
        }
      }

      // Traverse right subtree if it might contain matches
      if (source, target) > (raw.source, raw.target)
        && self.traverse_source_tree(
          raw.source_tree.right,
          source,
          target,
          handler,
        ) == Flow::Break
      {
        return Flow::Break;
      }
    }

    Flow::Continue
  }

  /// Recursively traverse target tree for links with matching target
  #[allow(dead_code)]
  fn traverse_target_tree<H: ReadHandler<T>>(
    &self,
    current: Option<usize>,
    target: usize,
    source: usize,
    handler: &mut H,
  ) -> Flow {
    let idx = match current {
      Some(i) => i,
      None => return Flow::Continue,
    };

    let slice = self.mem.as_slice();
    let raw = match slice.get(idx) {
      Some(r) => r,
      None => return Flow::Continue,
    };

    // When searching by target with wildcard source
    if source == usize::MAX {
      // Do an in-order traversal, visiting only nodes where target matches
      // Since tree is ordered by (target, source), matching nodes are
      // contiguous in in-order traversal

      // If current node's target < search target, go right
      if raw.target < target {
        return self.traverse_target_tree(
          raw.target_tree.right,
          target,
          source,
          handler,
        );
      }

      // If current node's target > search target, go left
      if raw.target > target {
        return self.traverse_target_tree(
          raw.target_tree.left,
          target,
          source,
          handler,
        );
      }

      // Current node's target == search target, traverse both subtrees
      if self.traverse_target_tree(
        raw.target_tree.left,
        target,
        source,
        handler,
      ) == Flow::Break
      {
        return Flow::Break;
      }

      // Check current node
      let link = Link::new(
        T::from_usize(idx),
        T::from_usize(raw.source),
        T::from_usize(raw.target),
      );
      if handler.handle(link) == Flow::Break {
        return Flow::Break;
      }

      // Continue to right subtree
      return self.traverse_target_tree(
        raw.target_tree.right,
        target,
        source,
        handler,
      );
    } else {
      // Exact (target, source) search - can prune efficiently
      // Traverse left subtree if it might contain matches
      if (target, source) < (raw.target, raw.source)
        && self.traverse_target_tree(
          raw.target_tree.left,
          target,
          source,
          handler,
        ) == Flow::Break
      {
        return Flow::Break;
      }

      // Check current node
      if raw.target == target && raw.source == source {
        let link = Link::new(
          T::from_usize(idx),
          T::from_usize(raw.source),
          T::from_usize(raw.target),
        );
        if handler.handle(link) == Flow::Break {
          return Flow::Break;
        }
      }

      // Traverse right subtree if it might contain matches
      if (target, source) > (raw.target, raw.source)
        && self.traverse_target_tree(
          raw.target_tree.right,
          target,
          source,
          handler,
        ) == Flow::Break
      {
        return Flow::Break;
      }
    }

    Flow::Continue
  }

  /// Count all non-free links
  fn count_total(&self) -> usize {
    self.allocated - self.free_count - 1
  }
}

impl<T, M, SourceStrategy, TargetStrategy> Links<T>
  for Store<T, M, SourceStrategy, TargetStrategy>
where
  T: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
  SourceStrategy: TreeStrategy<usize>,
  TargetStrategy: TreeStrategy<usize>,
{
  fn count<const N: usize>(&self, query: [T; N]) -> T {
    match N {
      0 => T::from_usize(self.count_total()),
      1 => {
        let index = query[0];
        if index == T::ANY {
          T::from_usize(self.count_total())
        } else if self.exists(index) {
          T::ONE
        } else {
          T::ZERO
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

  fn create<const N: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N],
    handler: &mut H,
  ) -> Result<Flow, T> {
    let index = self.allocate_index()?;
    let before = Link::nothing();

    let (source, target) = match N {
      0 => (T::ZERO, T::ZERO),
      1 => (query[0], query[0]),
      _ => (query[0], query[1]),
    };

    let idx = index.as_usize();

    if let Some(raw) = self.repr_mut_at(idx) {
      raw.source = source.as_usize();
      raw.target = target.as_usize();
      raw.is_free = 0;
      raw.source_tree = Node::default();
      raw.target_tree = Node::default();
    }

    // Attach to both trees for efficient searching
    self.attach_to_source_tree(idx);
    self.attach_to_target_tree(idx);

    let after = Link::new(index, source, target);
    Ok(handler.handle(before, after))
  }

  fn each<const N: usize, H: ReadHandler<T>>(
    &self,
    query: [T; N],
    handler: &mut H,
  ) -> Flow {
    if N == 0 {
      // Enumerate all links
      for i in 1..self.allocated {
        let index = T::from_usize(i);
        if self.exists(index)
          && let Some(raw) = self.repr_at(i)
        {
          let source = T::from_usize(raw.source);
          let target = T::from_usize(raw.target);
          let link = Link::new(index, source, target);
          if handler.handle(link) == Flow::Break {
            return Flow::Break;
          }
        }
      }
      return Flow::Continue;
    }

    let index_query = query[0];

    if N == 1 {
      if index_query == T::ANY {
        return self.each([], handler);
      } else if self.exists(index_query)
        && let Some(raw) = self.repr_at(index_query.as_usize())
      {
        let source = T::from_usize(raw.source);
        let target = T::from_usize(raw.target);
        return handler.handle(Link::new(index_query, source, target));
      }
      return Flow::Continue;
    }

    let source = if N >= 2 { query[1] } else { T::ANY };
    let target = if N >= 3 { query[2] } else { T::ANY };

    // Use tree-based search when possible for better performance
    if index_query == T::ANY {
      // Query by source and/or target
      if source != T::ANY && target != T::ANY {
        // Exact (source, target) search - use tree
        if let Some(idx) =
          self.search_in_source_tree(source.as_usize(), target.as_usize())
          && self.exists(T::from_usize(idx))
        {
          let raw = self.repr_at(idx).unwrap();
          let link = Link::new(
            T::from_usize(idx),
            T::from_usize(raw.source),
            T::from_usize(raw.target),
          );
          return handler.handle(link);
        }
        return Flow::Continue;
      } else if source != T::ANY || target != T::ANY {
        // Wildcard queries - use linear scan due to SBT corruption bugs
        // TODO: Fix SBT remove bugs or switch to ART to enable tree traversal
        for i in 1..self.allocated {
          let index = T::from_usize(i);
          if self.exists(index)
            && let Some(raw) = self.repr_at(i)
          {
            let raw_source = T::from_usize(raw.source);
            let raw_target = T::from_usize(raw.target);

            let matches = (source == T::ANY || source == raw_source)
              && (target == T::ANY || target == raw_target);

            if matches {
              let link = Link::new(index, raw_source, raw_target);
              if handler.handle(link) == Flow::Break {
                return Flow::Break;
              }
            }
          }
        }
        return Flow::Continue;
      } else {
        // No constraints - enumerate all
        return self.each([], handler);
      }
    }

    // Query with specific index - direct lookup
    if !self.exists(index_query) {
      return Flow::Continue;
    }

    let raw = match self.repr_at(index_query.as_usize()) {
      Some(r) => r,
      None => return Flow::Continue,
    };

    let raw_source = T::from_usize(raw.source);
    let raw_target = T::from_usize(raw.target);

    let matches = (source == T::ANY || source == raw_source)
      && (target == T::ANY || target == raw_target);

    if matches {
      let link = Link::new(index_query, raw_source, raw_target);
      return handler.handle(link);
    }

    Flow::Continue
  }

  fn update<const N1: usize, const N2: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N1],
    change: [T; N2],
    handler: &mut H,
  ) -> Result<Flow, T> {
    if N1 == 0 || N2 == 0 {
      return Err(Error::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(Error::NotExists(index));
    }

    let before = self.get(index).ok_or(Error::NotExists(index))?;

    let new_source = if N2 >= NC_SOURCE { change[1] } else { before.source };
    let new_target = if N2 >= NC_TARGET { change[2] } else { before.target };

    let idx = index.as_usize();

    // If source or target changed, update tree positions
    if new_source != before.source || new_target != before.target {
      // Detach from old positions in both trees
      self.detach_from_source_tree(idx);
      self.detach_from_target_tree(idx);

      // Update the link data
      if let Some(raw) = self.repr_mut_at(idx) {
        raw.source = new_source.as_usize();
        raw.target = new_target.as_usize();
      }

      // Reattach to new positions in both trees
      self.attach_to_source_tree(idx);
      self.attach_to_target_tree(idx);
    }

    let after = Link::new(index, new_source, new_target);
    Ok(handler.handle(before, after))
  }

  fn delete<const N: usize, H: WriteHandler<T>>(
    &mut self,
    query: [T; N],
    handler: &mut H,
  ) -> Result<Flow, T> {
    if N == 0 {
      return Err(Error::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(Error::NotExists(index));
    }

    let before = self.get(index).ok_or(Error::NotExists(index))?;

    // Detach from both trees before freeing
    let idx = index.as_usize();
    self.detach_from_source_tree(idx);
    self.detach_from_target_tree(idx);

    self.free_index(index);

    let after = Link::nothing();
    Ok(handler.handle(before, after))
  }

  fn get(&self, index: T) -> Option<Link<T>> {
    if !self.exists(index) {
      return None;
    }

    let raw = self.repr_at(index.as_usize())?;
    let source = T::from_usize(raw.source);
    let target = T::from_usize(raw.target);
    Some(Link::new(index, source, target))
  }
}

/// Create a doublets store with heap allocation using SBT
/// (Size-Balanced Tree) for both source and target trees
pub fn create_heap_store<T>()
-> Result<Store<T, Alloc<RawLink>, SbtStrategy, SbtStrategy>, T>
where
  T: Index,
{
  Store::new(Alloc::new())
}

/// Create a doublets store with heap allocation and custom tree strategies
pub fn create_heap_store_with_strategies<T, SourceStrategy, TargetStrategy>()
-> Result<Store<T, Alloc<RawLink>, SourceStrategy, TargetStrategy>, T>
where
  T: Index,
  SourceStrategy: TreeStrategy<usize>,
  TargetStrategy: TreeStrategy<usize>,
{
  Store::new(Alloc::new())
}
