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
  S: BenchStore<T> + trees::SizeBalanced<T>,
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
  S: BenchStore<T> + trees::SizeBalanced<T>,
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
  S: BenchStore<T> + trees::SizeBalanced<T>,
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

// Generic benchmark registration to avoid repetition
fn register_benchmarks<S, T>(
  c: &mut Criterion,
  tree_name: &str,
  sizes: &[usize],
) where
  S: BenchStore<T> + trees::SizeBalanced<T> + 'static,
  T: Idx + From<usize> + 'static,
{
  // Insert-only benchmarks
  for &size in sizes {
    c.bench_function(&format!("{}::insert({})", tree_name, size), |b| {
      b.iter(bench_insert_impl::<S, T>(size))
    });
  }

  // Insert + search benchmarks
  for &size in sizes {
    c.bench_function(&format!("{}::insert_search({})", tree_name, size), |b| {
      b.iter(bench_insert_and_search_impl::<S, T>(size))
    });
  }

  // Full cycle (insert + remove) benchmarks
  for &size in sizes {
    c.bench_function(&format!("{}::full_cycle({})", tree_name, size), |b| {
      b.iter(bench_insert_remove_impl::<S, T>(size))
    });
  }
}

fn sbt_benchmarks(c: &mut Criterion) {
  const SIZES: &[usize] = &[100, 1_000, 10_000];
  register_benchmarks::<Store<usize>, usize>(c, "sbt", SIZES);
}

fn art_benchmarks(c: &mut Criterion) {
  const SIZES: &[usize] = &[100, 1_000, 10_000];
  register_benchmarks::<common::ArtStore<usize>, usize>(c, "art", SIZES);
}

criterion_group!(benches, sbt_benchmarks, art_benchmarks);
criterion_main!(benches);
