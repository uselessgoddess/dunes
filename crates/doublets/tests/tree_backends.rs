// Tests demonstrating tree backend selection for the doublets store
//
// This module tests the ability to choose between different tree implementations
// (SBT - Size-Balanced Tree and ART - Adaptive Radix Tree) for source and target indexing.
//
// Users can specify tree backends like:
// - Store<T, M, SbtStrategy, SbtStrategy> - Both SBT (fully functional)
// - Store<T, M, ArtStrategy, ArtStrategy> - Both ART (demonstration, search not yet implemented)
// - Store<T, M, SbtStrategy, ArtStrategy> - Mixed strategies
//
// Note: ART implementation is currently a simplified demonstration and doesn't support
// search operations. Only SBT backend is fully functional. Future work can complete the
// ART implementation to make it fully operational.

use doublets::{
  ArtStrategy, Doublets, Flow, Link, Links, Result, SbtStrategy,
  create_heap_store, create_heap_store_with_strategies,
};

/// Test that the default create_heap_store uses SBT for both trees
#[test]
fn test_default_sbt_backend() -> Result<(), usize> {
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

/// Test explicit SBT backend for both source and target trees
#[test]
fn test_explicit_sbt_sbt_backend() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  assert_eq!(store.count_all(), 3);
  assert_eq!(store.search(a, b), Some(c));

  Ok(())
}

/// Test that store can be created with ART backend (compilation check)
/// Note: Full ART operations not yet implemented, so we only test creation
#[test]
fn test_art_backend_compilation() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, ArtStrategy, ArtStrategy>()?;

  // Create points works
  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;

  // Count works
  assert_eq!(store.count_all(), 3);

  // Note: search operation not tested as ART implementation is incomplete
  // Future work: implement full ART search functionality

  Ok(())
}

/// Test mixed strategy: SBT for source, ART for target (compilation check)
#[test]
fn test_mixed_sbt_art_backend() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, ArtStrategy>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;

  assert_eq!(store.count_all(), 3);
  // SBT source tree works, so source-based queries work
  // But full search requires both trees to work

  Ok(())
}

/// Test mixed strategy: ART for source, SBT for target (compilation check)
#[test]
fn test_mixed_art_sbt_backend() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, ArtStrategy, SbtStrategy>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;

  assert_eq!(store.count_all(), 3);

  Ok(())
}

/// Test that different tree backends handle iteration correctly
#[test]
fn test_tree_backend_iteration_sbt() -> Result<(), usize> {
  // Test with SBT/SBT
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
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

/// Test querying by source works with SBT backend
#[test]
fn test_query_by_source_sbt() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
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

/// Test querying by target works with SBT backend
#[test]
fn test_query_by_target_sbt() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
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

/// Test exact search with SBT backend
#[test]
fn test_exact_search_sbt() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  assert_eq!(store.search(a, b), Some(c));
  assert_eq!(store.search(b, a), None);

  Ok(())
}

/// Test tree backend handles updates correctly
#[test]
fn test_update_consistency_sbt() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;
  let link_ab = store.create_link(a, b)?;

  store.update_link(link_ab, b, c)?;
  assert_eq!(store.search(a, b), None);
  assert_eq!(store.search(b, c), Some(link_ab));

  Ok(())
}

/// Test scalability with SBT backend
#[test]
fn test_scalability_sbt() -> Result<(), usize> {
  let mut store =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
  let a = store.create_point()?;

  for i in 0..50 {
    let b = store.create_point()?;
    let _link = store.create_link(a, b)?;

    if i % 10 == 0 {
      let found = store.search(a, b);
      assert!(found.is_some());
    }
  }

  assert_eq!(store.count_all() as usize, 51 + 50); // 51 points + 50 links

  Ok(())
}

/// Demonstrate factory function usage for different backends
#[test]
fn test_factory_functions() -> Result<(), usize> {
  // Default SBT store
  let mut sbt_store = create_heap_store::<usize>()?;
  let _a = sbt_store.create_point()?;
  assert_eq!(sbt_store.count_all(), 1);

  // Explicit SBT/SBT store
  let mut sbt_explicit =
    create_heap_store_with_strategies::<usize, SbtStrategy, SbtStrategy>()?;
  let _b = sbt_explicit.create_point()?;
  assert_eq!(sbt_explicit.count_all(), 1);

  // ART store (compilation check)
  let mut art_store =
    create_heap_store_with_strategies::<usize, ArtStrategy, ArtStrategy>()?;
  let _c = art_store.create_point()?;
  assert_eq!(art_store.count_all(), 1);

  // Mixed SBT/ART store (compilation check)
  let mut mixed_store =
    create_heap_store_with_strategies::<usize, SbtStrategy, ArtStrategy>()?;
  let _d = mixed_store.create_point()?;
  assert_eq!(mixed_store.count_all(), 1);

  Ok(())
}
