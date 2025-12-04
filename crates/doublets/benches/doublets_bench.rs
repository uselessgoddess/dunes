use {
  criterion::{Criterion, criterion_group, criterion_main},
  doublets::{Doublets, create_heap_store},
  std::hint::black_box,
};

fn bench_create_point(c: &mut Criterion) {
  c.bench_function("create_point", |b| {
    b.iter(|| {
      let mut store = create_heap_store::<usize>().unwrap();
      for _ in 0..100 {
        black_box(store.create_point().unwrap());
      }
    });
  });
}

fn bench_create_link(c: &mut Criterion) {
  c.bench_function("create_link", |b| {
    b.iter(|| {
      let mut store = create_heap_store::<usize>().unwrap();
      let a = store.create_point().unwrap();
      let b = store.create_point().unwrap();
      for _ in 0..100 {
        black_box(store.create_link(a, b).unwrap());
      }
    });
  });
}

fn bench_search(c: &mut Criterion) {
  c.bench_function("search", |b| {
    let mut store = create_heap_store::<usize>().unwrap();
    let a = store.create_point().unwrap();
    let target = store.create_point().unwrap();
    store.create_link(a, target).unwrap();

    b.iter(|| {
      black_box(store.search(a, target));
    });
  });
}

fn bench_iteration(c: &mut Criterion) {
  c.bench_function("iterate_all", |b| {
    let mut store = create_heap_store::<usize>().unwrap();
    for _ in 0..100 {
      store.create_point().unwrap();
    }

    b.iter(|| {
      let links: Vec<_> = store.iter().collect();
      black_box(links);
    });
  });
}

fn bench_create_million_points(c: &mut Criterion) {
  c.bench_function("create_million_points", |b| {
    b.iter(|| {
      let mut store = create_heap_store::<usize>().unwrap();
      for _ in 0..1_000_000 {
        black_box(store.create_point().unwrap());
      }
    });
  });
}

criterion_group!(
  benches,
  bench_create_point,
  bench_create_link,
  bench_search,
  bench_iteration,
  bench_create_million_points
);
criterion_main!(benches);
