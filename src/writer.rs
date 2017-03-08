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
}
