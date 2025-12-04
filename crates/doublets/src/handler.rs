use crate::{Link, LinkIndex};

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

/// Handler function for read operations (iteration)
///
/// Called for each link during iteration. Returns Flow to control
/// whether to continue.
pub type ReadHandler<'a, T> = &'a mut dyn FnMut(Link<T>) -> Flow;

/// Handler function for write operations (create, update, delete)
///
/// Called with before and after states. Returns Flow to control
/// whether to continue.
pub type WriteHandler<'a, T> = &'a mut dyn FnMut(Link<T>, Link<T>) -> Flow;

/// Constants for the doublets store
#[derive(Debug, Clone)]
pub struct Constants<T: LinkIndex> {
  /// Value used to match any link component in queries
  pub any: T,
  /// First valid internal link index
  pub internal_start: T,
  /// Last valid internal link index (exclusive upper bound)
  pub internal_end: T,
}

impl<T: LinkIndex> Constants<T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      any: T::zero(),
      internal_start: T::from_usize(1),
      internal_end: T::from_usize(capacity),
    }
  }

  /// Check if a value is the "any" sentinel
  #[inline]
  pub fn is_any(&self, value: T) -> bool {
    value == self.any
  }

  /// Check if a value is in the internal range
  #[inline]
  pub fn is_internal(&self, value: T) -> bool {
    value >= self.internal_start && value < self.internal_end
  }

  /// Check if a value is external (outside internal range, but not any)
  #[inline]
  pub fn is_external(&self, value: T) -> bool {
    !self.is_any(value) && !self.is_internal(value)
  }
}
