# touch

Copyright 2016 Matthew Fornaciari <mattforni@gmail.com>
A dead simple wrapper around file and directory manipulation.

## Usage

This crate is [on crates.io](https://crates.io/crates/touch) and can be
used by adding `args` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
touch = "0"
```

and this to your crate root:

```rust
extern crate touch;
```

## Example

```rust
extern crate touch;

use touch::exists;
use touch::dir;
use touch::file;

const DIR: &'static str = "/tmp/touch";
const FILE_NAME: &'static str = ".example";

fn main() {
    assert!(!exists(DIR));
    assert!(!exists(&path()));

    // Write
    let content = "Content";
    assert!(file::write(&path(), content, false).is_ok());

    // Read
    let mut output = file::read(&path());
    assert_eq!(content, output.unwrap());

    // Overwrite
    let new_content = "New Content";
    assert!(file::write(&path(), new_content, true).is_ok());
    output = file::read(&path());
    assert_eq!(new_content, output.unwrap());

    // Delete
    assert!(dir::delete(DIR).is_ok());
    assert!(!exists(&path()));
    assert!(!exists(DIR));
}

fn path() -> String {
    format!("{}/{}", DIR, FILE_NAME)
}
```
