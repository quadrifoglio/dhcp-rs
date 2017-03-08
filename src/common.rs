/*
 * Common global structures
 */

use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::convert::{From, Into};
use std::fmt::{self, Display, Debug, Formatter};
use std::io::{self};

/*
 * Represents a BOOTP/DHCP option
 */
pub struct Option {
    pub tag:  u8,      // Option unique identifier
    pub len:  u8,      // Option length
    pub data: Vec<u8>  // Option data, 'len' bytes of data
}

/*
 * Represents a complete DHCP frame
 */
pub struct Frame {
    pub op:     u8,      // Opcode
    pub htype:  u8,      // Hardware address type
    pub hlen:   u8,      // Hardware address length
    pub hops:   u8,      // Initially 0, incremented by each relay
    pub xid:    u32,     // Transation ID
    pub secs:   u16,     // Seconds elapsed since the process was initiated
    pub flags:  u16,     // Flags
    pub ciaddr: Vec<u8>, // Client IP address (not used in DHCP)
    pub yiaddr: Vec<u8>, // Your/client IP address
    pub siaddr: Vec<u8>, // Next server IP address
    pub giaddr: Vec<u8>, // Relay IP address
    pub chaddr: Vec<u8>, // Client hardware address
    pub sname:  Vec<u8>, // Server hostname
    pub file:   Vec<u8>, // Boot file name, if any

    pub options: Vec<Option> // List of BOOTP/DHCP options
}

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
 * Display trait
 */
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
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
