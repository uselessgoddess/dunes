use {
  crate::{Error, Page, RawMem, Result, place::RawPlace},
  bytemuck::Pod,
  std::{
    alloc::{self, Layout},
    mem::MaybeUninit,
    slice,
  },
};

pub struct Alloc<T> {
  place: RawPlace<T>,
  cap: usize,
}

impl<T> Alloc<T> {
  pub const fn new() -> Self {
    Self { place: RawPlace::dangling(), cap: 0 }
  }

  pub fn capacity(&self) -> usize {
    self.cap
  }

  pub fn len(&self) -> usize {
    self.place.len()
  }

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
    let layout =
      Layout::array::<T>(new_cap).map_err(|_| Error::CapacityOverflow)?;

    let ptr = if old_cap == 0 {
      // SAFETY: layout has non-zero size since new_cap > 0
      let ptr = unsafe { alloc::alloc(layout) };
      if ptr.is_null() {
        return Err(Error::AllocError { layout, non_exhaustive: () });
      }
      ptr
    } else {
      let old_layout =
        Layout::array::<T>(old_cap).map_err(|_| Error::CapacityOverflow)?;

      // SAFETY: reallocating with matching old_layout and growing to larger size
      let ptr = unsafe {
        let old_ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
        alloc::realloc(old_ptr, old_layout, layout.size())
      };

      if ptr.is_null() {
        return Err(Error::AllocError { layout, non_exhaustive: () });
      }
      ptr
    };

    self.cap = new_cap;

    // SAFETY: ptr is valid for new_cap elements
    let uninit: &mut [MaybeUninit<T>] =
      unsafe { slice::from_raw_parts_mut(ptr as *mut MaybeUninit<T>, new_cap) };

    Ok(self.place.grow(uninit))
  }

  fn shrink(&mut self, reduction: usize) -> Result<()> {
    let new_cap = self.cap.saturating_sub(reduction);

    if new_cap == 0 {
      // Deallocate everything
      if self.cap > 0 {
        let layout =
          Layout::array::<T>(self.cap).map_err(|_| Error::CapacityOverflow)?;

        // SAFETY: deallocating with matching layout before resetting to dangling
        unsafe {
          let ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
          alloc::dealloc(ptr, layout);
        }
      }
      self.cap = 0;
      self.place = RawPlace::dangling();
      return Ok(());
    }

    let old_layout =
      Layout::array::<T>(self.cap).map_err(|_| Error::CapacityOverflow)?;
    let new_layout =
      Layout::array::<T>(new_cap).map_err(|_| Error::CapacityOverflow)?;

    // SAFETY: reallocating with matching old_layout and shrinking to smaller size
    let ptr = unsafe {
      let old_ptr = self.place.as_mut_slice().as_mut_ptr() as *mut u8;
      alloc::realloc(old_ptr, old_layout, new_layout.size())
    };

    if ptr.is_null() {
      return Err(Error::AllocError { layout: new_layout, non_exhaustive: () });
    }

    self.cap = new_cap;
    self.place.shrink_to(new_cap);

    Ok(())
  }
}

impl<T> Drop for Alloc<T> {
  fn drop(&mut self) {
    if self.cap > 0 {
      if let Ok(layout) = Layout::array::<T>(self.cap) {
        // SAFETY: deallocating with matching layout during final cleanup
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
