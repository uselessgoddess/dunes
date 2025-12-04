use core::{
  fmt::{self, Debug, Formatter},
  num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
  },
};

/// Trait for types that can be used as link identifiers
///
/// This trait is implemented for both regular primitives (usize, u64, etc.)
/// and NonZero primitives (NonZeroUsize, NonZeroU64, etc.)
pub trait LinkIndex:
  Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Debug + Send + Sync
{
  /// The zero value for this type
  fn zero() -> Self;

  /// Check if the value is zero
  fn is_zero(&self) -> bool;

  /// Convert from usize
  fn from_usize(val: usize) -> Self;

  /// Convert to usize
  fn as_usize(&self) -> usize;

  /// Try to increment the value by one
  fn checked_add_one(&self) -> Option<Self>;

  /// Try to decrement the value by one
  fn checked_sub_one(&self) -> Option<Self>;
}

macro_rules! impl_link_index {
  ($($t:ty),*) => {
    $(
      impl LinkIndex for $t {
        #[inline]
        fn zero() -> Self {
          0
        }

        #[inline]
        fn is_zero(&self) -> bool {
          *self == 0
        }

        #[inline]
        fn from_usize(val: usize) -> Self {
          val as Self
        }

        #[inline]
        fn as_usize(&self) -> usize {
          *self as usize
        }

        #[inline]
        fn checked_add_one(&self) -> Option<Self> {
          self.checked_add(1)
        }

        #[inline]
        fn checked_sub_one(&self) -> Option<Self> {
          self.checked_sub(1)
        }
      }
    )*
  };
}

impl_link_index!(
  usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
);

macro_rules! impl_link_index_nonzero {
  ($($nz:ty, $inner:ty),*) => {
    $(
      impl LinkIndex for $nz {
        #[inline]
        fn zero() -> Self {
          // NonZero types can't represent zero, so we use 1 as the
          // "zero" sentinel. This is safe because we're treating 1
          // as the starting index
          unsafe { Self::new_unchecked(1) }
        }

        #[inline]
        fn is_zero(&self) -> bool {
          self.get() == 1
        }

        #[inline]
        fn from_usize(val: usize) -> Self {
          // Map 0 -> 1, 1 -> 2, etc.
          unsafe { Self::new_unchecked((val as $inner) + 1) }
        }

        #[inline]
        fn as_usize(&self) -> usize {
          // Map 1 -> 0, 2 -> 1, etc.
          (self.get() - 1) as usize
        }

        #[inline]
        fn checked_add_one(&self) -> Option<Self> {
          Self::new(self.get().checked_add(1)?)
        }

        #[inline]
        fn checked_sub_one(&self) -> Option<Self> {
          if self.get() <= 1 {
            None
          } else {
            Self::new(self.get() - 1)
          }
        }
      }
    )*
  };
}

impl_link_index_nonzero!(
  NonZeroUsize,
  usize,
  NonZeroU8,
  u8,
  NonZeroU16,
  u16,
  NonZeroU32,
  u32,
  NonZeroU64,
  u64,
  NonZeroU128,
  u128,
  NonZeroIsize,
  isize,
  NonZeroI8,
  i8,
  NonZeroI16,
  i16,
  NonZeroI32,
  i32,
  NonZeroI64,
  i64,
  NonZeroI128,
  i128
);

/// Represents a link/edge in the doublets database
///
/// A link has an index (identifier), source, and target.
/// All three components use the same type T which implements LinkIndex.
#[derive(Default, Eq, PartialEq, Clone, Hash, Copy)]
#[repr(C)]
pub struct Link<T: LinkIndex> {
  pub index: T,
  pub source: T,
  pub target: T,
}

impl<T: LinkIndex> Link<T> {
  /// Create a new link
  #[inline]
  #[must_use]
  pub const fn new(index: T, source: T, target: T) -> Self {
    Self { index, source, target }
  }

  /// Create a "point" link where all components are the same
  #[inline]
  #[must_use]
  pub const fn point(val: T) -> Self {
    Self::new(val, val, val)
  }

  /// Create a null/default link
  #[inline]
  #[must_use]
  pub fn nothing() -> Self {
    Self::new(T::zero(), T::zero(), T::zero())
  }

  /// Check if this is a null link
  #[inline]
  #[must_use]
  pub fn is_null(&self) -> bool {
    self.index.is_zero() && self.source.is_zero() && self.target.is_zero()
  }

  /// Check if this is a "full" point (all components equal)
  #[inline]
  #[must_use]
  pub fn is_full(&self) -> bool {
    self.index == self.source && self.index == self.target
  }

  /// Check if this is a "partial" point (index equals source OR target)
  #[inline]
  #[must_use]
  pub fn is_partial(&self) -> bool {
    self.index == self.source || self.index == self.target
  }
}

impl<T: LinkIndex> Debug for Link<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}: {:?} {:?}", self.index, self.source, self.target)
  }
}

unsafe impl<T: LinkIndex> bytemuck::Pod for Link<T> where T: bytemuck::Pod {}
unsafe impl<T: LinkIndex> bytemuck::Zeroable for Link<T> where
  T: bytemuck::Zeroable
{
}
