# Doublets

A modern Rust implementation of the Doublets associative database.

This crate provides an efficient in-memory graph database where data is stored as directed links (doublets) between nodes.

## Features

- Support for both regular and NonZero primitive types as link indices
- Efficient storage using size-balanced trees
- Modern, safe Rust implementation using the trees and mem crates
- No nightly features required
