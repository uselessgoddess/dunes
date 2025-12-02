use {
  mem::{PreAlloc, RawMem},
  std::error::Error,
};

type Result = std::result::Result<(), Box<dyn Error>>;

#[test]
fn ignore_grow_fillup() -> Result {
  let mut slice = [1, 2, 3];
  let mut mem = PreAlloc::new(&mut slice[..]);

  mem.grow(3)?.zeroed();

  assert_eq!(mem.as_slice().to_vec(), slice);

  Ok(())
}

#[test]
fn prealloc_overgrow() {
  let mut buf = [0u64; 10];
  let mut mem = PreAlloc::new(&mut buf[..]);

  mem.grow(10).unwrap().zeroed();
  assert_eq!(mem.as_slice().len(), 10);

  assert!(mem.grow(1).is_err());
}
