use std::fmt::Debug;

mod miri;
mod pre;

macro_rules! define_impls {
  (impl RawMem: {
    $($ctor:expr /* -- */ $(=> in $cfg:meta)? ),+ $(,)?
  } for [
    $($test:path as $name:ident),* $(,)?
  ]) => {
    define_impls! { @loop
      [/* empty result */]
      [ $($ctor $(=> $cfg)? )*]
      [ $($test as $name |)* ]
    }
  };

  (@loop [ $($result:tt)* ] // result accumulation
          [ $($ctor:expr $(=> $cfg:meta)? )* ] // each ctor with our cfg `not(miri)`
          [ $test:path as $name:ident | $($tail:tt)* ] // match test with name + tail
  ) => {
    define_impls! { @loop
      [
        $($result)*

        #[test]
        fn $name() {
            $( $(#[cfg($cfg)])? Report::result($test($ctor));)*
        }
      ]
      [$($ctor $(=> $cfg)? )*]
      [ $($tail)* ]
    }
  };

  (@loop [ $($result:tt)* ] [ $($_:tt)* ] [ /* tests still coming */ ] ) => {
    $($result)*
  };
}

trait Report {
  fn result(me: Self);
}

impl Report for () {
  fn result(_: Self) {}
}

impl<T, E: Debug> Report for Result<T, E> {
  fn result(me: Self) {
    me.unwrap();
  }
}

define_impls! {
    impl RawMem: {
        mem::Alloc::new(),
        mem::TempFile::new().unwrap() => in all(feature = "tempfile", not(miri)),
    } for [
        miri::miri as miri,
    ]
}
