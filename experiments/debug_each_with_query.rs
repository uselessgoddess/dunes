use doublets::{Doublets, Flow, Link, Links, create_heap_store};

fn main() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  println!("Created links:");
  println!("  a = {} (point a->a)", a);
  println!("  b = {} (point b->b)", b);
  println!("  c = {} (link a->b)", c);

  println!("\nSearching for all links with source = {} (query [0, {}, 0]):", a, a);
  let mut found_links = Vec::new();

  store.each([0, a, 0], &mut |link| {
    println!("  Found: {:?}", link);
    found_links.push(link);
    Flow::Continue
  });

  println!("\nExpected: 2 links (a->a and c->b)");
  println!("Actually found: {} links", found_links.len());

  if found_links.len() != 2 {
    println!("\n❌ TEST FAILED: Expected 2 links but found {}", found_links.len());
    return Err(1);
  }

  println!("\n✅ TEST PASSED");
  Ok(())
}
