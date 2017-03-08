/*
 * DHCP Server - Handle & respond to requests
 */

use std::error::Error as StdError;
use std::net::UdpSocket;

use common::{Result, Error, Frame};

/*
 * Start listening for requests on the specified address and port
 */
pub fn listen(addr: &str) -> Result<()> {
    // Bind the socket
    let mut socket = match UdpSocket::bind(addr) {
        Ok(socket) => socket,
        Err(e) => return Err(Error::new(e.description()))
    };

    // Forever
    loop {
        // 1024 bytes buffer
        let mut buf = [0; 1024];

        // On each datagram
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                // Handle the request
                let frame = match Frame::parse(&buf[..len]) {
                    Ok(frame) => {

                    },
                    Err(e) => {
                        println!("Failed to parse DHCP frame: {}", e);
                        continue;
                    }
                };
            },
            Err(e) => return Err(Error::new(e.description()))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn listen() {
        super::listen("0.0.0.0:67").unwrap();
    }
}
