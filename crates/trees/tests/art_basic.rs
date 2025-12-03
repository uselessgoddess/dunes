mod common;

use {common::ArtStore, trees::Tree};

#[test]
fn test_art_single_insert() {
  let mut store: ArtStore<usize> = ArtStore::new(10);
  let root = store.insert(None, 5);
  assert_eq!(root, Some(5));

  // Check the node was initialized
  let node = store.get(5).unwrap();
  assert!(node.size > 0, "Node should be initialized");
}

#[test]
fn test_art_two_inserts() {
  let mut store: ArtStore<usize> = ArtStore::new(10);

  let mut root = store.insert(None, 5);
  assert_eq!(root, Some(5));

  root = store.insert(root, 3);
  assert!(root.is_some(), "Root should not be None after second insert");
}

#[test]
fn test_art_multiple_inserts() {
  let mut store: ArtStore<usize> = ArtStore::new(100);

  let mut root = None;
  for i in 1..10 {
    root = store.insert(root, i);
  }

  assert!(root.is_some(), "Root should exist after multiple inserts");
}

#[test]
fn test_art_insert_and_remove() {
  let mut store: ArtStore<usize> = ArtStore::new(10);

  let mut root = store.insert(None, 5);
  assert_eq!(root, Some(5));

  root = store.remove(root, 5);
  // After removing the only element, tree should be empty
  assert_eq!(root, None);
}
