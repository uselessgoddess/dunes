use {mem::RawMem, std::error::Error};

type Result = std::result::Result<(), Box<dyn Error>>;

pub fn basic_invariants<M: RawMem<Item = u64>>(mut mem: M) -> Result {
  assert_eq!(mem.as_slice().len(), 0);

  mem.grow(10)?.zeroed();
  assert_eq!(mem.as_slice().len(), 10);
  assert_eq!(mem.as_slice(), &[0u64; 10]);

  mem.grow(5)?.filled(42);
  assert_eq!(mem.as_slice().len(), 15);

  mem.shrink(5)?;
  assert_eq!(mem.as_slice().len(), 10);

  mem.shrink(10)?;
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}

pub fn edge_cases<M: RawMem<Item = u8>>(mut mem: M) -> Result {
  mem.grow(0)?.zeroed();
  assert_eq!(mem.as_slice().len(), 0);

  mem.grow(10)?.filled(123);
  assert_eq!(mem.as_slice().len(), 10);

  mem.shrink(0)?;
  assert_eq!(mem.as_slice().len(), 10);

  mem.shrink(100)?;
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}

pub fn mutability<M: RawMem<Item = u32>>(mut mem: M) -> Result {
  mem.grow(10)?.zeroed();

  for (i, elem) in mem.as_mut_slice().iter_mut().enumerate() {
    *elem = i as u32;
  }

  for (i, &elem) in mem.as_slice().iter().enumerate() {
    assert_eq!(elem, i as u32);
  }

  Ok(())
}
