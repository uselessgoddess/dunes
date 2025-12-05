use crate::{Idx, Node};

/// Base trait for all tree implementations
///
/// Provides common operations that all tree types should support:
/// - Node access (get/set)
/// - Tree navigation (left/right children)
/// - Tree operations (insert, search, remove)
/// - Tree traversal helpers
pub trait Tree<T: Idx> {
  /// Get node at index
  fn get(&self, idx: T) -> Option<Node<T>>;

  /// Set node at index
  fn set(&mut self, idx: T, node: Node<T>);

  /// Get mutable reference to left child (if it exists)
  fn left_mut(&mut self, idx: T) -> Option<&mut T>;

  /// Get mutable reference to right child (if it exists)
  fn right_mut(&mut self, idx: T) -> Option<&mut T>;

  /// Compare two indices - true if first should be left of second
  fn is_left_of(&self, first: T, second: T) -> bool;

  /// Compare two indices - true if first should be right of second
  #[inline]
  fn is_right_of(&self, first: T, second: T) -> bool {
    first != second && !self.is_left_of(first, second)
  }

  /// Get left child index
  #[inline]
  fn left(&self, idx: T) -> Option<T> {
    self.get(idx)?.left
  }

  /// Get right child index
  #[inline]
  fn right(&self, idx: T) -> Option<T> {
    self.get(idx)?.right
  }

  /// Set left child
  #[inline]
  fn set_left(&mut self, idx: T, left: Option<T>) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { left, ..node });
    }
  }

  /// Set right child
  #[inline]
  fn set_right(&mut self, idx: T, right: Option<T>) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { right, ..node });
    }
  }

  /// Find the rightmost (maximum) node in subtree
  fn rightest(&self, mut current: T) -> T {
    while let Some(next) = self.right(current) {
      current = next;
    }
    current
  }

  /// Find the leftmost (minimum) node in subtree
  fn leftest(&self, mut current: T) -> T {
    while let Some(next) = self.left(current) {
      current = next;
    }
    current
  }

  /// Get in-order successor (next node in sorted order)
  #[inline]
  fn next(&self, idx: T) -> Option<T> {
    self.right(idx).map(|idx| self.leftest(idx))
  }

  /// Get in-order predecessor (previous node in sorted order)
  #[inline]
  fn prev(&self, idx: T) -> Option<T> {
    self.left(idx).map(|idx| self.rightest(idx))
  }

  /// Search for a node in the tree
  fn contains(&self, mut root: T, idx: T) -> bool {
    loop {
      if self.is_left_of(idx, root) {
        match self.left(root) {
          Some(next) => root = next,
          None => return false,
        }
      } else if self.is_right_of(idx, root) {
        match self.right(root) {
          Some(next) => root = next,
          None => return false,
        }
      } else {
        return true;
      }
    }
  }

  /// Clear node (set to default)
  #[inline]
  fn clear(&mut self, idx: T) {
    self.set(idx, Node::default())
  }

  /// Insert index into tree, returns new root
  ///
  /// Implementations should use their specific tree strategy
  /// (e.g., SizeBalanced::insert_sbt or AdaptiveRadix::insert_art).
  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T>;

  /// Remove index from tree, returns new root (None if tree empty)
  ///
  /// Implementations should use their specific tree strategy
  /// (e.g., SizeBalanced::remove_sbt or AdaptiveRadix::remove_art).
  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T>;
}
