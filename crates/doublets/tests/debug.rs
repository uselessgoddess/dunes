use doublets::{Doublets, Flow, Link, Links, Result, create_heap_store};

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
