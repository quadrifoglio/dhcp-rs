/*
 * Writer - Construct DHCP packets
 */

use std::error::Error as StdError;
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};

use common::{Result, Error, Option, Frame};

impl Option {
    /*
     * Construct a DHCP option
     */
    pub fn new(tag: u8) -> Option {
        Option {
            tag: tag,
            len: 0,
            data: Vec::new()
        }
    }

    /*
     * Set an option's data
     */
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.len = data.len() as u8;
        self.data = data;
    }

    /*
     * Set an option's data as a string
     */
    pub fn set_data_str(&mut self, data: &str) {
        self.len = data.len() as u8;
        self.data = data.to_string().into_bytes();
    }

    /*
     * Set an option's data as a single byte
     */
    pub fn set_data_u8(&mut self, data: u8) {
        self.len = 1;
        self.data = vec![data];
    }

    /*
     * Set an option's data as a 16 bit integer
     */
    pub fn set_data_u16(&mut self, data: u16) -> Result<()> {
        self.len = 2;
        self.data = Vec::new();

        match self.data.write_u16::<BigEndian>(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    /*
     * Set an option's data as a 32 bit integer
     */
    pub fn set_data_u32(&mut self, data: u32) -> Result<()> {
        self.len = 4;
        self.data = Vec::new();

        match self.data.write_u32::<BigEndian>(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    /*
     * Set an option's data as a 64 bit integer
     */
    pub fn set_data_u64(&mut self, data: u64) -> Result<()> {
        self.len = 8;
        self.data = Vec::new();

        match self.data.write_u64::<BigEndian>(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    /*
     * Get the binary representation of an option
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2 + self.data.len());

        buf.push(self.tag);
        buf.push(self.len);
        buf.extend(self.data.iter());

        buf
    }
}

impl Frame {
    /*
     * Construct a classical (ethernet) DHCP frame
     */
    pub fn new(op: u8, xid: u32) -> Frame {
        Frame {
            op: op,
            htype: 0x01,
            hlen: 6,
            hops: 0,
            xid: xid,
            secs: 0,
            flags: 0x00,
            ciaddr: vec![0; 4],
            yiaddr: vec![0; 4],
            siaddr: vec![0; 4],
            giaddr: vec![0; 4],
            chaddr: vec![0; 16],
            sname: vec![0; 64],
            file: vec![0; 128],
            options: Vec::new(),
        }
    }

    /*
     * Add an option to the frame
     */
    pub fn add_option(&mut self, opt: Option) {
        self.options.push(opt);
    }

    /*
     * Get the binary representation of a frame
     */
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(236);

        // One byte fields, first line
        buf.push(self.op);
        buf.push(self.htype);
        buf.push(self.hlen);
        buf.push(self.hops);

        // 2nd and 3rd line
        try!(buf.write_u32::<BigEndian>(self.xid));
        try!(buf.write_u16::<BigEndian>(self.secs));
        try!(buf.write_u16::<BigEndian>(self.flags));

        // Adresses
        buf.extend(self.ciaddr.iter());
        buf.extend(self.yiaddr.iter());
        buf.extend(self.siaddr.iter());
        buf.extend(self.giaddr.iter());
        buf.extend(self.chaddr.iter());

        // Strings
        buf.extend(self.sname.iter());
        buf.extend(self.file.iter());

        // DHCP Magic cookie
        buf.extend(vec![0x63, 0x82, 0x53, 0x63]);

        // Options
        for opt in self.options.iter() {
            buf.extend(opt.to_bytes());
        }

        Ok(buf)
    }
}
