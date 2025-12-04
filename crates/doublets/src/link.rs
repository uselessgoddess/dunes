use core::{
  fmt::{self, Debug, Formatter},
  num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
  },
};

/// Trait for types that can be used as link identifiers
///
/// This trait is implemented for primitives (usize, u64, etc.)
/// The Repr type is used for memory-efficient tree storage using
/// NonZero variants.
pub trait Index:
  Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Debug + Send + Sync
{
  /// The representation type for tree storage (typically NonZero variant)
  type Repr: Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Debug
    + Send
    + Sync;

  /// Special constant values known at compile time
  const ZERO: Self;
  const ANY: Self;
  const ONE: Self;

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

  /// Convert to Repr (NonZero representation)
  fn to_repr(self) -> Option<Self::Repr>;

  /// Convert from Repr (NonZero representation)
  fn from_repr(repr: Self::Repr) -> Self;
}

macro_rules! impl_index {
  ($prim:ty, $nonzero:ty) => {
    impl Index for $prim {
      type Repr = $nonzero;

      const ZERO: Self = 0;
      const ANY: Self = 0;
      const ONE: Self = 1;

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

      #[inline]
      fn to_repr(self) -> Option<Self::Repr> {
        <$nonzero>::new(self)
      }

      #[inline]
      fn from_repr(repr: Self::Repr) -> Self {
        repr.get()
      }
    }
  };
}

impl_index!(usize, NonZeroUsize);
impl_index!(u8, NonZeroU8);
impl_index!(u16, NonZeroU16);
impl_index!(u32, NonZeroU32);
impl_index!(u64, NonZeroU64);
impl_index!(u128, NonZeroU128);
impl_index!(isize, NonZeroIsize);
impl_index!(i8, NonZeroI8);
impl_index!(i16, NonZeroI16);
impl_index!(i32, NonZeroI32);
impl_index!(i64, NonZeroI64);
impl_index!(i128, NonZeroI128);

/// Macro to reserve a constant range for compile-time link constant checking
///
/// This macro generates const assertions to ensure that specific indices
/// are reserved for constant links. Use this to define semantic constants
/// that have special meaning in your application.
///
/// # Example
/// ```ignore
/// reserve_constants! {
///   const NULL = 0;
///   const MEANING = 1;
///   const OF = 2;
/// }
/// ```
#[macro_export]
macro_rules! reserve_constants {
  ($(const $name:ident = $value:expr;)*) => {
    $(
      pub const $name: usize = $value;

      // Compile-time assertion that constants are in valid range
      const _: () = {
        if $value == 0 {
          panic!("Reserved constant cannot be zero (reserved for ANY)");
        }
      };
    )*

    // Find the maximum constant value to determine reserved range
    pub const MAX_RESERVED: usize = {
      let mut max = 0;
      $(
        if $value > max {
          max = $value;
        }
      )*
      max
    };
  };
}

/// Represents a link/edge in the doublets database
///
/// A link has an index (identifier), source, and target.
/// All three components use the same type T which implements Index.
#[derive(Default, Eq, PartialEq, Clone, Hash, Copy)]
#[repr(C)]
pub struct Link<T: Index> {
  pub index: T,
  pub source: T,
  pub target: T,
}

impl<T: Index> Link<T> {
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
  pub const fn nothing() -> Self {
    Self::new(T::ZERO, T::ZERO, T::ZERO)
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

impl<T: Index> Debug for Link<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}: {:?} {:?}", self.index, self.source, self.target)
  }
}

unsafe impl<T: Index> bytemuck::Pod for Link<T> where T: bytemuck::Pod {}
#[rustfmt::skip]
unsafe impl<T: Index> bytemuck::Zeroable for Link<T>
where T: bytemuck::Zeroable {}
