mod common;

use {
  common::Store,
  criterion::{Criterion, black_box, criterion_group, criterion_main},
  trees::{Idx, Tree},
};

/// Trait for tree stores that can be reset and created
trait BenchStore<T: Idx>: Tree<T> {
  fn new(capacity: usize) -> Self;
  fn reset(&mut self);
}

impl<T: Idx> BenchStore<T> for Store<T> {
  fn new(capacity: usize) -> Self {
    Store::new(capacity)
  }

  fn reset(&mut self) {
    Store::reset(self)
  }
}

impl<T: Idx> BenchStore<T> for common::ArtStore<T> {
  fn new(capacity: usize) -> Self {
    common::ArtStore::new(capacity)
  }

  fn reset(&mut self) {
    common::ArtStore::reset(self)
  }
}

// Helper function for insert benchmarks
fn bench_insert_impl<S, T>(n: usize) -> impl FnMut()
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  move || {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    black_box(root);
    store.reset();
  }
}

// Helper function for insert and search benchmarks
fn bench_insert_and_search_impl<S, T>(n: usize) -> impl FnMut()
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  move || {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    for i in 1..n {
      black_box(store.contains(root.unwrap(), T::from(i)));
    }
    store.reset();
  }
}

// Helper function for full cycle benchmarks
fn bench_insert_remove_impl<S, T>(n: usize) -> impl FnMut()
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  move || {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    for i in 1..n {
      root = store.remove(root, T::from(i));
    }
    black_box(root);
    store.reset();
  }
}

fn sbt_benchmarks(c: &mut Criterion) {
  c.bench_function("sbt_insert_100", |b| {
    b.iter(bench_insert_impl::<Store<usize>, usize>(100))
  });

  c.bench_function("sbt_insert_1000", |b| {
    b.iter(bench_insert_impl::<Store<usize>, usize>(1_000))
  });

  c.bench_function("sbt_insert_10000", |b| {
    b.iter(bench_insert_impl::<Store<usize>, usize>(10_000))
  });

  c.bench_function("sbt_insert_search_100", |b| {
    b.iter(bench_insert_and_search_impl::<Store<usize>, usize>(100))
  });

  c.bench_function("sbt_insert_search_1000", |b| {
    b.iter(bench_insert_and_search_impl::<Store<usize>, usize>(1_000))
  });

  c.bench_function("sbt_insert_search_10000", |b| {
    b.iter(bench_insert_and_search_impl::<Store<usize>, usize>(10_000))
  });

  c.bench_function("sbt_full_cycle_100", |b| {
    b.iter(bench_insert_remove_impl::<Store<usize>, usize>(100))
  });

  c.bench_function("sbt_full_cycle_1000", |b| {
    b.iter(bench_insert_remove_impl::<Store<usize>, usize>(1_000))
  });
}

fn art_benchmarks(c: &mut Criterion) {
  c.bench_function("art_insert_100", |b| {
    b.iter(bench_insert_impl::<common::ArtStore<usize>, usize>(100))
  });

  c.bench_function("art_insert_1000", |b| {
    b.iter(bench_insert_impl::<common::ArtStore<usize>, usize>(1_000))
  });

  c.bench_function("art_insert_10000", |b| {
    b.iter(bench_insert_impl::<common::ArtStore<usize>, usize>(10_000))
  });

  c.bench_function("art_insert_search_100", |b| {
    b.iter(bench_insert_and_search_impl::<common::ArtStore<usize>, usize>(100))
  });

  c.bench_function("art_insert_search_1000", |b| {
    b.iter(bench_insert_and_search_impl::<common::ArtStore<usize>, usize>(
      1_000,
    ))
  });

  c.bench_function("art_insert_search_10000", |b| {
    b.iter(bench_insert_and_search_impl::<common::ArtStore<usize>, usize>(
      10_000,
    ))
  });

  c.bench_function("art_full_cycle_100", |b| {
    b.iter(bench_insert_remove_impl::<common::ArtStore<usize>, usize>(100))
  });

  c.bench_function("art_full_cycle_1000", |b| {
    b.iter(bench_insert_remove_impl::<common::ArtStore<usize>, usize>(1_000))
  });
}

criterion_group!(benches, sbt_benchmarks, art_benchmarks);
criterion_main!(benches);
