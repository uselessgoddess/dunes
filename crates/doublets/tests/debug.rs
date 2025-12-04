use doublets::{Flow, Link, Links, Result, create_heap_store};

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
