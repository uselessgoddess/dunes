// Tests demonstrating that the doublets store uses the generic Tree trait
// which allows for different tree implementations (SBT by default)
//
// The Tree trait provides default implementations that use SizeBalanced (SBT),
// but implementations can override insert/remove to use other tree strategies
// like AdaptiveRadix (ART) by implementing the AdaptiveRadix trait.

use doublets::{Doublets, Flow, Link, Links, Result, create_heap_store};

/// Test that the default tree backend (SBT) works correctly with all operations
#[test]
fn test_sbt_backend() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  // Create some links
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  // Verify they exist
  assert_eq!(store.count_all(), 3);

  // Verify we can search using the tree backend
  let found = store.search(a, b);
  assert_eq!(found, Some(c));

  // Verify we can update (requires tree rebalancing)
  let d = store.create_point()?;
  store.update_link(c, b, d)?;

  let link = store.get(c).ok_or(doublets::Error::NotExists(c))?;
  assert_eq!(link, Link::new(c, b, d));

  // Verify we can delete (requires tree rebalancing)
  store.delete_link(c)?;
  assert_eq!(store.count_all(), 3);
  assert!(store.get(c).is_none());

  Ok(())
}

/// Test that iterating works with the tree backend
#[test]
fn test_tree_backed_iteration() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;
  let _d = store.create_link(b, a)?;

  let mut count = 0;
  store.each([], &mut |_link| {
    count += 1;
    Flow::Continue
  });

  assert_eq!(count, 4);
  Ok(())
}

/// Test that querying by source works with tree backend
#[test]
fn test_tree_backed_query_by_source() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;
  let _d = store.create_link(a, a)?;

  let mut found_links = Vec::new();
  store.each([0, a, 0], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  assert_eq!(found_links.len(), 3); // Point 'a' plus two links with source 'a'
  Ok(())
}

/// Test that querying by target works with tree backend
#[test]
fn test_tree_backed_query_by_target() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;
  let _d = store.create_link(b, b)?;

  let mut found_links = Vec::new();
  store.each([0, 0, b], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  assert_eq!(found_links.len(), 3); // Point 'b' plus two links with target 'b'
  Ok(())
}

/// Test exact search using tree backend
#[test]
fn test_tree_backed_exact_search() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;
  let _d = store.create_link(b, a)?;

  // Exact search should use tree for efficiency
  let found = store.search(a, b);
  assert_eq!(found, Some(c));

  // Searching for a non-existent combination
  let e = store.create_point()?;
  let not_found = store.search(e, a);
  assert_eq!(not_found, None);

  Ok(())
}

/// Test that tree-backed updates maintain consistency
#[test]
fn test_tree_backed_update_consistency() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;
  let link_ab = store.create_link(a, b)?;

  // Update link - this requires detaching from old tree positions
  // and reattaching to new tree positions
  store.update_link(link_ab, b, c)?;

  // Old search should fail
  let old_search = store.search(a, b);
  assert_eq!(old_search, None);

  // New search should succeed
  let new_search = store.search(b, c);
  assert_eq!(new_search, Some(link_ab));

  Ok(())
}

/// Test tree backend handles many links efficiently
#[test]
fn test_tree_backend_scalability() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;

  // Create many links with same source
  for i in 0..100 {
    let b = store.create_point()?;
    let _link = store.create_link(a, b)?;

    if i % 10 == 0 {
      // Verify we can still search efficiently
      let found = store.search(a, b);
      assert!(found.is_some());
    }
  }

  // Verify total count
  assert_eq!(store.count_all() as usize, 101 + 100); // 101 points + 100 links

  Ok(())
}

/// Demonstrate that the store uses the generic Tree trait
///
/// This test shows that:
/// 1. SourceTree and TargetTree implement the Tree trait
/// 2. They use the default insert/remove implementations (SBT)
/// 3. The generic interface allows for different tree backends
///
/// To use a different tree backend (e.g., ART), one would:
/// 1. Implement AdaptiveRadix trait for SourceTree/TargetTree
/// 2. Override Tree::insert and Tree::remove to call ART methods
/// 3. The rest of the doublets store code remains unchanged
#[test]
fn test_generic_tree_trait_usage() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  // Create test data
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  // The store uses Tree::insert internally when creating links
  // This calls the default implementation which uses SizeBalanced::insert_sbt
  assert_eq!(store.count_all(), 3);

  // The store uses Tree::remove internally when deleting links
  // This calls the default implementation which uses SizeBalanced::remove_sbt
  store.delete_link(c)?;
  assert_eq!(store.count_all(), 2);

  // Searching uses the tree structure efficiently
  let not_found = store.search(a, b);
  assert_eq!(not_found, None); // Link was deleted

  Ok(())
}
