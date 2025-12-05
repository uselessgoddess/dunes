use trees::{AdaptiveRadix, Idx, Node, SizeBalanced, Tree};

/// Vector-backed tree store for testing and benchmarking.
/// Generic over the tree implementation strategy.
#[derive(Debug, Clone)]
pub struct VecStore<T> {
  nodes: Vec<Node<T>>,
}

impl<T> VecStore<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  #[allow(dead_code)]
  pub fn with_nodes(nodes: Vec<Node<T>>) -> Self {
    Self { nodes }
  }

  #[inline]
  #[allow(dead_code)]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  #[inline]
  #[allow(dead_code)]
  pub fn nodes_mut(&mut self) -> &mut [Node<T>] {
    &mut self.nodes
  }

  #[allow(dead_code)]
  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }

  #[allow(dead_code)]
  pub fn is_empty(&self) -> bool {
    self.nodes.iter().all(|n| n.size == 0)
  }
}

// Base Tree trait implementation for SBT
impl<T: Idx> Tree<T> for VecStore<T> {
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

  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    SizeBalanced::insert_sbt(self, root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    SizeBalanced::remove_sbt(self, root, idx)
  }
}

// SBT strategy
impl<T: Idx> SizeBalanced<T> for VecStore<T> {}

// ART strategy
impl<T: Idx> AdaptiveRadix<T> for VecStore<T> {}

// Type alias for convenience
pub type Store<T> = VecStore<T>;

/// ART-specific store that uses ART insert/remove by default
#[derive(Debug, Clone)]
pub struct ArtStore<T> {
  inner: VecStore<T>,
}

impl<T> ArtStore<T> {
  pub fn new(capacity: usize) -> Self {
    Self { inner: VecStore::new(capacity) }
  }

  #[allow(dead_code)]
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

  // Override to use ART instead of default SBT
  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    AdaptiveRadix::insert_art(self, root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    AdaptiveRadix::remove_art(self, root, idx)
  }
}

impl<T: Idx> AdaptiveRadix<T> for ArtStore<T> {}
