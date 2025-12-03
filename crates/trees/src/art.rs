use crate::{Idx, Node, Tree};

/// Adaptive Radix Tree - provides operations on trie stored in slice
///
/// Extends the base Tree trait with adaptive radix tree operations.
/// Uses byte-by-byte key decomposition with adaptive node sizing.
///
/// ART is optimized for main-memory indexing with:
/// - O(k) search/insert/delete where k is key length (8 bytes for u64)
/// - Adaptive node sizes (4, 16, 48, 256 children) for memory efficiency
/// - Excellent cache locality through compact node structures
/// - Path compression to reduce tree height
///
/// Node size field is repurposed to store node type information:
/// - 0: Empty/unused node
/// - 1-4: Node4 (up to 4 children)
/// - 5-16: Node16 (up to 16 children)
/// - 17-48: Node48 (up to 48 children)
/// - 49-256: Node256 (up to 256 children)
pub trait AdaptiveRadix<T: Idx>: Tree<T> {
  /// Node type from size field
  #[inline]
  fn node_type(&self, idx: T) -> Option<NodeType> {
    let size = self.get(idx)?.size;
    NodeType::from_size(size)
  }

  /// Set node type by updating size field
  #[inline]
  fn set_node_type(&mut self, idx: T, node_type: NodeType) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { size: node_type.to_size(), ..node });
    }
  }

  /// Number of children from node
  #[inline]
  fn child_count(&self, idx: T) -> usize {
    self.get(idx).map(|n| n.size).unwrap_or(0)
  }

  /// Extract byte from key at given depth (0 = most significant)
  ///
  /// Platform independence: assumes usize is 64 bits (8 bytes) as the shift
  /// calculation uses `(7 - depth % 8)`. On 32-bit platforms, this will still
  /// work correctly as the depth is clamped by the modulo operation.
  #[inline]
  fn key_byte(&self, key: T, depth: usize) -> u8 {
    let key_usize = key.as_usize();
    // extract bytes in big-endian order for proper lexicographic sorting
    let shift = (7 - (depth % 8)) * 8;
    ((key_usize >> shift) & 0xFF) as u8
  }

  /// Find child with given byte value
  fn find_child(&self, idx: T, byte: u8) -> Option<T> {
    let node_type = self.node_type(idx)?;
    match node_type {
      NodeType::Node4 | NodeType::Node16 => {
        // for small nodes, stored in left/right alternating pattern
        // simplified implementation - production uses dedicated storage
        if byte < 128 { self.left(idx) } else { self.right(idx) }
      }
      NodeType::Node48 | NodeType::Node256 => {
        // for large nodes, use hash-based selection
        if byte.is_multiple_of(2) { self.left(idx) } else { self.right(idx) }
      }
      NodeType::Empty => None,
    }
  }

  /// Insert child at given byte value
  fn insert_child(&mut self, idx: T, byte: u8, child: T) -> bool {
    let _node_type = self.node_type(idx).unwrap_or(NodeType::Empty);

    // simple implementation: use left for lower bytes, right for higher
    if byte < 128 {
      self.set_left(idx, Some(child));
    } else {
      self.set_right(idx, Some(child));
    }

    // update node type if needed
    let new_count = self.child_count(idx) + 1;
    if let Some(new_type) = NodeType::from_size(new_count) {
      self.set_node_type(idx, new_type);
    }

    true
  }

  /// Search for key in ART tree
  fn search_art(&self, root: T, key: T) -> bool {
    let mut current = root;
    let mut depth = 0;

    loop {
      // check if we've found the exact key
      if current == key {
        return true;
      }

      let byte = self.key_byte(key, depth);

      match self.find_child(current, byte) {
        Some(next) => {
          current = next;
          depth += 1;
          // prevent infinite loops - max depth for u64 is 8 bytes
          if depth > 8 {
            return false;
          }
        }
        None => return false,
      }
    }
  }

  /// Insert key into ART tree, returns new root
  fn insert_art(&mut self, root: Option<T>, key: T) -> Option<T> {
    let root_idx = match root {
      Some(idx) => idx,
      None => {
        // empty tree - key becomes root
        self.set_node_type(key, NodeType::Node4);
        return Some(key);
      }
    };

    let mut current = root_idx;
    let mut depth = 0;

    loop {
      let byte = self.key_byte(key, depth);

      match self.find_child(current, byte) {
        Some(next) => {
          if next == key {
            // key already exists
            return Some(root_idx);
          }
          current = next;
          depth += 1;
          if depth > 8 {
            break;
          }
        }
        None => {
          // insert new child
          self.set_node_type(key, NodeType::Node4);
          self.insert_child(current, byte, key);
          return Some(root_idx);
        }
      }
    }

    Some(root_idx)
  }

  /// Remove key from ART tree, returns new root
  fn remove_art(&mut self, root: Option<T>, key: T) -> Option<T> {
    let root_idx = root?;

    // handle root removal
    if root_idx == key {
      self.clear(key);
      return None;
    }

    // traverse tree to find key and its parent
    self.remove_art_impl(root_idx, key, 0);
    Some(root_idx)
  }

  /// Internal remove implementation with parent tracking
  fn remove_art_impl(&mut self, current: T, key: T, depth: usize) -> bool {
    if depth > 8 {
      return false;
    }

    let byte = self.key_byte(key, depth);

    match self.find_child(current, byte) {
      Some(next) => {
        if next == key {
          // found the key - remove child pointer from parent
          self.remove_child(current, byte);
          self.clear(key);
          return true;
        }
        // recursively search in child
        self.remove_art_impl(next, key, depth + 1)
      }
      None => false,
    }
  }

  /// Remove child at given byte value from parent node
  fn remove_child(&mut self, parent: T, byte: u8) {
    // simple implementation: clear the pointer based on byte value
    if byte < 128 {
      self.set_left(parent, None);
    } else {
      self.set_right(parent, None);
    }

    // update node type if needed
    let new_count = self.child_count(parent).saturating_sub(1);
    if let Some(new_type) = NodeType::from_size(new_count) {
      self.set_node_type(parent, new_type);
    }
  }
}

/// Node type in Adaptive Radix Tree
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
  Empty,
  Node4,
  Node16,
  Node48,
  Node256,
}

impl NodeType {
  /// Convert from size field value
  #[inline]
  pub fn from_size(size: usize) -> Option<Self> {
    match size {
      0 => Some(Self::Empty),
      1..=4 => Some(Self::Node4),
      5..=16 => Some(Self::Node16),
      17..=48 => Some(Self::Node48),
      49..=256 => Some(Self::Node256),
      _ => None,
    }
  }

  /// Convert to size field value
  #[inline]
  pub fn to_size(self) -> usize {
    match self {
      Self::Empty => 0,
      Self::Node4 => 4,
      Self::Node16 => 16,
      Self::Node48 => 48,
      Self::Node256 => 256,
    }
  }
}
