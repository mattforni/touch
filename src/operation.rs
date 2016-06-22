use std::fmt::{self, Display, Formatter};

use self::Operation::*;

mod strings {
   pub const CREATE: &'static str = "create";
   pub const DELETE: &'static str = "delete";
   pub const READ: &'static str = "read";
   pub const WRITE: &'static str = "write";
}

pub enum Operation {
    Create,
    Delete,
    Read,
    Write
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match *self {
            Create => strings::CREATE.to_string(),
            Delete => strings::DELETE.to_string(),
            Read => strings::READ.to_string(),
            Write => strings::WRITE.to_string()
        };

        write!(f, "{}", string)
    }
}
