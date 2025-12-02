use dunes_trees::{Idx, Node, SizeBalanced, Treap, Tree};

/// Simple vector-backed tree store for testing and benchmarking
/// (SBT implementation)
#[derive(Debug, Clone)]
pub struct Store<T> {
  nodes: Vec<Node<T>>,
}

impl<T> Store<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  pub fn with_nodes(nodes: Vec<Node<T>>) -> Self {
    Self { nodes }
  }

  #[inline]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  #[inline]
  pub fn nodes_mut(&mut self) -> &mut [Node<T>] {
    &mut self.nodes
  }

  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.iter().all(|n| n.size == 0)
  }
}

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

  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.insert_sbt(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_sbt(root, idx)
  }
}

impl<T: Idx> SizeBalanced<T> for Store<T> {}

/// Vector-backed Treap store for testing and benchmarking
#[derive(Debug, Clone)]
pub struct TreapStore<T> {
  nodes: Vec<Node<T>>,
}

impl<T> TreapStore<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  #[inline]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }
}

impl<T: Idx> Tree<T> for TreapStore<T> {
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
    self.insert_treap(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_treap(root, idx)
  }
}

impl<T: Idx> Treap<T> for TreapStore<T> {}
