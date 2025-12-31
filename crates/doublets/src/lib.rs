#![doc = include_str!("../README.md")]

mod error;
mod handler;
mod link;
mod store;
mod traits;

pub use {
  error::{Error, Result},
  handler::{Flow, IntoFlow, ReadHandler, WriteHandler},
  link::{Index, Link},
  store::{
    ArtStrategy, RawLink, SbtStrategy, Store, TreeStrategy, create_heap_store,
  },
  traits::{Doublets, Links},
};
