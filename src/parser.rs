/*
 * DHCP Frame parsing
 */

use std::vec::Vec;
use std::io::{self, Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use common::{Result, Error};

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
            op: op,
            htype: htype,
            hlen: hlen,
            hops: hops,
            xid: xid,
            secs: secs,
            flags: flags,
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
    #[should_panic]
    fn test_empty_invalid() {
        let data = [];
        super::Frame::parse(&data).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_too_short_invalid() {
        let data = [
            0x01, 0x01, 0x06, 0x00
        ];

        super::Frame::parse(&data).unwrap();
    }

    #[test]
    fn test_header_valid() {
        let data = [
            0x01, 0x01, 0x06, 0x00,
            0x6e, 0x86, 0x44, 0x4c,
            0x00, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x52, 0x54, 0x01, 0x12,
            0x34, 0x56, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let frame = super::Frame::parse(&data).unwrap();

        assert_eq!(frame.op, 0x01);
        assert_eq!(frame.htype, 0x01);
        assert_eq!(frame.hlen, 6);
        assert_eq!(frame.hops, 0);
        assert_eq!(frame.xid, 0x6e86444c);
        assert_eq!(frame.secs, 8);
        assert_eq!(frame.flags, 0);

        assert_eq!(frame.ciaddr.as_slice(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(frame.yiaddr.as_slice(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(frame.siaddr.as_slice(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(frame.giaddr.as_slice(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(frame.chaddr.as_slice(), [0x52, 0x54, 0x01, 0x12, 0x34, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }
}
