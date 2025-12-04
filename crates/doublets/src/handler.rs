use crate::{Index, Link};

/// Flow control for iteration handlers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
  /// Continue iterating
  Continue,
  /// Stop iterating
  Break,
}

impl From<bool> for Flow {
  fn from(value: bool) -> Self {
    if value { Flow::Continue } else { Flow::Break }
  }
}

/// Trait for types that can be converted to Flow
pub trait IntoFlow {
  fn into_flow(self) -> Flow;
}

impl IntoFlow for Flow {
  #[inline]
  fn into_flow(self) -> Flow {
    self
  }
}

impl IntoFlow for bool {
  #[inline]
  fn into_flow(self) -> Flow {
    Flow::from(self)
  }
}

/// Handler function for read operations (iteration)
///
/// Called for each link during iteration. Can return Flow or bool
/// to control whether to continue.
pub trait ReadHandler<T: Index> {
  fn handle(&mut self, link: Link<T>) -> Flow;
}

impl<T, F, R> ReadHandler<T> for F
where
  T: Index,
  F: FnMut(Link<T>) -> R,
  R: IntoFlow,
{
  fn handle(&mut self, link: Link<T>) -> Flow {
    self(link).into_flow()
  }
}

/// Handler function for write operations (create, update, delete)
///
/// Called with before and after states. Can return Flow or bool
/// to control whether to continue.
pub trait WriteHandler<T: Index> {
  fn handle(&mut self, before: Link<T>, after: Link<T>) -> Flow;
}

impl<T, F, R> WriteHandler<T> for F
where
  T: Index,
  F: FnMut(Link<T>, Link<T>) -> R,
  R: IntoFlow,
{
  fn handle(&mut self, before: Link<T>, after: Link<T>) -> Flow {
    self(before, after).into_flow()
  }
}
