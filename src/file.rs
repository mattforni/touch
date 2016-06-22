//! This module contains basic methods to manipulate the contents of local files.
//! All methods in this module represent cross-platform filesystem operations.

use std::fs::{self,File};
use std::io::{Read,Write};
use std::path::Path;

use error::Error;
use operation::Operation;
use super::Result;
use super::dir;
use super::exists as exists;

/// Creates the file indicated by the provided path. If truncate is `true`
/// and the file already exists, this method will overwrite the file.
///
/// # Failures
/// Fails if the file cannot be created for any reason.
pub fn create(path: &str, truncate: bool) -> Result<Option<File>> {
    let file_exists = exists(path);
    if !truncate && file_exists { return Ok(None); }

    if !file_exists { debug!("'{}' DNE, creating", path); }

    let directory = try!(Path::new(path).parent().ok_or(
        Error::new(&Operation::Read, path)
    ).and_then(|parent| {
        parent.as_os_str().to_str().ok_or(
            Error::new(&Operation::Read, path)
        )
    }));

    if !exists(directory) {
        debug!("'{}' DNE, creating", directory);
        try!(dir::create(directory));
    }

    let file = try!(File::create(path).map_err(|error| {
        Error::with_cause(&Operation::Create, path, error)
    }));
    if !exists(path) { debug!("Successfully created '{}'", path); }

    Ok(Some(file))
}

/// Idempotently deletes the file indicated by the provided path.
///
/// # Failures
/// Fails if the file cannot be deleted for any reason.
pub fn delete(path: &str) -> Result<()> {
    if !exists(path) { return Ok(()); }

    fs::remove_file(path).map_err(|error| {
        Error::with_cause(&Operation::Delete, path, error)
    })
}

/// Reads the contents of the file indicated by the provided path into a String.
///
/// # Failures
/// Fails if the file cannot be opened or read for any reason.
pub fn read(path: &str) -> Result<String> {
    let mut file = try!(File::open(path).map_err(|error| {
        Error::with_cause(&Operation::Read, path, error)
    }));

    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(|error| {
        Error::with_cause(&Operation::Read, path, error)
    }));

    Ok(content)
}

/// Creates the file indicated by the provided path and writes the
/// contents if the content string is not empty. If truncate is `true`
/// and the file already exists, this method will overwrite the file.
///
/// # Failures
/// Fails if the file cannot be created or written to for any reason.
pub fn write(path: &str, content: &str, truncate: bool) -> Result<()> {
    if content.is_empty() { return Ok(()); }

    let mut file = try!(create(path, truncate)).unwrap();

    try!(file.write_all(content.as_bytes()).map_err(|error| {
        Error::with_cause(&Operation::Write, path, error)
    }));

    debug!("Wrote '{}' to '{}'", content, path);
    Ok(())
}
