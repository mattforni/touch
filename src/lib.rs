// Copyright 2016 Matthew Fornaciari <mattforni@gmail.com>
//! A dead simple wrapper around file and directory manipulation.
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
//! extern crate touch;
//!
//! use touch::exists;
//! use touch::dir;
//! use touch::file;
//!
//! const DIR: &'static str = "/tmp/touch";
//! const FILE_NAME: &'static str = ".example";
//!
//! fn main() {
//!     assert!(!exists(DIR));
//!     assert!(!exists(&path()));
//!
//!     // Write
//!     let content = "Content";
//!     assert!(file::write(&path(), content, false).is_ok());
//!
//!     // Read
//!     let mut output = file::read(&path());
//!     assert_eq!(content, output.unwrap());
//!
//!     // Overwrite
//!     let new_content = "New Content";
//!     assert!(file::write(&path(), new_content, true).is_ok());
//!     output = file::read(&path());
//!     assert_eq!(new_content, output.unwrap());
//!
//!     // Delete
//!     assert!(dir::delete(DIR).is_ok());
//!     assert!(!exists(&path()));
//!     assert!(!exists(DIR));
//! }
//!
//! fn path() -> String {
//!     format!("{}/{}", DIR, FILE_NAME)
//! }
//! ```

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
#[cfg(test)] mod tst;

/// A specialized Result type for I/O operations.
///
/// This type is broadly used across the `touch` crate for any operation which
/// may produce an error.
pub type Result<T> = result::Result<T, Error>;
pub use error::Error;

/// Returns whether or not the file at the provided path exists.
pub fn exists(path: &str) -> bool { Path::new(path).exists() }
