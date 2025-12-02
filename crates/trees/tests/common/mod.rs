use trees::{Idx, Node, SizeBalanced, Tree};

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

  // Default implementation - will be overridden by strategy traits
  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.insert_sbt(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_sbt(root, idx)
  }
}

// SBT strategy
impl<T: Idx> SizeBalanced<T> for VecStore<T> {}

// Type alias for convenience
pub type Store<T> = VecStore<T>;
