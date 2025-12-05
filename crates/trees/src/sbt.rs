use crate::{Idx, Node, Tree};

/// Size-Balanced Tree - provides operations on tree stored in slice
///
/// Extends the base Tree trait with size-balancing operations.
/// Uses subtree size to maintain balance (Chinese student's tree algorithm).
pub trait SizeBalanced<T: Idx>: Tree<T> {
  /// Get size of subtree rooted at index
  #[inline]
  fn size(&self, idx: T) -> Option<usize> {
    self.get(idx).map(|n| n.size)
  }

  /// Set size of subtree
  #[inline]
  fn set_size(&mut self, idx: T, size: usize) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { size, ..node });
    }
  }

  /// Get size of left subtree
  #[inline]
  fn left_size(&self, idx: T) -> Option<usize> {
    self.left(idx).and_then(|idx| self.size(idx))
  }

  /// Get size of right subtree
  #[inline]
  fn right_size(&self, idx: T) -> Option<usize> {
    self.right(idx).and_then(|idx| self.size(idx))
  }

  /// Increment subtree size
  #[inline]
  fn inc_size(&mut self, idx: T) {
    if let Some(size) = self.size(idx) {
      self.set_size(idx, size + 1)
    }
  }

  /// Decrement subtree size
  #[inline]
  fn dec_size(&mut self, idx: T) {
    if let Some(size) = self.size(idx) {
      self.set_size(idx, size - 1)
    }
  }

  /// Recalculate and fix subtree size based on children
  #[inline]
  fn fix_size(&mut self, idx: T) {
    let size =
      self.left_size(idx).unwrap_or(0) + self.right_size(idx).unwrap_or(0) + 1;
    self.set_size(idx, size)
  }

  #[must_use]
  fn rotate_left(&mut self, root: T) -> Option<T> {
    let right = self.right(root)?;
    self.set_right(root, self.left(right));
    self.set_left(right, Some(root));
    self.set_size(right, self.size(root)?);
    self.fix_size(root);
    Some(right)
  }

  #[must_use]
  fn rotate_right(&mut self, root: T) -> Option<T> {
    let left = self.left(root)?;
    self.set_left(root, self.right(left));
    self.set_right(left, Some(root));
    self.set_size(left, self.size(root)?);
    self.fix_size(root);
    Some(left)
  }

  /// Insert index into tree using SBT balancing, returns new root
  fn insert_sbt(&mut self, root: Option<T>, idx: T) -> Option<T> {
    if let Some(mut root_val) = root {
      unsafe { self.insert_impl(&mut root_val, idx)? }
      Some(root_val)
    } else {
      self.set_size(idx, 1);
      Some(idx)
    }
  }

  /// Remove index from tree using SBT balancing
  ///
  /// Returns new root (None if tree empty)
  ///
  /// Note: If the index doesn't exist in the tree, returns the original root unchanged
  fn remove_sbt(&mut self, root: Option<T>, idx: T) -> Option<T> {
    let mut root_val = root?;
    match unsafe { self.remove_impl(&mut root_val, idx) } {
      Some(true) => None, // Tree became empty
      Some(false) => Some(root_val), // Removed successfully, tree not empty
      None => root, // Value not found, return original root
    }
  }

  /// Internal insert implementation using pointer for in-place updates
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid and point to a value from
  /// `left_mut` or `right_mut`. No other tree node refs allowed.
  unsafe fn insert_impl(&mut self, mut root: *mut T, idx: T) -> Option<()> {
    loop {
      if self.is_left_of(idx, *root) {
        let Some(left_ref) = self.left_mut(*root) else {
          self.inc_size(*root);
          self.set_size(idx, 1);
          self.set_left(*root, Some(idx));
          return Some(());
        };
        let left = left_ref as *mut T;

        let left_size = self.size(*left)?;
        let right_size = self.right_size(*root).unwrap_or(0);

        if self.is_left_of(idx, *left) {
          if left_size >= right_size {
            *root = self.rotate_right(*root)?;
          } else {
            self.inc_size(*root);
            root = left;
          }
        } else {
          let lr_size = self.right_size(*left).unwrap_or(0);
          if lr_size >= right_size {
            if lr_size == 0 && right_size == 0 {
              self.set_left(idx, Some(*left));
              self.set_right(idx, Some(*root));
              self.set_size(idx, left_size + 2);
              self.set_left(*root, None);
              self.set_size(*root, 1);
              *root = idx;
              return Some(());
            }
            *left = self.rotate_left(*left)?;
            *root = self.rotate_right(*root)?;
          } else {
            self.inc_size(*root);
            root = left;
          }
        }
      } else {
        let Some(right_ref) = self.right_mut(*root) else {
          self.inc_size(*root);
          self.set_size(idx, 1);
          self.set_right(*root, Some(idx));
          return Some(());
        };
        let right = right_ref as *mut T;

        let right_size = self.size(*right)?;
        let left_size = self.left_size(*root).unwrap_or(0);

        if self.is_right_of(idx, *right) {
          if right_size >= left_size {
            *root = self.rotate_left(*root)?;
          } else {
            self.inc_size(*root);
            root = right;
          }
        } else {
          let rl_size = self.left_size(*right).unwrap_or(0);
          if rl_size >= left_size {
            if rl_size == 0 && left_size == 0 {
              self.set_left(idx, Some(*root));
              self.set_right(idx, Some(*right));
              self.set_size(idx, right_size + 2);
              self.set_right(*root, None);
              self.set_size(*root, 1);
              *root = idx;
              return Some(());
            }
            *right = self.rotate_right(*right)?;
            *root = self.rotate_left(*root)?;
          } else {
            self.inc_size(*root);
            root = right;
          }
        }
      }
    }
  }

  /// Internal remove implementation - returns true if tree becomes empty
  ///
  /// Based on C# reference from linksplatform/Collections.Methods
  /// Decrements sizes while traversing to find node (not after removal)
  ///
  /// Returns:
  /// - Some(true): Tree became empty after removal
  /// - Some(false): Removal successful, tree not empty
  /// - None: Value not found in tree (tree state may be corrupted - caller must handle)
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid and point to a value from
  /// `left_mut` or `right_mut`. No other tree node refs allowed.
  ///
  /// # Important
  ///
  /// Callers should ensure the value exists in the tree before calling this function.
  /// If the value doesn't exist, the tree's size metadata may be corrupted.
  unsafe fn remove_impl(&mut self, root: *mut T, idx: T) -> Option<bool> {
    // Traverse to find the node, decrementing sizes along the path
    let mut current = root;
    let mut parent = root;

    while *current != idx {
      self.dec_size(*current);

      if self.is_left_of(idx, *current) {
        parent = current;
        current = self.left_mut(*current).map(|r| r as *mut T)?;
      } else if self.is_right_of(idx, *current) {
        parent = current;
        current = self.right_mut(*current).map(|r| r as *mut T)?;
      } else {
        // This should not happen - means duplicate found
        return None;
      }
    }

    // Now current points to the node to detach
    let node_to_detach = *current;
    let left = self.left(node_to_detach);
    let right = self.right(node_to_detach);

    let replacement = match (left, right) {
      (Some(left_child), Some(right_child)) => {
        // Two children: find leftmost node in right subtree
        let leftmost = self.leftest(right_child);

        // Recursively detach the leftmost node from right subtree
        let right_ptr = self.right_mut(node_to_detach).map(|r| r as *mut T)?;
        self.detach_node(right_ptr, leftmost)?;

        // Set up the leftmost node as replacement
        self.set_left(leftmost, Some(left_child));
        let new_right = self.right(node_to_detach);

        if let Some(new_right_val) = new_right {
          self.set_right(leftmost, Some(new_right_val));
          let left_size = self.size(left_child)?;
          let right_size = self.size(new_right_val)?;
          self.set_size(leftmost, left_size + right_size + 1);
        } else {
          let left_size = self.size(left_child)?;
          self.set_size(leftmost, left_size + 1);
        }

        Some(leftmost)
      }
      (Some(left_child), None) => Some(left_child),
      (None, Some(right_child)) => Some(right_child),
      (None, None) => None,
    };

    // Update parent's pointer
    if *root == node_to_detach {
      // Detaching root
      if let Some(repl) = replacement {
        *root = repl;
      } else {
        // Tree becomes empty
        self.clear(node_to_detach);
        return Some(true);
      }
    } else if self.left(*parent) == Some(node_to_detach) {
      self.set_left(*parent, replacement);
    } else if self.right(*parent) == Some(node_to_detach) {
      self.set_right(*parent, replacement);
    }

    // Clear the detached node
    self.clear(node_to_detach);
    Some(false)
  }

  /// Helper to detach a specific node from a subtree
  /// Similar to remove_impl but doesn't handle the root == node case
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid
  unsafe fn detach_node(&mut self, root: *mut T, idx: T) -> Option<()> {
    let mut current = root;
    let mut parent = root;

    // Find the node, decrementing sizes
    while *current != idx {
      self.dec_size(*current);

      if self.is_left_of(idx, *current) {
        parent = current;
        current = self.left_mut(*current).map(|r| r as *mut T)?;
      } else if self.is_right_of(idx, *current) {
        parent = current;
        current = self.right_mut(*current).map(|r| r as *mut T)?;
      } else {
        return None;
      }
    }

    // Determine replacement
    let left = self.left(*current);
    let right = self.right(*current);

    let replacement = match (left, right) {
      (Some(left_child), Some(right_child)) => {
        let leftmost = self.leftest(right_child);
        let right_ptr = self.right_mut(*current).map(|r| r as *mut T)?;
        self.detach_node(right_ptr, leftmost)?;

        self.set_left(leftmost, Some(left_child));
        let new_right = self.right(*current);

        if let Some(new_right_val) = new_right {
          self.set_right(leftmost, Some(new_right_val));
          let left_size = self.size(left_child)?;
          let right_size = self.size(new_right_val)?;
          self.set_size(leftmost, left_size + right_size + 1);
        } else {
          let left_size = self.size(left_child)?;
          self.set_size(leftmost, left_size + 1);
        }

        Some(leftmost)
      }
      (Some(left_child), None) => Some(left_child),
      (None, Some(right_child)) => Some(right_child),
      (None, None) => None,
    };

    // Update parent
    if self.left(*parent) == Some(*current) {
      self.set_left(*parent, replacement);
    } else if self.right(*parent) == Some(*current) {
      self.set_right(*parent, replacement);
    }

    Some(())
  }
}
