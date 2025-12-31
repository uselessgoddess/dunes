use {
  doublets::{Doublets, Flow, Link, Links, Result, create_heap_store},
  std::collections::HashSet,
};

#[test]
fn test_basic_create() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let mut index = 0;
  store.create([], &mut |_before: Link<usize>, after: Link<usize>| {
    index = after.index;
    println!("Created link with index: {}", index);
    Flow::Continue
  })?;

  println!("Checking if link {} exists...", index);
  let link = store.get(index);
  println!("Result: {:?}", link);
  assert!(link.is_some(), "Link should exist");
  Ok(())
}

#[test]
fn debug_each_with_query() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  println!("Created links:");
  println!("  a = {} (point a->a)", a);
  println!("  b = {} (point b->b)", b);
  println!("  c = {} (link a->b)", c);

  println!("\nAll links in store:");
  store.each([], &mut |link| {
    println!("  {:?}", link);
    Flow::Continue
  });

  println!(
    "\nSearching for all links with source = {} (query [0, {}, 0]):",
    a, a
  );
  let mut found_links = Vec::new();
  let mut call_count = 0;

  store.each([0, a, 0], &mut |link| {
    call_count += 1;
    println!("  Call #{}: Found: {:?}", call_count, link);
    found_links.push(link);
    Flow::Continue
  });

  println!("\nExpected: 2 links (a->a and c->b)");
  println!(
    "Actually found: {} links in {} calls",
    found_links.len(),
    call_count
  );
  println!("Links: {:?}", found_links);

  // Check for duplicates
  let mut unique_indices = std::collections::HashSet::new();
  for link in &found_links {
    if !unique_indices.insert(link.index) {
      println!("WARNING: Duplicate link index: {}", link.index);
    }
  }

  assert_eq!(
    found_links.len(),
    2,
    "Expected 2 links but found {}",
    found_links.len()
  );

  Ok(())
}

#[test]
fn debug_rebase_simple() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;

  println!("Created a={}, b={}", a, b);
  println!("About to call rebase(a={}, b={})...", a, b);

  // This should replace all occurrences of 'a' with 'b'
  // But if there are no other links using 'a', it should be quick
  store.rebase(a, b)?;

  println!("Rebase completed successfully!");
  Ok(())
}

#[test]
fn debug_rebase_full() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;

  let c = store.create_point()?;
  store.update_link(c, c, a)?;

  let d = store.create_point()?;
  store.update_link(d, a, d)?;

  println!("Created links:");
  println!("  a={} (point a->a)", a);
  println!("  b={} (point b->b)", b);
  println!("  c={} (link c->a)", c);
  println!("  d={} (link a->d)", d);

  let links: Vec<_> = store.iter().collect();
  println!("\nLinks before rebase: {:?}", links);

  println!("\nAbout to call rebase(a={}, b={})...", a, b);
  store.rebase(a, b)?;
  println!("Rebase completed successfully!");

  let links: Vec<_> = store.iter().collect();
  println!("\nLinks after rebase: {:?}", links);

  assert_eq!(
    links,
    vec![
      Link::new(a, a, a),
      Link::new(b, b, b),
      Link::new(c, c, b),
      Link::new(d, b, d)
    ]
  );
  Ok(())
}

/// Test tree traversal with delete operations to verify correctness
#[test]
fn test_tree_traversal_with_delete() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  // Create a bunch of points and links
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;

  let link_ab = store.create_link(a, b)?;
  let link_ac = store.create_link(a, c)?;
  let link_bc = store.create_link(b, c)?;
  let link_ba = store.create_link(b, a)?;
  let link_ca = store.create_link(c, a)?;

  println!("Created:");
  println!("  Points: a={}, b={}, c={}", a, b, c);
  println!(
    "  Links: ab={}, ac={}, bc={}, ba={}, ca={}",
    link_ab, link_ac, link_bc, link_ba, link_ca
  );

  // Query by source = a: should find a (point), link_ab, link_ac
  let mut found: HashSet<usize> = HashSet::new();
  store.each([0, a, 0], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });
  println!("Source={}: {:?}", a, found);
  assert!(found.contains(&a));
  assert!(found.contains(&link_ab));
  assert!(found.contains(&link_ac));
  assert_eq!(found.len(), 3);

  // Delete link_ab
  store.delete_link(link_ab)?;
  println!("Deleted link_ab={}", link_ab);

  // Query by source = a: should find a (point), link_ac only
  found.clear();
  store.each([0, a, 0], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });
  println!("Source={} after delete: {:?}", a, found);
  assert!(found.contains(&a));
  assert!(found.contains(&link_ac));
  assert!(!found.contains(&link_ab)); // Should NOT be present
  assert_eq!(found.len(), 2);

  // Query by target = a: should find a (point), link_ba, link_ca
  found.clear();
  store.each([0, 0, a], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });
  println!("Target={}: {:?}", a, found);
  assert!(found.contains(&a));
  assert!(found.contains(&link_ba));
  assert!(found.contains(&link_ca));
  assert_eq!(found.len(), 3);

  // Delete link_ca
  store.delete_link(link_ca)?;
  println!("Deleted link_ca={}", link_ca);

  // Query by target = a: should find a (point), link_ba only
  found.clear();
  store.each([0, 0, a], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });
  println!("Target={} after delete: {:?}", a, found);
  assert!(found.contains(&a));
  assert!(found.contains(&link_ba));
  assert!(!found.contains(&link_ca)); // Should NOT be present
  assert_eq!(found.len(), 2);

  Ok(())
}

/// Test with many links to stress-test tree operations
#[test]
fn test_tree_traversal_stress() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  // Create 10 points
  let points: Vec<_> = (0..10).map(|_| store.create_point().unwrap()).collect();

  // Create links from each point to all others
  let mut links = Vec::new();
  for i in 0..10 {
    for j in 0..10 {
      if i != j {
        let link = store.create_link(points[i], points[j]).unwrap();
        links.push((link, points[i], points[j]));
      }
    }
  }

  println!("Created {} points and {} links", 10, links.len());

  // Verify querying by source works for first point
  let first_point = points[0];
  let mut found: HashSet<usize> = HashSet::new();
  store.each([0, first_point, 0], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });

  // Should find the point itself + 9 outgoing links
  assert_eq!(found.len(), 10, "Expected 10 links with source={}", first_point);
  assert!(found.contains(&first_point));

  // Delete all links from first point
  for (link, source, _) in &links {
    if *source == first_point {
      store.delete_link(*link)?;
    }
  }

  // Now should only find the point
  found.clear();
  store.each([0, first_point, 0], &mut |link: Link<usize>| {
    found.insert(link.index);
    Flow::Continue
  });
  assert_eq!(found.len(), 1);
  assert!(found.contains(&first_point));

  println!("Stress test passed!");
  Ok(())
}

/// Test to specifically verify tree traversal after deletes
/// This tests the scenario where linear scan is used vs tree traversal
#[test]
fn test_tree_vs_linear_consistency() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  // Create points and links
  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;

  let link1 = store.create_link(a, b)?;
  let link2 = store.create_link(a, c)?;
  let _link3 = store.create_link(b, a)?;

  // Collect results using current implementation (linear scan)
  let mut linear_results: HashSet<usize> = HashSet::new();
  store.each([0, a, 0], &mut |link: Link<usize>| {
    linear_results.insert(link.index);
    Flow::Continue
  });

  println!("Results for source={}: {:?}", a, linear_results);

  // Should contain: a (point), link1, link2
  assert!(linear_results.contains(&a), "Should contain point a");
  assert!(linear_results.contains(&link1), "Should contain link1");
  assert!(linear_results.contains(&link2), "Should contain link2");
  assert_eq!(linear_results.len(), 3);

  // Now delete link1 and verify again
  store.delete_link(link1)?;

  let mut after_delete: HashSet<usize> = HashSet::new();
  store.each([0, a, 0], &mut |link: Link<usize>| {
    after_delete.insert(link.index);
    Flow::Continue
  });

  println!("After delete, source={}: {:?}", a, after_delete);

  // Should contain: a (point), link2 only
  assert!(after_delete.contains(&a), "Should contain point a");
  assert!(after_delete.contains(&link2), "Should contain link2");
  assert!(!after_delete.contains(&link1), "Should NOT contain deleted link1");
  assert_eq!(after_delete.len(), 2);

  // Create a new link that reuses the index
  let new_link = store.create_link(c, a)?;
  println!("New link {} reused index? {}", new_link, new_link == link1);

  // Query again - should reflect new state
  let mut final_results: HashSet<usize> = HashSet::new();
  store.each([0, a, 0], &mut |link: Link<usize>| {
    final_results.insert(link.index);
    Flow::Continue
  });

  println!("Final source={}: {:?}", a, final_results);

  // Should contain: a (point), link2
  // new_link has source=c, so should NOT be in source=a results
  assert!(final_results.contains(&a), "Should contain point a");
  assert!(final_results.contains(&link2), "Should contain link2");
  assert_eq!(final_results.len(), 2);

  Ok(())
}
