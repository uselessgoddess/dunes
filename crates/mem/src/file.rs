use {
  crate::{Error::CapacityOverflow, Page, RawMem, Result},
  memmap2::{MmapMut, MmapOptions},
  std::{
    alloc::Layout,
    fmt::{self, Formatter},
    fs::{File, OpenOptions},
    io,
    mem::MaybeUninit,
    path::Path,
    ptr::NonNull,
    slice,
  },
};

pub struct FileMapped<T> {
  file: File,
  map: Option<MmapMut>,
  place: RawPlace<T>,
}

impl<T> FileMapped<T> {
  // todo: say about mapping, read-write guarantees, and `MIN_PAGE_SIZE`
  pub fn new(file: File) -> io::Result<Self> {
    const MIN_PAGE_SIZE: u64 = 8 * 1024;

    if file.metadata()?.len() < MIN_PAGE_SIZE {
      file.set_len(MIN_PAGE_SIZE)?;
    }

    Ok(Self { file, map: None, place: RawPlace::dangling() })
  }

  fn options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.create(true).read(true).write(true);
    options
  }

  pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
    Self::options().open(path).and_then(Self::new)
  }

  fn map_mut(&mut self, cap: u64) -> io::Result<MmapMut> {
    unsafe { MmapOptions::new().len(cap as usize).map_mut(&self.file) }
  }

  fn map_replace(&mut self, cap: u64) -> io::Result<&mut MmapMut> {
    let map = self.map_mut(cap)?;
    let _ = self.map.replace(map);
    Ok(self.map.as_mut().unwrap())
  }

  fn capacity(&self) -> Option<usize> {
    self.map.as_ref().map(|map| map.len() / size_of::<T>())
  }
}

use {crate::place::RawPlace, bytemuck::Pod};

impl<T: Pod> RawMem for FileMapped<T> {
  type Item = T;

  fn as_slice(&self) -> &[Self::Item] {
    unsafe { self.place.as_slice() }
  }

  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    unsafe { self.place.as_mut_slice() }
  }

  fn grow(&mut self, addition: usize) -> Result<Page<'_, Self::Item>> {
    // grow from initialized part that means `len`
    let cap = self.place.len().checked_add(addition).ok_or(CapacityOverflow)?;

    // use layout to prevent all capacity bugs
    let layout = Layout::array::<T>(cap).map_err(|_| CapacityOverflow)?;

    // unmap the file by dropping
    let _ = self.map.take();

    let new = layout.size() as u64;
    let old = self.file.metadata()?.len();

    if new > old {
      self.file.set_len(new)?;
    }

    let ptr = NonNull::from(self.map_replace(new)?.as_mut());
    // SAFETY: provide valid lifetime inferred from inner `buf`
    let uninit: &mut [MaybeUninit<T>] =
      unsafe { slice::from_raw_parts_mut(ptr.cast().as_ptr(), cap) };
    Ok(self.place.grow(uninit))
  }

  fn shrink(&mut self, shrink: usize) -> Result<()> {
    let Some(cap) = self.capacity() else {
      return Ok(());
    };
    let cap = cap.saturating_sub(shrink);

    let _ = self.map.take();
    // SAFETY: avoid checked mul because memory layout is valid
    //  then smaller layout will also be valid
    let new = unsafe { size_of::<T>().unchecked_mul(cap) as u64 };

    self.file.set_len(new)?;

    let _ = self.map_replace(new)?;
    self.place.shrink_to(cap);

    Ok(())
  }
}

impl<T> Drop for FileMapped<T> {
  fn drop(&mut self) {
    let _ = self.file.sync_all();
  }
}

impl<T> fmt::Debug for FileMapped<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("FileMapped")
      .field("mmap", &self.map)
      .field("file", &self.file)
      .finish()
  }
}
