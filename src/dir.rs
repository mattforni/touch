//! TODO add docs

use std::fs;

use error::Error;
use operation::Operation;
use super::Result;
use super::exists as exists;

/// Recursively creates the directory indicated by the provided path if it
/// does not exist, otherwise it short circuits and returns immediately.
///
/// # Failures
/// Fails if the directory cannot be created for any reason.
pub fn create(path: &str) -> Result<()> {
    if exists(path) { return Ok(()); }

    match fs::create_dir_all(path) {
        Ok(_) => {
            debug!("Successfully created '{}'", path);
            Ok(())
        },
        Err(error) => {
            if exists(path) {
                Ok(())
            } else {
                Err(Error::with_cause(&Operation::Create, path, error)) }
        }
    }
}

/// Deletes the directory indicated by the provided path if it exists.
///
/// # Failures
/// Fails if the directory cannot be deleted for any reason.
pub fn delete(path: &str) -> Result<()> {
    if !exists(path) { return Ok(()); }

    match fs::remove_dir_all(path) {
        Ok(_) => {
            debug!("Successfully deleted '{}'", path);
            Ok(())
        },
        Err(error) => {
            if !exists(path) {
                Ok(())
            } else {
                Err(Error::with_cause(&Operation::Delete, path, error))
            }
        }
    }
}
