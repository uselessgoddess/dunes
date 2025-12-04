use doublets::{Doublets, Flow, Links, create_heap_store};

fn main() {
  let mut store = create_heap_store::<usize>().unwrap();

  let a = store.create_point().unwrap();
  assert_eq!(a, 1);
  
  let b = store.create_point().unwrap();
  assert_eq!(b, 2);
  
  let c = store.create_link(a, b).unwrap();
  assert_eq!(c, 3);

  println!("Created: a={}, b={}, c={}", a, b, c);
  println!("\nAll links:");
  store.each([], &mut |link| {
    println!("  Index {}: {} -> {}", link.index, link.source, link.target);
    Flow::Continue
  });

  println!("\nSearching for source={}:", a);
  let mut found = Vec::new();
  store.each([0, a, 0], &mut |link| {
    println!("  Found Index {}: {} -> {}", link.index, link.source, link.target);
    found.push(link.index);
    Flow::Continue
  });

  println!("\nExpected to find 2 links (indices 1 and 3), found {}:", found.len());
  for idx in &found {
    println!("  {}", idx);
  }

  if found.len() != 2 {
    println!("\nERROR: Expected 2, got {}", found.len());
    std::process::exit(1);
  }
}
