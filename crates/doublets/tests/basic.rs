use doublets::{Doublets, Flow, Link, Links, Result, create_heap_store};

#[test]
fn test_create_point() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  assert_eq!(a, 1);

  let link = store.get(a).ok_or(doublets::Error::NotExists(a))?;
  assert_eq!(link, Link::new(a, a, a));
  Ok(())
}

#[test]
fn test_create_link() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;

  let c = store.create_link(a, b)?;

  let link = store.get(c).ok_or(doublets::Error::NotExists(c))?;
  assert_eq!(link, Link::new(c, a, b));
  Ok(())
}

#[test]
fn test_update_link() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_point()?;

  store.update_link(c, a, b)?;

  let link = store.get(c).ok_or(doublets::Error::NotExists(c))?;
  assert_eq!(link, Link::new(c, a, b));
  Ok(())
}

#[test]
fn test_delete_link() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;

  store.delete_link(a)?;

  assert!(store.get(a).is_none());
  Ok(())
}

#[test]
fn test_search() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let c = store.create_link(a, b)?;

  let found = store.search(a, b);
  assert_eq!(found, Some(c));

  let not_found = store.search(b, a);
  assert_eq!(not_found, None);
  Ok(())
}

#[test]
fn test_count() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  assert_eq!(store.count_all(), 0);

  let _a = store.create_point()?;
  assert_eq!(store.count_all(), 1);

  let _b = store.create_point()?;
  assert_eq!(store.count_all(), 2);

  let _c = store.create_link(1, 2)?;
  assert_eq!(store.count_all(), 3);
  Ok(())
}

#[test]
#[ignore = "SBT remove bug causes infinite loop - see trees/src/sbt.rs:93"]
fn test_rebase() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;

  let c = store.create_point()?;
  store.update_link(c, c, a)?;

  let d = store.create_point()?;
  store.update_link(d, a, d)?;

  let links: Vec<_> = store.iter().collect();
  assert_eq!(
    links,
    vec![
      Link::new(a, a, a),
      Link::new(b, b, b),
      Link::new(c, c, a),
      Link::new(d, a, d)
    ]
  );

  store.rebase(a, b)?;

  let links: Vec<_> = store.iter().collect();
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

#[test]
fn test_each() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;

  let mut count = 0;
  store.each([], &mut |_link| {
    count += 1;
    Flow::Continue
  });

  assert_eq!(count, 3);
  Ok(())
}

#[test]
fn test_each_with_query() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;
  let _c = store.create_link(a, b)?;

  let mut found_links = Vec::new();

  store.each([0, a, 0], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  assert_eq!(found_links.len(), 2);
  Ok(())
}

#[test]
fn test_get_or_create() -> Result<(), usize> {
  let mut store = create_heap_store::<usize>()?;

  let a = store.create_point()?;
  let b = store.create_point()?;

  let c1 = store.get_or_create(a, b)?;
  let c2 = store.get_or_create(a, b)?;

  assert_eq!(c1, c2);
  assert_eq!(store.count_all(), 3);
  Ok(())
}
