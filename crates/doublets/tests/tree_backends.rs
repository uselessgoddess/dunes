// Tests demonstrating tree backend selection for the doublets store
//
// This module tests the ability to choose between different tree
// implementations (SBT - Size-Balanced Tree and ART - Adaptive Radix
// Tree) for source and target indexing.
//
// Both strategies maintain BST ordering by (source, target) tuples,
// enabling O(log n + k) range traversal and exact search for all
// strategy combinations.

use {
  doublets::{
    ArtStrategy, Doublets, Flow, Link, Links, RawLink, Result, SbtStrategy,
    Store, TreeStrategy,
  },
  mem::Alloc,
};

/// Helper to create a store with custom tree strategies
fn create_store<S, T>() -> Result<Store<usize, Alloc<RawLink>, S, T>, usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  Store::new(Alloc::new())
}

/// Macro to generate tests for a specific tree backend combination
macro_rules! define_tests_for_backend {
  ($src:ty, $tgt:ty, $suffix:literal) => {
    paste::paste! {
      #[test]
      fn [<test_basic_operations_ $suffix>]() -> Result<(), usize> {
        test_basic_operations::<$src, $tgt>()
      }

      #[test]
      fn [<test_iteration_ $suffix>]() -> Result<(), usize> {
        test_iteration::<$src, $tgt>()
      }

      #[test]
      fn [<test_query_by_source_ $suffix>]() -> Result<(), usize> {
        test_query_by_source::<$src, $tgt>()
      }

      #[test]
      fn [<test_query_by_target_ $suffix>]() -> Result<(), usize> {
        test_query_by_target::<$src, $tgt>()
      }

      #[test]
      fn [<test_update_consistency_ $suffix>]() -> Result<(), usize> {
        test_update_consistency::<$src, $tgt>()
      }

      #[test]
      fn [<test_scalability_ $suffix>]() -> Result<(), usize> {
        test_scalability::<$src, $tgt>()
      }
    }
  };
}

/// Macro to generate exact search tests for all backends
macro_rules! define_exact_search_tests {
  ($src:ty, $tgt:ty, $suffix:literal) => {
    paste::paste! {
      #[test]
      fn [<test_exact_search_ $suffix>]() -> Result<(), usize> {
        test_exact_search::<$src, $tgt>()
      }
    }
  };
}

// Generic test function that works with any tree backend combination
fn test_basic_operations<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;

  // Create some links
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  // Verify they exist
  assert_eq!(store.count_all(), 3);

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

fn test_iteration<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
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

fn test_query_by_source<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;
  let _d = store.create_link(a, a)?;

  let mut found_links = Vec::new();
  store.each([0, a, 0], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  // Point 'a' plus two links with source 'a'
  assert_eq!(found_links.len(), 3);

  Ok(())
}

fn test_query_by_target<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;
  let _d = store.create_link(b, b)?;

  let mut found_links = Vec::new();
  store.each([0, 0, b], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  // Point 'b' plus two links with target 'b'
  assert_eq!(found_links.len(), 3);

  Ok(())
}

// Test exact search for all tree backend combinations
fn test_exact_search<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  // Exact search works for all tree strategies
  assert_eq!(store.search(a, b), Some(c));
  assert_eq!(store.search(b, a), None);

  Ok(())
}

fn test_update_consistency<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;
  let link_ab = store.create_link(a, b)?;

  // Update should work consistently across all strategies
  store.update_link(link_ab, b, c)?;

  let link = store.get(link_ab).ok_or(doublets::Error::NotExists(link_ab))?;
  assert_eq!(link, Link::new(link_ab, b, c));

  Ok(())
}

fn test_scalability<S, T>() -> Result<(), usize>
where
  S: TreeStrategy<usize>,
  T: TreeStrategy<usize>,
{
  let mut store = create_store::<S, T>()?;
  let a = store.create_point()?;

  for _ in 0..50 {
    let b = store.create_point()?;
    let _link = store.create_link(a, b)?;
  }

  assert_eq!(store.count_all() as usize, 51 + 50); // 51 points + 50 links

  Ok(())
}

// Instantiate tests for all backend combinations
define_tests_for_backend!(SbtStrategy, SbtStrategy, "sbt_sbt");
define_tests_for_backend!(ArtStrategy, ArtStrategy, "art_art");
define_tests_for_backend!(SbtStrategy, ArtStrategy, "sbt_art");
define_tests_for_backend!(ArtStrategy, SbtStrategy, "art_sbt");

// Exact search tests for all backend combinations
define_exact_search_tests!(SbtStrategy, SbtStrategy, "sbt_sbt");
define_exact_search_tests!(SbtStrategy, ArtStrategy, "sbt_art");
define_exact_search_tests!(ArtStrategy, SbtStrategy, "art_sbt");
define_exact_search_tests!(ArtStrategy, ArtStrategy, "art_art");
