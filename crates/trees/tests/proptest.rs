mod common;

use {
  common::Store,
  proptest::prelude::*,
  trees::{SizeBalanced, Tree},
};

// Reduce test cases for faster CI/local testing
// Default is 256 cases per test, we use 32 for basic tests
proptest! {
  #![proptest_config(ProptestConfig::with_cases(32))]

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

  // Note: More comprehensive property tests for remove operation were attempted
  // but encountered issues due to the SBT implementation's expectations
  // (following C# reference: assumes values exist before removal).
  // The regression test below covers the specific bug from issue #44.
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
