use {crate::Index, core::fmt::Debug, thiserror::Error};
/// Errors that can occur during doublets operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error<T: Index> {
  #[error("Link {0:?} does not exist")]
  NotExists(T),
  #[error("Link {0:?} already exists with source {1:?} and target {2:?}")]
  AlreadyExists(T, T, T),
  #[error("Link {0:?} has usages and cannot be deleted")]
  HasUsages(T),
  #[error("Memory allocation failed")]
  AllocationFailed,
  #[error("Operation would overflow capacity")]
  Overflow,
  #[error("Invalid query parameters")]
  InvalidQuery,
}
pub type Result<R, T> = core::result::Result<R, Error<T>>;
