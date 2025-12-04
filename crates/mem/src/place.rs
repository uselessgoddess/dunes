use std::{
  fmt,
  mem::{self, MaybeUninit},
  ptr::NonNull,
  slice,
};

use crate::{Page, uninit};

/// `RawVec` alternative to easy `RawMem` implementation
pub struct RawPlace<T> {
  ptr: NonNull<[T]>,
  /// Initialized part length of pointer data
  len: usize,
}

impl<T> RawPlace<T> {
  pub const fn dangling() -> Self {
    Self { ptr: NonNull::slice_from_raw_parts(NonNull::dangling(), 0), len: 0 }
  }

  pub unsafe fn as_slice(&self) -> &[T] {
    slice::from_raw_parts(self.ptr.as_ptr().cast(), self.len)
  }

  pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
    slice::from_raw_parts_mut(self.ptr.as_ptr().cast(), self.len)
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn grow(&mut self, slice: &mut [MaybeUninit<T>]) -> Page<'_, T> {
    // SAFETY: `NonNull` is transparent for this conversion
    self.ptr = unsafe { mem::transmute::<_, NonNull<[T]>>(slice) };
    // SAFETY: pointer inspired from valid `slice` before
    Page {
      uninit: unsafe { &mut uninit::as_uninit_slice_mut(self.ptr)[self.len..] },
      len: Some(&mut self.len),
    }
  }

  #[allow(dead_code)]
  pub fn shrink_to(&mut self, cap: usize) {
    assert!(cap <= self.ptr.len());
    self.ptr = NonNull::slice_from_raw_parts(self.ptr.cast(), cap);
    self.len = self.len.min(cap);
  }

  /// Update the pointer after reallocation, preserving the initialized length
  pub fn update_ptr(&mut self, slice: &mut [MaybeUninit<T>]) {
    let new_cap = slice.len();
    // SAFETY: `NonNull` is transparent for this conversion
    self.ptr = unsafe { mem::transmute::<_, NonNull<[T]>>(slice) };
    self.len = self.len.min(new_cap);
  }
}

impl<T> fmt::Debug for RawPlace<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?}::{})", self.ptr, self.len)
  }
}

// pointer is unique
unsafe impl<T: Sync> Sync for RawPlace<T> {}
unsafe impl<T: Send> Send for RawPlace<T> {}

#[test]
fn zst_build() {
  let _: RawPlace<()> = RawPlace::dangling();
}
