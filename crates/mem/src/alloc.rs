use {
  crate::{place::RawPlace, Error, Page, RawMem, Result},
  bytemuck::Pod,
  std::{
    alloc::{self, Layout},
    mem::MaybeUninit,
    slice,
  },
};

/// A memory allocator using stable Rust's allocation API
///
/// This implementation provides dynamic memory allocation using
/// `std::alloc::{alloc, realloc, dealloc}` which are stable APIs.
///
/// # Examples
///
/// ```
/// # use mem::{Alloc, RawMem};
/// let mut alloc = Alloc::<u64>::new();
/// let page = alloc.grow(10).unwrap();
/// let data = page.zeroed();
/// assert_eq!(data.len(), 10);
/// assert_eq!(data, &[0u64; 10]);
/// ```
pub struct Alloc<T> {
  place: RawPlace<T>,
  cap: usize,
}

impl<T> Alloc<T> {
  /// Creates a new empty allocator
  pub const fn new() -> Self {
    Self { place: RawPlace::dangling(), cap: 0 }
  }

  /// Returns the current capacity
  pub fn capacity(&self) -> usize {
    self.cap
  }

  /// Returns the current length (number of initialized elements)
  pub fn len(&self) -> usize {
    self.place.len()
  }

  /// Returns true if the allocator has no initialized elements
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl<T> Default for Alloc<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Pod> RawMem for Alloc<T> {
  type Item = T;

  fn as_slice(&self) -> &[Self::Item] {
    // SAFETY: RawPlace guarantees valid slice for initialized elements
    unsafe { self.place.as_slice() }
  }

  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    // SAFETY: RawPlace guarantees valid slice for initialized elements
    unsafe { self.place.as_mut_slice() }
  }

  fn grow(&mut self, addition: usize) -> Result<Page<'_, Self::Item>> {
    let old_cap = self.cap;
    let new_cap =
      self.cap.checked_add(addition).ok_or(Error::CapacityOverflow)?;

    // Check if new capacity exceeds layout limits
    let layout = Layout::array::<T>(new_cap).map_err(|_| Error::CapacityOverflow)?;

    let ptr = if old_cap == 0 {
      // Initial allocation
      // SAFETY: layout has non-zero size (new_cap > 0)
      let ptr = unsafe { alloc::alloc(layout) };
      if ptr.is_null() {
        return Err(Error::AllocError {
          layout,
          non_exhaustive: (),
        });
      }
      ptr
    } else {
      // Reallocation
      let old_layout = Layout::array::<T>(old_cap)
        .map_err(|_| Error::CapacityOverflow)?;

      // SAFETY:
      // - self.place.ptr points to currently allocated memory
      // - old_layout matches the previous allocation
      // - new_layout.size() >= old_layout.size()
      let ptr = unsafe {
        let old_ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
        alloc::realloc(old_ptr, old_layout, layout.size())
      };

      if ptr.is_null() {
        return Err(Error::AllocError {
          layout,
          non_exhaustive: (),
        });
      }
      ptr
    };

    self.cap = new_cap;

    // SAFETY: ptr is valid for new_cap elements
    let uninit: &mut [MaybeUninit<T>] = unsafe {
      slice::from_raw_parts_mut(ptr as *mut MaybeUninit<T>, new_cap)
    };

    Ok(self.place.grow(uninit))
  }

  fn shrink(&mut self, reduction: usize) -> Result<()> {
    let new_cap = self.cap.saturating_sub(reduction);

    if new_cap == 0 {
      // Deallocate everything
      if self.cap > 0 {
        let layout = Layout::array::<T>(self.cap)
          .map_err(|_| Error::CapacityOverflow)?;

        // SAFETY:
        // - ptr was allocated with this layout
        // - we're about to set cap to 0
        unsafe {
          let ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
          alloc::dealloc(ptr, layout);
        }
      }
      self.cap = 0;
      self.place = RawPlace::dangling();
      return Ok(());
    }

    // Shrink to new capacity
    let old_layout = Layout::array::<T>(self.cap)
      .map_err(|_| Error::CapacityOverflow)?;
    let new_layout = Layout::array::<T>(new_cap)
      .map_err(|_| Error::CapacityOverflow)?;

    // SAFETY:
    // - ptr was allocated with old_layout
    // - new_layout.size() <= old_layout.size()
    let ptr = unsafe {
      let old_ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
      alloc::realloc(old_ptr, old_layout, new_layout.size())
    };

    if ptr.is_null() {
      return Err(Error::AllocError {
        layout: new_layout,
        non_exhaustive: (),
      });
    }

    self.cap = new_cap;
    self.place.shrink_to(new_cap);

    Ok(())
  }
}

impl<T> Drop for Alloc<T> {
  fn drop(&mut self) {
    if self.cap > 0 {
      // SAFETY: We have a valid layout for the current capacity
      if let Ok(layout) = Layout::array::<T>(self.cap) {
        // SAFETY:
        // - ptr was allocated with this layout
        // - we're in Drop so this is the final cleanup
        unsafe {
          let ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
          alloc::dealloc(ptr, layout);
        }
      }
    }
  }
}

use std::fmt::{self, Formatter};

impl<T> fmt::Debug for Alloc<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    crate::utils::debug_mem(f, &self.place, "Alloc")?
      .field("cap", &self.cap)
      .finish()
  }
}

// SAFETY: Alloc owns its data and can be sent between threads
unsafe impl<T: Send> Send for Alloc<T> {}
// SAFETY: Alloc provides exclusive access to its data
unsafe impl<T: Sync> Sync for Alloc<T> {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let alloc = Alloc::<u64>::new();
    assert_eq!(alloc.len(), 0);
    assert_eq!(alloc.capacity(), 0);
    assert!(alloc.is_empty());
  }

  #[test]
  fn test_grow_and_shrink() -> Result<()> {
    let mut alloc = Alloc::<u64>::new();

    // Grow
    alloc.grow(10)?.zeroed();
    assert_eq!(alloc.len(), 10);
    assert_eq!(alloc.capacity(), 10);

    // Shrink
    alloc.shrink(5)?;
    assert_eq!(alloc.capacity(), 5);

    Ok(())
  }

  #[test]
  fn test_zeroed() -> Result<()> {
    let mut alloc = Alloc::<u64>::new();
    let data = alloc.grow(5)?.zeroed();
    assert_eq!(data, &[0u64; 5]);
    Ok(())
  }

  #[test]
  fn test_filled() -> Result<()> {
    let mut alloc = Alloc::<i32>::new();
    let data = alloc.grow(3)?.filled(42);
    assert_eq!(data, &[42, 42, 42]);
    Ok(())
  }
}
