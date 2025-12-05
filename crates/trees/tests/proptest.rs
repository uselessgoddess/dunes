mod common;

use {
  common::Store,
  proptest::prelude::*,
  trees::{SizeBalanced, Tree},
};

// Comprehensive proptest cases to avoid corrupted tree bugs (issue #46)
// Using 128 cases for good coverage while keeping CI times reasonable
// For more extensive fuzzing, use cargo-fuzz (see crates/trees/fuzz/)
// 60s timeout per test case to avoid infinite recursion issues
proptest! {
  #![proptest_config(ProptestConfig {
    cases: 128,
    fork: true,  // Required for timeout to work
    timeout: 60000, // 60s timeout per test case (in milliseconds)
    .. ProptestConfig::default()
  })]

  #[test]
  fn prop_insert_and_search(
    values in prop::collection::hash_set(1usize..100, 1..20)
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let mut store: Store<usize> = Store::new(100);

    let mut root = None;
    for &v in &vals {
      root = store.insert(root, v);
    }

    // All inserted values should be found
    if let Some(r) = root {
      for &v in &vals {
        prop_assert!(store.contains(r, v), "Should contain {}", v);
      }
    }
  }

  #[test]
  fn prop_insert_maintains_size(
    values in prop::collection::hash_set(1usize..50, 1..15)
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let count = vals.len();
    let mut store: Store<usize> = Store::new(100);

    let mut root = None;
    for &v in &vals {
      root = store.insert(root, v);
    }

    if let Some(r) = root {
      let root_size = store.size(r).unwrap();
      let msg = "Root size equals insert count";
      prop_assert_eq!(root_size, count, "{}", msg);
    }
  }

  #[test]
  fn prop_nonexistent_not_found(
    values in prop::collection::hash_set(10usize..50, 1..15),
    search in 1usize..10
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let mut store: Store<usize> = Store::new(100);

    let mut root = None;
    for &v in &vals {
      root = store.insert(root, v);
    }

    if let Some(r) = root {
      prop_assert!(!store.contains(r, search),
        "Should not find value {} that was not inserted", search);
    }
  }

  #[test]
  fn prop_search_respects_ordering(v1 in 10usize..50, v2 in 50usize..100) {
    let mut store: Store<usize> = Store::new(100);

    let mut root = store.insert(None, v1);
    root = store.insert(root, v2);

    if let Some(r) = root {
      prop_assert!(store.contains(r, v1));
      prop_assert!(store.contains(r, v2));
    }
  }

  #[test]
  fn prop_mixed_insert_remove_operations(
    ops in prop::collection::vec(
      (1usize..100, prop::bool::ANY),
      1..50
    )
  ) {
    let mut store: Store<usize> = Store::new(200);
    let mut root = None;
    let mut inserted_values = std::collections::HashSet::new();

    for (value, is_insert) in ops {
      if is_insert {
        root = store.insert(root, value);
        inserted_values.insert(value);
      } else if inserted_values.contains(&value) {
        root = store.remove(root, value);
        inserted_values.remove(&value);
      }
    }

    // Verify all remaining values can be found
    if let Some(r) = root {
      for &v in &inserted_values {
        prop_assert!(store.contains(r, v), "Should contain {}", v);
      }

      // Verify tree size matches inserted values count
      let size = store.size(r).unwrap();
      prop_assert_eq!(size, inserted_values.len(),
        "Tree size should match number of inserted values");
    } else {
      prop_assert!(inserted_values.is_empty(),
        "Tree is empty but values remain: {:?}", inserted_values);
    }
  }

  #[test]
  fn prop_sequential_insert_then_remove_all(
    values in prop::collection::hash_set(1usize..100, 5..30)
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let mut store: Store<usize> = Store::new(200);

    // Insert all values
    let mut root = None;
    for &v in &vals {
      root = store.insert(root, v);
    }

    // Remove all values in reverse order
    for &v in vals.iter().rev() {
      root = store.remove(root, v);
    }

    // Tree should be empty
    prop_assert!(
      root.is_none(),
      "Tree should be empty after removing all values"
    );
  }

  #[test]
  fn prop_remove_reinsert_cycle(
    values in prop::collection::hash_set(10usize..50, 5..15),
    cycles in 1usize..5
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let mut store: Store<usize> = Store::new(200);

    let mut root = None;

    // Perform multiple cycles of insert/remove
    for _ in 0..cycles {
      // Insert all
      for &v in &vals {
        root = store.insert(root, v);
      }

      // Remove half
      for &v in vals.iter().take(vals.len() / 2) {
        root = store.remove(root, v);
      }
    }

    // Verify remaining values
    if let Some(r) = root {
      for &v in vals.iter().skip(vals.len() / 2) {
        prop_assert!(store.contains(r, v), "Should contain value {}", v);
      }
    }
  }

  #[test]
  fn prop_tree_size_invariant(
    values in prop::collection::hash_set(1usize..100, 10..40)
  ) {
    let vals: Vec<usize> = values.into_iter().collect();
    let mut store: Store<usize> = Store::new(200);

    let mut root = None;
    for &v in &vals {
      root = store.insert(root, v);
    }

    // Helper to count nodes recursively
    fn count_nodes<T: trees::Idx>(
      store: &Store<T>,
      node: Option<T>
    ) -> usize {
      match node {
        None => 0,
        Some(n) => {
          let left_count = count_nodes(store, store.left(n));
          let right_count = count_nodes(store, store.right(n));
          1 + left_count + right_count
        }
      }
    }

    if let Some(r) = root {
      let stored_size = store.size(r).unwrap();
      let actual_count = count_nodes(&store, Some(r));
      prop_assert_eq!(stored_size, actual_count,
        "Stored size should match actual node count");
    }
  }

  // Note: Disabled temporarily - too slow for CI (investigating optimization)
  #[test]
  fn prop_insert_remove_stress_test(
    ops in prop::collection::vec(
      (1usize..200, prop::bool::ANY),
      20..80
    )
  ) {
    let mut store: Store<usize> = Store::new(300);
    let mut root = None;
    let mut values = std::collections::HashSet::new();

    for (value, is_insert) in ops {
      if is_insert && !values.contains(&value) {
        // Insert
        root = store.insert(root, value);
        values.insert(value);
      } else if !is_insert && values.contains(&value) {
        // Remove
        root = store.remove(root, value);
        values.remove(&value);
      }
    }

    // Final verification
    if let Some(r) = root {
      let size = store.size(r).unwrap();
      prop_assert_eq!(size, values.len(),
        "Final tree size should match tracked values");

      for &v in &values {
        prop_assert!(store.contains(r, v),
          "Tree should contain all tracked values: {}", v);
      }
    } else {
      prop_assert!(values.is_empty(), "Empty tree should have no values");
    }
  }
}

#[test]
fn prop_sequential_inserts() {
  let mut store: Store<usize> = Store::new(20);

  let mut root = None;
  for i in 1..11 {
    root = store.insert(root, i);
  }

  if let Some(r) = root {
    assert_eq!(store.size(r).unwrap(), 10);
    for i in 1..11 {
      assert!(store.contains(r, i), "Should contain {}", i);
    }
  }
}

// Regression test for issue #44 - SBT remove bug
#[test]
fn regression_issue_44_sbt_remove() {
  let mut store: Store<usize> = Store::new(100);

  // Insert multiple values
  let values = vec![10, 5, 15, 3, 7, 12, 20];
  let mut root = None;
  for &v in &values {
    root = store.insert(root, v);
  }

  // Remove a few values
  root = store.remove(root, 7);
  root = store.remove(root, 15);

  // Verify correct size and remaining values can be found
  if let Some(r) = root {
    assert_eq!(store.size(r).unwrap(), 5);
    assert!(store.contains(r, 10));
    assert!(store.contains(r, 5));
    assert!(!store.contains(r, 7));
    assert!(!store.contains(r, 15));
    assert!(store.contains(r, 12));
  }
}
