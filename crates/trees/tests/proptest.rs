use {dunes_trees::{SizeBalanced, Store, Tree}, proptest::prelude::*};

proptest! {
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
