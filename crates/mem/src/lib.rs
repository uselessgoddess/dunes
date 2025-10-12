// at your own risk, at your own risk, but right now there is no normal way
// to write single-line unsafe functions
#![allow(unsafe_op_in_unsafe_fn, clippy::missing_transmute_annotations)]
extern crate core;

#[cfg(feature = "memmap")]
mod file;
mod place;
mod pre;
mod raw;
mod uninit;

pub(crate) use place::RawPlace;
pub use {
  pre::PreAlloc,
  raw::{Error, Page, RawMem},
};

mod utils {
  use {crate::place::RawPlace, std::fmt};

  pub fn debug_mem<'a, 'b: 'a, T>(
    f: &'a mut fmt::Formatter<'b>,
    buf: &RawPlace<T>,
    alt: &str,
  ) -> Result<fmt::DebugStruct<'a, 'b>, fmt::Error> {
    write!(f, "{:?} ", buf)?;
    Ok(f.debug_struct(alt))
  }
}

#[cfg(feature = "memmap")]
pub use file::FileMapped;

/// Alias for `Result<T, Error>` to return from `RawMem` methods
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! memory {
  ($($name:ident<$param:ident>($inner:ty) { $($body:tt)* } )*) => {$(
    pub struct $name<$param>($inner);

    impl<$param> $name<$param> {
      $($body)*
    }

    const _: () = {
      use std::{
        mem::MaybeUninit,
        fmt::{self, Formatter},
      };

      impl<$param: bytemuck::Pod> RawMem for $name<$param> {
        type Item = $param;

        fn as_slice(&self) -> &[Self::Item] {
          self.0.as_slice()
        }

        fn as_mut_slice(&mut self) -> &mut [Self::Item] {
          self.0.as_mut_slice()
        }

        fn grow(&mut self, cap: usize) -> Result<Page<'_, Self::Item>> {
          self.0.grow(cap)
        }

        fn shrink(&mut self, cap: usize) -> Result<()> {
          self.0.shrink(cap)
        }
      }

      impl<T> fmt::Debug for $name<$param> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
          f.debug_tuple(stringify!($name)).field(&self.0).finish()
        }
      }
    };
  )*};
}

#[cfg(feature = "tempfile")]
use std::{fs::File, io, path::Path};

#[cfg(feature = "tempfile")]
memory! {
   TempFile<T>(FileMapped<T>) {
       pub fn new() -> io::Result<Self> {
           Self::from_temp(tempfile::tempfile())
       }

       pub fn new_in<P: AsRef<Path>>(path: P) -> io::Result<Self> {
           Self::from_temp(tempfile::tempfile_in(path))
       }

       fn from_temp(file: io::Result<File>) -> io::Result<Self> {
           file.and_then(FileMapped::new).map(Self)
       }
   }
}
