// Property-based tests for doublets store operations
//
// These tests verify the correctness of the store under various random
// operation sequences, ensuring invariants are maintained.

use {
  doublets::{Doublets, Flow, Link, Links, create_heap_store},
  proptest::prelude::*,
};

/// Generate a sequence of store operations
#[derive(Debug, Clone)]
enum StoreOp {
  CreatePoint,
  CreateLink { source_idx: usize, target_idx: usize },
  DeleteLink { link_idx: usize },
  UpdateLink { link_idx: usize, source_idx: usize, target_idx: usize },
  Search { source_idx: usize, target_idx: usize },
  GetOrCreate { source_idx: usize, target_idx: usize },
}

fn arb_store_op() -> impl Strategy<Value = StoreOp> {
  prop_oneof![
    3 => Just(StoreOp::CreatePoint),
    2 => (0..20usize, 0..20usize).prop_map(|(s, t)| StoreOp::CreateLink {
      source_idx: s,
      target_idx: t,
    }),
    1 => (0..50usize).prop_map(|idx| StoreOp::DeleteLink { link_idx: idx }),
    1 => (0..50usize, 0..20usize, 0..20usize).prop_map(|(idx, s, t)| {
      StoreOp::UpdateLink { link_idx: idx, source_idx: s, target_idx: t }
    }),
    2 => (0..20usize, 0..20usize).prop_map(|(s, t)| StoreOp::Search {
      source_idx: s,
      target_idx: t,
    }),
    1 => (0..20usize, 0..20usize).prop_map(|(s, t)| StoreOp::GetOrCreate {
      source_idx: s,
      target_idx: t,
    }),
  ]
}

proptest! {
  #![proptest_config(proptest::prelude::ProptestConfig::with_cases(16))]

  /// Test that count is always consistent with actual links
  #[test]
  fn prop_count_consistency(
    ops in prop::collection::vec(arb_store_op(), 1..15)
  ) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut created_links: Vec<usize> = Vec::new();

    for op in ops {
      match op {
        StoreOp::CreatePoint => {
          let idx = store.create_point().unwrap();
          created_links.push(idx);
        }
        StoreOp::CreateLink { source_idx, target_idx } => {
          if !created_links.is_empty() {
            let source = created_links[source_idx % created_links.len()];
            let target = created_links[target_idx % created_links.len()];
            if let Ok(idx) = store.create_link(source, target) {
              created_links.push(idx);
            }
          }
        }
        StoreOp::DeleteLink { link_idx } => {
          if !created_links.is_empty() {
            let idx = created_links[link_idx % created_links.len()];
            if store.get(idx).is_some() && !store.has_usages(idx) {
              let _ = store.delete_link(idx);
              created_links.retain(|&x| x != idx);
            }
          }
        }
        StoreOp::UpdateLink { link_idx, source_idx, target_idx } => {
          if !created_links.is_empty() {
            let idx = created_links[link_idx % created_links.len()];
            let source = created_links[source_idx % created_links.len()];
            let target = created_links[target_idx % created_links.len()];
            if store.get(idx).is_some() {
              let _ = store.update_link(idx, source, target);
            }
          }
        }
        StoreOp::Search { source_idx, target_idx } => {
          if !created_links.is_empty() {
            let source = created_links[source_idx % created_links.len()];
            let target = created_links[target_idx % created_links.len()];
            let _ = store.search(source, target);
          }
        }
        StoreOp::GetOrCreate { source_idx, target_idx } => {
          if !created_links.is_empty() {
            let source = created_links[source_idx % created_links.len()];
            let target = created_links[target_idx % created_links.len()];
            if let Ok(idx) = store.get_or_create(source, target) {
              if !created_links.contains(&idx) {
                created_links.push(idx);
              }
            }
          }
        }
      }

      // Invariant: count_all should match collected links
      let actual_count = store.count_all();
      let mut existing_count = 0usize;
      store.each([], &mut |_: Link<usize>| {
        existing_count += 1;
        Flow::Continue
      });
      prop_assert_eq!(actual_count, existing_count);
    }
  }

  /// Test that search returns correct results
  #[test]
  fn prop_search_correctness(
    num_points in 2..20usize,
    num_links in 0..50usize
  ) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut points = Vec::new();

    for _ in 0..num_points {
      points.push(store.create_point().unwrap());
    }

    let mut created_links: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..num_links {
      let source = points[i % points.len()];
      let target = points[(i * 7) % points.len()];
      if let Ok(idx) = store.create_link(source, target) {
        created_links.push((idx, source, target));
      }
    }

    // Verify each created link can be found
    for (idx, source, target) in &created_links {
      if store.get(*idx).is_some() {
        let found = store.search(*source, *target);
        // Search should find it or another with same source/target
        prop_assert!(
          found.is_some(),
          "Link {}: {} -> {} not found",
          idx, source, target
        );
      }
    }
  }

  /// Test that get returns consistent data
  #[test]
  fn prop_get_consistency(
    ops in prop::collection::vec(arb_store_op(), 1..50)
  ) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut created_links: Vec<usize> = Vec::new();

    for op in ops {
      if let StoreOp::CreatePoint = op {
        let idx = store.create_point().unwrap();
        created_links.push(idx);

        // Verify get returns correct data for point
        let link = store.get(idx);
        prop_assert!(link.is_some());
        let link = link.unwrap();
        prop_assert_eq!(link.index, idx);
        prop_assert_eq!(link.source, idx);
        prop_assert_eq!(link.target, idx);
      }
    }
  }

  /// Test that iteration covers all links exactly once
  #[test]
  fn prop_iteration_completeness(num_ops in 1..30usize) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut link_indices = std::collections::HashSet::new();

    for _ in 0..num_ops {
      let idx = store.create_point().unwrap();
      link_indices.insert(idx);
    }

    // Collect all links via iteration
    let mut visited = std::collections::HashSet::new();
    store.each([], &mut |link: Link<usize>| {
      visited.insert(link.index);
      Flow::Continue
    });

    // All created links should be visited
    prop_assert_eq!(visited, link_indices);
  }

  /// Test that update preserves link identity but changes source/target
  #[test]
  fn prop_update_preserves_identity(num_points in 4..10usize) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut points = Vec::new();

    for _ in 0..num_points {
      points.push(store.create_point().unwrap());
    }

    // Create a link between two different points
    let link_idx = store.create_link(points[0], points[1]).unwrap();

    // Update to new source/target (indices 2 and 3)
    let new_source = points[2];
    let new_target = points[3];
    store.update_link(link_idx, new_source, new_target).unwrap();

    // Verify the link still exists with same index but new data
    let link = store.get(link_idx).unwrap();
    prop_assert_eq!(link.index, link_idx);
    prop_assert_eq!(link.source, new_source);
    prop_assert_eq!(link.target, new_target);

    // Old source/target search should not find this link
    let old_search = store.search(points[0], points[1]);
    prop_assert!(old_search.is_none() || old_search != Some(link_idx));

    // New source/target search should find some link
    let new_search = store.search(new_source, new_target);
    prop_assert!(new_search.is_some());
    // It may find a different link if one exists with same src/tgt
  }

  /// Test that delete removes link completely
  #[test]
  fn prop_delete_completeness(num_points in 2..10usize) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut points = Vec::new();

    for _ in 0..num_points {
      points.push(store.create_point().unwrap());
    }

    // Create and delete a link
    let link_idx = store.create_link(points[0], points[1]).unwrap();
    let source = points[0];
    let target = points[1];

    // Verify it exists
    prop_assert!(store.get(link_idx).is_some());

    // Delete it
    store.delete_link(link_idx).unwrap();

    // Verify it's gone
    prop_assert!(store.get(link_idx).is_none());

    // Search should not find it
    let found = store.search(source, target);
    prop_assert!(found.is_none() || found != Some(link_idx));

    // Count should not include it
    let count_before = store.count_all();

    // Re-create should get a new or same index (index reuse)
    let new_link_idx = store.create_link(source, target).unwrap();
    let count_after = store.count_all();
    prop_assert_eq!(count_after, count_before + 1);

    // The new link should be findable
    prop_assert!(store.get(new_link_idx).is_some());
  }

  /// Test get_or_create idempotency
  #[test]
  fn prop_get_or_create_idempotent(
    num_points in 2..10usize,
    repetitions in 1..10usize
  ) {
    let mut store = create_heap_store::<usize>().unwrap();
    let mut points = Vec::new();

    for _ in 0..num_points {
      points.push(store.create_point().unwrap());
    }

    let source = points[0];
    let target = points[1];

    // First get_or_create should create
    let first_idx = store.get_or_create(source, target).unwrap();
    let initial_count = store.count_all();

    // Subsequent calls should return same index
    for _ in 0..repetitions {
      let idx = store.get_or_create(source, target).unwrap();
      prop_assert_eq!(idx, first_idx);
      prop_assert_eq!(store.count_all(), initial_count);
    }
  }
}

/// Test rebase operation correctness
#[test]
fn test_rebase_correctness() {
  let mut store = create_heap_store::<usize>().unwrap();

  let a = store.create_point().unwrap();
  let b = store.create_point().unwrap();

  // Create links referencing 'a'
  let c = store.create_link(a, b).unwrap();
  let d = store.create_link(b, a).unwrap();

  // Rebase 'a' to 'b' - all references to 'a' should become 'b'
  store.rebase(a, b).unwrap();

  // Check that links now reference 'b'
  let link_c = store.get(c).unwrap();
  assert_eq!(link_c.source, b);

  let link_d = store.get(d).unwrap();
  assert_eq!(link_d.target, b);
}

/// Test count_usages correctness
#[test]
fn test_count_usages() {
  let mut store = create_heap_store::<usize>().unwrap();

  // Self-referencing: source=a, target=a
  let a = store.create_point().unwrap();
  let b = store.create_point().unwrap();

  // 'a' is used by itself (point), so usages should be 0 (excluding self)
  let usages_a = store.count_usages(a).unwrap();
  assert_eq!(usages_a, 0);

  // Create link from b to a - 'a' now has 1 usage as target
  let _c = store.create_link(b, a).unwrap();
  let usages_a_after = store.count_usages(a).unwrap();
  assert_eq!(usages_a_after, 1);

  // Create link from a to b - 'a' has 2 usages:
  // 1 as target (c), 1 as source (d)
  let _d = store.create_link(a, b).unwrap();
  let usages_a_final = store.count_usages(a).unwrap();
  assert_eq!(usages_a_final, 2);
}
