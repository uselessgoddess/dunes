use {
  mem::{Alloc, PreAlloc, RawMem},
  std::error::Error,
};

type Result = std::result::Result<(), Box<dyn Error>>;

/// Test invariants that must hold for any RawMem implementation
fn test_invariants<M: RawMem<Item = u64>>(mut mem: M) -> Result {
  // Initial state: empty
  assert_eq!(mem.as_slice().len(), 0);

  // Grow and check length
  mem.grow(10)?.zeroed();
  assert_eq!(mem.as_slice().len(), 10);
  assert_eq!(mem.as_slice(), &[0u64; 10]);

  // Grow again and check cumulative length
  mem.grow(5)?.filled(42);
  assert_eq!(mem.as_slice().len(), 15);
  assert_eq!(&mem.as_slice()[0..10], &[0u64; 10]);
  assert_eq!(&mem.as_slice()[10..15], &[42u64; 5]);

  // Shrink and verify
  mem.shrink(5)?;
  assert_eq!(mem.as_slice().len(), 10);
  assert_eq!(mem.as_slice(), &[0u64; 10]);

  // Shrink to empty
  mem.shrink(10)?;
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}

/// Test grow/shrink cycle behavior
fn test_grow_shrink_cycles<M: RawMem<Item = i32>>(mut mem: M) -> Result {
  const CYCLES: usize = 100;
  const SIZE: usize = if cfg!(miri) { 10 } else { 100 };

  for i in 0..CYCLES {
    // Grow with unique value for this cycle
    mem.grow(SIZE)?.filled(i as i32);
    assert_eq!(mem.as_slice().len(), SIZE * (i + 1));

    // Verify all previous cycles are intact
    for j in 0..=i {
      let start = j * SIZE;
      let end = (j + 1) * SIZE;
      assert_eq!(
        &mem.as_slice()[start..end],
        &vec![j as i32; SIZE][..],
        "Cycle {} data corrupted at iteration {}",
        j,
        i
      );
    }
  }

  // Shrink back down
  for i in (0..CYCLES).rev() {
    mem.shrink(SIZE)?;
    assert_eq!(mem.as_slice().len(), SIZE * i);
  }

  assert_eq!(mem.as_slice().len(), 0);
  Ok(())
}

/// Test edge cases like zero-sized operations
fn test_edge_cases<M: RawMem<Item = u8>>(mut mem: M) -> Result {
  // Grow by 0 should be a no-op
  mem.grow(0)?.zeroed();
  assert_eq!(mem.as_slice().len(), 0);

  // Grow by some amount
  mem.grow(10)?.filled(123);
  assert_eq!(mem.as_slice().len(), 10);

  // Shrink by 0 should be a no-op
  mem.shrink(0)?;
  assert_eq!(mem.as_slice().len(), 10);

  // Shrink more than available should saturate to 0
  mem.shrink(100)?;
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}

/// Test that as_mut_slice actually allows mutation
fn test_mutability<M: RawMem<Item = u32>>(mut mem: M) -> Result {
  mem.grow(10)?.zeroed();

  // Mutate through as_mut_slice
  for (i, elem) in mem.as_mut_slice().iter_mut().enumerate() {
    *elem = i as u32;
  }

  // Verify mutation persisted
  for (i, &elem) in mem.as_slice().iter().enumerate() {
    assert_eq!(elem, i as u32);
  }

  Ok(())
}

/// Test large allocations (stress test)
fn test_large_allocation<M: RawMem<Item = u64>>(mut mem: M) -> Result {
  const LARGE: usize = if cfg!(miri) { 1000 } else { 100_000 };

  mem.grow(LARGE)?.filled(0xDEADBEEF);
  assert_eq!(mem.as_slice().len(), LARGE);
  assert!(mem.as_slice().iter().all(|&x| x == 0xDEADBEEF));

  mem.shrink(LARGE)?;
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}

/// Test with various Pod types
fn test_different_types<M: RawMem<Item = u8>>(mut mem: M) -> Result {
  mem.grow(256)?.zeroed();

  // Fill with pattern
  for (i, byte) in mem.as_mut_slice().iter_mut().enumerate() {
    *byte = (i & 0xFF) as u8;
  }

  // Verify pattern
  for (i, &byte) in mem.as_slice().iter().enumerate() {
    assert_eq!(byte, (i & 0xFF) as u8);
  }

  Ok(())
}

// Tests for Alloc implementation
#[test]
fn alloc_invariants() -> Result {
  test_invariants(Alloc::<u64>::new())
}

#[test]
fn alloc_grow_shrink_cycles() -> Result {
  test_grow_shrink_cycles(Alloc::<i32>::new())
}

#[test]
fn alloc_edge_cases() -> Result {
  test_edge_cases(Alloc::<u8>::new())
}

#[test]
fn alloc_mutability() -> Result {
  test_mutability(Alloc::<u32>::new())
}

#[test]
fn alloc_large_allocation() -> Result {
  test_large_allocation(Alloc::<u64>::new())
}

#[test]
fn alloc_different_types() -> Result {
  test_different_types(Alloc::<u8>::new())
}

// Tests for PreAlloc implementation
#[test]
fn prealloc_invariants() -> Result {
  let mut buf = [0u64; 100];
  test_invariants(PreAlloc::new(&mut buf[..]))
}

#[test]
fn prealloc_mutability() -> Result {
  let mut buf = [0u32; 100];
  test_mutability(PreAlloc::new(&mut buf[..]))
}

#[test]
fn prealloc_edge_cases() -> Result {
  let mut buf = [0u8; 100];
  test_edge_cases(PreAlloc::new(&mut buf[..]))
}

// Test PreAlloc overflow behavior
#[test]
fn prealloc_overgrow() {
  let mut buf = [0u64; 10];
  let mut mem = PreAlloc::new(&mut buf[..]);

  // Should succeed
  mem.grow(10).unwrap().zeroed();
  assert_eq!(mem.as_slice().len(), 10);

  // Should fail - no space left
  assert!(mem.grow(1).is_err());
}

// Test capacity overflow
#[test]
fn test_capacity_overflow() {
  let mut alloc = Alloc::<u64>::new();
  assert!(alloc.grow(usize::MAX).is_err());
}

// Test interleaved operations
#[test]
fn test_interleaved_ops() -> Result {
  let mut alloc = Alloc::<i64>::new();

  alloc.grow(5)?.filled(1);
  alloc.grow(3)?.filled(2);
  alloc.shrink(2)?;
  alloc.grow(4)?.filled(3);

  assert_eq!(alloc.as_slice().len(), 10);
  assert_eq!(&alloc.as_slice()[0..5], &[1; 5]);
  assert_eq!(&alloc.as_slice()[5..6], &[2; 1]);
  assert_eq!(&alloc.as_slice()[6..10], &[3; 4]);

  Ok(())
}

// Test zeroed initialization
#[test]
fn test_zeroed_initialization() -> Result {
  let mut alloc = Alloc::<[u8; 16]>::new();

  let data = alloc.grow(100)?.zeroed();
  assert_eq!(data.len(), 100);
  for chunk in data {
    assert_eq!(chunk, &[0u8; 16]);
  }

  Ok(())
}

// Test filled initialization with Clone type
#[test]
fn test_filled_initialization() -> Result {
  let mut alloc = Alloc::<i32>::new();

  let data = alloc.grow(50)?.filled(-42);
  assert_eq!(data.len(), 50);
  assert!(data.iter().all(|&x| x == -42));

  Ok(())
}
