use trees::{AdaptiveRadix, Idx, Node, SizeBalanced, Tree};

/// Vector-backed tree store for testing and benchmarking.
/// Generic over the tree implementation strategy.
#[derive(Debug, Clone)]
pub struct Store<T> {
  nodes: Vec<Node<T>>,
}

impl<T> Store<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  #[inline]
  #[allow(dead_code)]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }
}

// Base Tree trait implementation
impl<T: Idx> Tree<T> for Store<T> {
  #[inline(always)]
  fn get(&self, idx: T) -> Option<Node<T>> {
    self.nodes.get(idx.as_usize()).copied()
  }

  #[inline(always)]
  fn set(&mut self, idx: T, node: Node<T>) {
    if let Some(slot) = self.nodes.get_mut(idx.as_usize()) {
      *slot = node;
    }
  }

  #[inline(always)]
  fn left_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize())?.left.as_mut()
  }

  #[inline(always)]
  fn right_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize())?.right.as_mut()
  }

  #[inline(always)]
  fn is_left_of(&self, first: T, second: T) -> bool {
    first.as_usize() < second.as_usize()
  }

  // Default implementation - will be overridden by strategy traits
  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.insert_sbt(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_sbt(root, idx)
  }
}

// SBT strategy
impl<T: Idx> SizeBalanced<T> for Store<T> {}

// ART strategy
impl<T: Idx> AdaptiveRadix<T> for Store<T> {}

/// ART-specific store that uses ART insert/remove by default
#[derive(Debug, Clone)]
pub struct ArtStore<T> {
  inner: Store<T>,
}

impl<T> ArtStore<T> {
  pub fn new(capacity: usize) -> Self {
    Self { inner: Store::new(capacity) }
  }

  pub fn reset(&mut self) {
    self.inner.reset()
  }
}

impl<T: Idx> Tree<T> for ArtStore<T> {
  #[inline(always)]
  fn get(&self, idx: T) -> Option<Node<T>> {
    self.inner.get(idx)
  }

  #[inline(always)]
  fn set(&mut self, idx: T, node: Node<T>) {
    self.inner.set(idx, node)
  }

  #[inline(always)]
  fn left_mut(&mut self, idx: T) -> Option<&mut T> {
    self.inner.left_mut(idx)
  }

  #[inline(always)]
  fn right_mut(&mut self, idx: T) -> Option<&mut T> {
    self.inner.right_mut(idx)
  }

  #[inline(always)]
  fn is_left_of(&self, first: T, second: T) -> bool {
    self.inner.is_left_of(first, second)
  }

  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.insert_art(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_art(root, idx)
  }
}

impl<T: Idx> AdaptiveRadix<T> for ArtStore<T> {}
