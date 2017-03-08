use std::vec::Vec;

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
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: u32,
    yiaddr: u32,
    siaddr: u32,
    ciaddr: u32,
    chaddr: Vec<u8>,
    sname: String,
    file: String,

    options: Vec<Option> // List of BOOTP/DHCP options
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
