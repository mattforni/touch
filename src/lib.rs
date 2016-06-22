// Copyright 2016 Matthew Fornaciari <mattforni@gmail.com>
//! A dead simple wrapper around file and directory manipulation.
//!
//! Any errors encountered during io will be returned wrapped in a `touch::Error`.
//!
//! That's it!
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/touch) and can be
//! used by adding `args` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! touch = "0"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate touch;
//! ```
//!
//! # Example
//!
//! ```rust
//! extern crate log;
//! extern crate touch;
//!
//! use touch::file;
//!
//! fn main() {
//!     // TODO EXAMPLE
//!     file::write("/tmp/testfile", "this is a test", true);
//! }
//! ```
//!

// TODO add tests
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://doc.rust-lang.org/")]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

#[macro_use] extern crate log;

use std::path::Path;
use std::result;

pub mod dir;
pub mod file;

mod error;
mod operation;

/// A specialized Result type for I/O operations.
///
/// This type is broadly used across the `touch` crate for any operation which
/// may produce an error.
pub type Result<T> = result::Result<T, Error>;
pub use error::Error;

/// Returns whether or not the file at the provided path exists.
pub fn exists(path: &str) -> bool { Path::new(path).exists() }
