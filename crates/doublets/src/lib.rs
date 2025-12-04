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
  store::{Store, create_heap_store},
  traits::{Doublets, Links},
};
