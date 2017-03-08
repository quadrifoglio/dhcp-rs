/*
 * Common global structures
 */

use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::convert::{From, Into};
use std::fmt::{self, Debug, Formatter};
use std::io::{self};

/*
 * Error type
 */
pub struct Error {
    pub msg: String
}

impl Error {
    /*
     * Construct a new Error with a message
     */
    pub fn new<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into()
        }
    }
}

/*
 * Convert an io::Error to our Error type
 */
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(e.description())
    }
}

/*
 * Debug trait
 */
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error {{ msg: {} }}", self.msg)
    }
}

/*
 * Our Result type
 */
pub type Result<T> = StdResult<T, Error>;
