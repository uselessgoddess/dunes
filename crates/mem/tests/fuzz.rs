use {
  mem::{Alloc, PreAlloc, RawMem},
  proptest::prelude::*,
};

#[derive(Debug, Clone)]
enum MemOp {
  Grow(usize),
  Shrink(usize),
}

fn mem_ops_strategy() -> impl Strategy<Value = Vec<MemOp>> {
  const MAX_OPS: usize = if cfg!(miri) { 20 } else { 100 };
  const MAX_SIZE: usize = if cfg!(miri) { 100 } else { 1000 };

  prop::collection::vec(
    prop_oneof![
      (1..=MAX_SIZE).prop_map(MemOp::Grow),
      (1..=MAX_SIZE).prop_map(MemOp::Shrink),
    ],
    1..=MAX_OPS,
  )
}

fn apply_ops<M: RawMem<Item = u64>>(mut mem: M, ops: Vec<MemOp>) {
  let mut expected_len: usize = 0;

  for op in ops {
    match op {
      MemOp::Grow(size) => {
        if let Ok(page) = mem.grow(size) {
          page.filled(42);
          expected_len += size;
          assert_eq!(mem.as_slice().len(), expected_len);
        }
      }
      MemOp::Shrink(size) => {
        mem.shrink(size).unwrap();
        expected_len = expected_len.saturating_sub(size);
        assert_eq!(mem.as_slice().len(), expected_len);
      }
    }
  }
}

proptest! {
  #[test]
  fn random_alloc_ops(ops in mem_ops_strategy()) {
    apply_ops(Alloc::<u64>::new(), ops);
  }

  #[test]
  fn random_prealloc_ops(ops in mem_ops_strategy()) {
    let mut buf = vec![0u64; 100000];
    apply_ops(PreAlloc::new(&mut buf[..]), ops);
  }

  #[test]
  fn grow_maintains_data(sizes in prop::collection::vec(1usize..100, 1..20)) {
    let mut alloc = Alloc::<i32>::new();
    let mut expected_data = Vec::new();

    for (idx, &size) in sizes.iter().enumerate() {
      alloc.grow(size).unwrap().filled(idx as i32);
      expected_data.extend(vec![idx as i32; size]);
    }

    assert_eq!(alloc.as_slice(), &expected_data[..]);
  }

  #[test]
  fn shrink_preserves_remaining(
    grow_size in 100usize..500,
    shrink_size in 1usize..100
  ) {
    let mut alloc = Alloc::<u8>::new();
    alloc.grow(grow_size).unwrap().filled(123);

    let remaining = grow_size.saturating_sub(shrink_size);
    alloc.shrink(shrink_size).unwrap();

    assert_eq!(alloc.as_slice().len(), remaining);
    assert!(alloc.as_slice().iter().all(|&x| x == 123));
  }
}

#[test]
fn capacity_overflow() {
  let mut alloc = Alloc::<u64>::new();
  assert!(alloc.grow(usize::MAX).is_err());
}
