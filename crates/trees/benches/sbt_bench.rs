use {
  criterion::{
    Bencher, BenchmarkId, Criterion, Throughput, criterion_group,
    criterion_main,
  },
  dunes_trees::{SizeBalancedTree, Store},
  std::{
    num::NonZeroUsize,
    time::{Duration, Instant},
  },
};

fn bench_insert<T: dunes_trees::Idx>(
  b: &mut Bencher,
  n: usize,
  from: impl Fn(usize) -> T,
) {
  let mut store = Store::<T>::new(n);
  b.iter_custom(|iters| {
    let mut elapsed = Duration::ZERO;
    for _ in 0..iters {
      let instant = Instant::now();
      let mut root = None;
      for i in 1..n {
        root = store.insert(root, from(i));
      }
      elapsed += instant.elapsed();
      store.reset();
    }
    elapsed
  });
}

fn bench_insert_and_search<T: dunes_trees::Idx>(
  b: &mut Bencher,
  n: usize,
  from: impl Fn(usize) -> T,
) {
  let mut store = Store::<T>::new(n);
  b.iter_custom(|iters| {
    let mut elapsed = Duration::ZERO;
    for _ in 0..iters {
      let instant = Instant::now();
      let mut root = None;
      for i in 1..n {
        root = store.insert(root, from(i));
      }
      for i in 1..n {
        assert!(store.contains(root.unwrap(), from(i)));
      }
      elapsed += instant.elapsed();
      store.reset();
    }
    elapsed
  });
}

fn bench_insert_remove<T: dunes_trees::Idx>(
  b: &mut Bencher,
  n: usize,
  from: impl Fn(usize) -> T,
) {
  let mut store = Store::<T>::new(n);
  b.iter_custom(|iters| {
    let mut elapsed = Duration::ZERO;
    for _ in 0..iters {
      let instant = Instant::now();
      let mut root = None;
      for i in 1..n {
        root = store.insert(root, from(i));
      }
      for i in 1..n {
        root = store.remove(root, from(i));
      }
      elapsed += instant.elapsed();
      store.reset();
    }
    elapsed
  });
}

fn benchmark_trees(c: &mut Criterion) {
  let sizes = [100, 1_000, 10_000];

  for &n in &sizes {
    let mut group = c.benchmark_group(format!("sbt_insert_{}", n));
    group.throughput(Throughput::Elements(n as u64));

    group.bench_with_input(BenchmarkId::new("usize", n), &n, |b, &n| {
      bench_insert(b, n, |x| x)
    });

    group.bench_with_input(
      BenchmarkId::new("nonzero", n),
      &n,
      |b, &n| unsafe { bench_insert(b, n, |x| NonZeroUsize::new_unchecked(x)) },
    );

    group.finish();
  }

  for &n in &sizes {
    let mut group = c.benchmark_group(format!("sbt_insert_search_{}", n));
    group.throughput(Throughput::Elements(n as u64 * 2));

    group.bench_with_input(BenchmarkId::new("usize", n), &n, |b, &n| {
      bench_insert_and_search(b, n, |x| x)
    });

    group.finish();
  }

  for &n in &[100, 1_000] {
    let mut group = c.benchmark_group(format!("sbt_full_cycle_{}", n));
    group.throughput(Throughput::Elements(n as u64 * 2));

    group.bench_with_input(BenchmarkId::new("usize", n), &n, |b, &n| {
      bench_insert_remove(b, n, |x| x)
    });

    group.finish();
  }
}

criterion_group!(benches, benchmark_trees);
criterion_main!(benches);
