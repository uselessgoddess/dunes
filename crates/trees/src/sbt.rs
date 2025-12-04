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
  /// TODO: This implementation has known bugs in certain cases.
  /// The removal logic needs to be reviewed against reference impl.
  fn remove_sbt(&mut self, root: Option<T>, idx: T) -> Option<T> {
    let mut root_val = root?;
    if unsafe { self.remove_impl(&mut root_val, idx)? } {
      None
    } else {
      Some(root_val)
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
  /// # Safety
  ///
  /// The `root` pointer must be valid and point to a value from
  /// `left_mut` or `right_mut`. No other tree node refs allowed.
  unsafe fn remove_impl(&mut self, mut root: *mut T, idx: T) -> Option<bool> {
    // Save original root to detect if we're removing it
    let original_root = *root;
    loop {
      let left = self.left_mut(*root).map(|r| r as *mut T);
      let right = self.right_mut(*root).map(|r| r as *mut T);

      if self.is_left_of(idx, *root) {
        let rl_size =
          self.right(*root).and_then(|r| self.left_size(r)).unwrap_or(0);
        let rr_size =
          self.right(*root).and_then(|r| self.right_size(r)).unwrap_or(0);
        let left_size = self.left_size(*root).unwrap_or(0);

        if rr_size >= left_size {
          *root = self.rotate_left(*root)?;
        } else if rl_size >= left_size {
          let right_ptr = right?;
          *right_ptr = self.rotate_right(*right_ptr)?;
          *root = self.rotate_left(*root)?;
        } else {
          self.dec_size(*root);
          root = left?;
        }
      } else if self.is_right_of(idx, *root) {
        let ll_size =
          self.left(*root).and_then(|l| self.left_size(l)).unwrap_or(0);
        let lr_size =
          self.left(*root).and_then(|l| self.right_size(l)).unwrap_or(0);
        let right_size = self.right_size(*root).unwrap_or(0);

        if ll_size >= right_size {
          *root = self.rotate_right(*root)?;
        } else if lr_size >= right_size {
          let left_ptr = left?;
          *left_ptr = self.rotate_left(*left_ptr)?;
          *root = self.rotate_right(*root)?;
        } else {
          self.dec_size(*root);
          root = right?;
        }
      } else {
        match (left, right) {
          (Some(left_ptr), Some(right_ptr)) => {
            let left_size = self.left_size(*root)?;
            let right_size = self.right_size(*root)?;

            let replacement;
            if left_size > right_size {
              replacement = self.rightest(*left_ptr);
              let _ = self.remove_impl(left_ptr, replacement);
            } else {
              replacement = self.leftest(*right_ptr);
              let _ = self.remove_impl(right_ptr, replacement);
            }
            self.set_left(replacement, self.left(*root));
            self.set_right(replacement, self.right(*root));
            self.set_size(replacement, left_size + right_size);
            *root = replacement;
          }
          (Some(left_ptr), _) => *root = *left_ptr,
          (_, Some(right_ptr)) => *root = *right_ptr,
          _ => {
            self.clear(idx);
            // Only return true (tree empty) if we're removing the original root
            return Some(*root == original_root);
          }
        };
        self.clear(idx);
        return Some(false);
      }
    }
  }
}
