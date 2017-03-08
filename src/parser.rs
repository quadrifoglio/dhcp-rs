/*
 * DHCP Frame parsing
 */

use std::error::Error as StdError;
use std::vec::Vec;
use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use common::{Result, Error, Option, Frame};

impl Option {
    /*
     * Parse an option from a bytes buffer
     */
    pub fn parse(buf: &[u8]) -> Result<Option> {
        if buf.len() < 2 {
            return Err(Error::new("Frame too short"));
        }

        let mut cur = Cursor::new(buf);

        // Parse tag & length
        let mut first = [0; 2];
        try!(cur.read_exact(&mut first));

        let tag = first[0];
        let len = first[1];

        // Get the data
        let mut data = vec![0; len as usize];
        try!(cur.read_exact(&mut data));

        // Construct the object
        Ok(Option {
            tag: tag,
            len: len,
            data: data
        })
    }

    /*
     * Return the value as a string
     */
    pub fn value_as_string(&self) -> Result<String> {
        let data = self.data.clone();

        match String::from_utf8(data) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(e.description()))
        }
    }
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

        // Parse first line, opcode, htype, hlen and hops
        let op = first[0];
        let htype = first[1];
        let hlen = first[2];
        let hops = first[3];

        // Parse xid, secs, flags
        let xid = try!(cur.read_u32::<BigEndian>());
        let secs = try!(cur.read_u16::<BigEndian>());
        let flags = try!(cur.read_u16::<BigEndian>());

        // Parse adresses
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

        // Parse strings
        let mut sname = vec![0; 64];
        let mut file = vec![0; 128];

        try!(cur.read_exact(&mut sname));
        try!(cur.read_exact(&mut file));

        let mut opts = Vec::new();
        while cur.position() < buf.len() as u64 {
            {
                let buf = cur.get_ref();

                match Option::parse(buf) {
                    Ok(opt) => {
                        if opt.tag == 255 {
                            break
                        }

                        opts.push(opt)
                    },
                    Err(e) => return Err(Error::new(format!("Failed to parse option: {}", e)))
                };
            }

            let pos = cur.position();
            let opt = opts.last().unwrap();

            cur.set_position(pos + 2 + opt.data.len() as u64);
        }

        // Construct object
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
            sname: sname,
            file: file,
            options: opts
        })
    }

    /*
     * Return the client's hardware address as a classical MAC address string
     */
    pub fn client_mac_string(&self) -> String {
        format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.chaddr[0], self.chaddr[1], self.chaddr[2], self.chaddr[3], self.chaddr[4], self.chaddr[5])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_option_empty_invalid() {
        let data = [];
        super::Option::parse(&data).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_option_too_short_invalid() {
        let data = [0x00];
        super::Option::parse(&data).unwrap();
    }

    #[test]
    fn test_option_message_type_valid() {
        // A valid DHCP Message Type (53) option
        let data = [
            0x35, 0x01, 0x01
        ];

        let opt = super::Option::parse(&data).unwrap();

        assert_eq!(opt.tag, 53);
        assert_eq!(opt.len, 1);
        assert_eq!(opt.data, [0x01]);
    }

    #[test]
    fn test_option_vendor_class_valid() {
        // A valid Vendor ID (60) option
        let data = [
            0x3c, 0x20, 0x50, 0x58, 0x45, 0x43, 0x6c, 0x69,
            0x65, 0x6e, 0x74, 0x3a, 0x41, 0x72, 0x63, 0x68,
            0x3a, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3a, 0x55,
            0x4e, 0x44, 0x49, 0x3a, 0x30, 0x30, 0x32, 0x30,
            0x30, 0x31
        ];

        let opt = super::Option::parse(&data).unwrap();

        assert_eq!(opt.tag, 60);
        assert_eq!(opt.len, 32);
        assert_eq!(opt.value_as_string().unwrap().as_str(), "PXEClient:Arch:00000:UNDI:002001");
    }

    #[test]
    #[should_panic]
    fn test_frame_empty_invalid() {
        let data = [];
        super::Frame::parse(&data).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_frame_too_short_invalid() {
        let data = [
            0x01, 0x01, 0x06, 0x00
        ];

        super::Frame::parse(&data).unwrap();
    }

    #[test]
    fn test_frame_header_valid() {
        // Valid DHCP Header
        let data = [
            // first fields (op, secs, flags, addrs...)
            0x01, 0x01, 0x06, 0x00, 0x6e, 0x86, 0x44, 0x4c,
            0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x52, 0x54, 0x01, 0x12,
            0x34, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            // sname
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            // file
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
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

        assert_eq!(frame.client_mac_string().as_str(), "52:54:01:12:34:56");

        assert_eq!(frame.sname.len(), 64);
        assert_eq!(frame.file.len(), 128);
    }
}
