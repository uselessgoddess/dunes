#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use std::collections::HashSet;
use trees::{Idx, Node, SizeBalanced, Tree};

/// A simple tree store implementation for fuzzing
struct Store<T> {
  nodes: Vec<Node<T>>,
}

impl<T: Idx> Store<T> {
  fn new(capacity: usize) -> Self {
    Self {
      nodes: vec![Node::default(); capacity],
    }
  }
}

impl<T: Idx> Tree<T> for Store<T> {
  fn get(&self, idx: T) -> Option<Node<T>> {
    self.nodes.get(idx.as_usize()).copied()
  }

  fn set(&mut self, idx: T, node: Node<T>) {
    if let Some(n) = self.nodes.get_mut(idx.as_usize()) {
      *n = node;
    }
  }

  fn left_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize()).and_then(|n| n.left.as_mut())
  }

  fn right_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize()).and_then(|n| n.right.as_mut())
  }

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

#[derive(Debug, Clone, Arbitrary)]
enum TreeOp {
  Insert(u8),
  Remove(u8),
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
  ops: Vec<TreeOp>,
}

fuzz_target!(|data: &[u8]| {
  // Parse fuzzer input
  let mut unstructured = Unstructured::new(data);
  let Ok(input) = FuzzInput::arbitrary(&mut unstructured) else {
    return;
  };

  // Limit operations to prevent timeouts
  let ops: Vec<_> = input.ops.into_iter().take(200).collect();
  if ops.is_empty() {
    return;
  }

  let mut store = Store::<usize>::new(256);
  let mut root = None;
  let mut inserted_values = HashSet::new();

  // Execute operations
  for op in ops {
    match op {
      TreeOp::Insert(val) => {
        let value = (val as usize) + 1; // Avoid 0
        if value < 256 {
          root = store.insert(root, value);
          inserted_values.insert(value);
        }
      }
      TreeOp::Remove(val) => {
        let value = (val as usize) + 1;
        if inserted_values.contains(&value) {
          root = store.remove(root, value);
          inserted_values.remove(&value);
        }
      }
    }

    // Verify tree consistency after each operation
    if let Some(r) = root {
      // Check that size field is correct
      let stored_size = store.size(r).unwrap();

      // Count actual nodes
      fn count_nodes<T: Idx>(store: &Store<T>, node: Option<T>) -> usize {
        match node {
          None => 0,
          Some(n) => {
            let left_count = count_nodes(store, store.left(n));
            let right_count = count_nodes(store, store.right(n));
            1 + left_count + right_count
          }
        }
      }

      let actual_count = count_nodes(&store, Some(r));

      // Assert invariants
      assert_eq!(
        stored_size, actual_count,
        "Size mismatch: stored={}, actual={}",
        stored_size, actual_count
      );
      assert_eq!(
        stored_size,
        inserted_values.len(),
        "Size mismatch with tracking: stored={}, tracked={}",
        stored_size,
        inserted_values.len()
      );

      // Verify all inserted values can be found
      for &v in &inserted_values {
        assert!(
          store.contains(r, v),
          "Tree should contain value {} but doesn't",
          v
        );
      }
    } else {
      // Tree is empty
      assert!(
        inserted_values.is_empty(),
        "Tree is None but tracking shows values: {:?}",
        inserted_values
      );
    }
  }
});
