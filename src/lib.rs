extern crate byteorder;

use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::convert::{From, Into};
use std::vec::Vec;
use std::io::{self, Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

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
 * Our Result type
 */
pub type Result<T> = StdResult<T, Error>;

/*
 * Represents a BOOTP/DHCP option
 */
pub struct Option {
    tag: u8,      // Option unique identifier
    len: u8,      // Option length
    data: Vec<u8> // Option data, 'len' bytes of data
}

/*
 * Represents a complete DHCP frame
 */
pub struct Frame {
    op: u8,          // Opcode
    htype: u8,       // Hardware address type
    hlen: u8,        // Hardware address length
    hops: u8,        // Initially 0, incremented by each relay
    xid: u32,        // Transation ID
    secs: u16,       // Seconds elapsed since the process was initiated
    flags: u16,      // Flags
    ciaddr: Vec<u8>, // Client IP address (not used in DHCP)
    yiaddr: Vec<u8>, // Your/client IP address
    siaddr: Vec<u8>, // Next server IP address
    giaddr: Vec<u8>, // Relay IP address
    chaddr: Vec<u8>, // Client hardware address
    sname: String,   // Server hostname
    file: String,    // Boot file name, if any

    options: Vec<Option> // List of BOOTP/DHCP options
}

impl Frame {
    /*
     * Construct a Frame structure based on received data
     */
    pub fn parse(buf: &[u8]) -> Result<Frame> {
        if buf.len() < 44 {
            return Err(Error::new("Frame too short"));
        }

        let mut cur = Cursor::new(buf);
        let mut first = [0; 4];

        try!(cur.read_exact(&mut first));

        let op = first[0];
        let htype = first[1];
        let hlen = first[2];
        let hops = first[3];

        let xid = try!(cur.read_u32::<BigEndian>());
        let secs = try!(cur.read_u16::<BigEndian>());
        let flags = try!(cur.read_u16::<BigEndian>());

        let mut ciaddr = vec![0; 4];
        let mut yiaddr = vec![0; 4];
        let mut siaddr = vec![0; 4];
        let mut giaddr = vec![0; 4];
        let mut chaddr = vec![0; 16];

        try!(cur.read_exact(&mut ciaddr));
        try!(cur.read_exact(&mut yiaddr));
        try!(cur.read_exact(&mut siaddr));
        try!(cur.read_exact(&mut giaddr));
        try!(cur.read_exact(&mut chaddr));

        Ok(Frame {
            op: 0,
            htype: 0,
            hlen: 0,
            hops: 0,
            xid: 0,
            secs: 0,
            flags: 0,
            ciaddr: ciaddr,
            yiaddr: yiaddr,
            siaddr: siaddr,
            giaddr: giaddr,
            chaddr: chaddr,
            sname: String::new(),
            file: String::new(),
            options: Vec::new()
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
