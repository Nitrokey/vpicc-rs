use std::io::prelude::*;
use std::fmt::Display;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use log::{warn, info, debug, trace};

use crate::constants::*;

pub fn connect() -> Connection {
    connect_socket(SocketAddr::new(DEFAULT_HOST.into(), DEFAULT_PORT))
}

pub fn connect_socket<A: ToSocketAddrs + Display>(addr: A) -> Connection {
    info!("Connecting to vpcd on {}", addr);
    let stream = TcpStream::connect(addr)
        .expect("Unable to connect to VPCD");
    Connection::from(stream)
}

pub trait VSmartCard {
    fn atr(&self) -> &[u8] {
        DEFAULT_ATR
    }
    fn power_on(&mut self) {}
    fn power_off(&mut self) {}
    fn reset(&mut self) {}
    fn execute(&mut self, msg: &[u8]) -> Vec<u8>;
}

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn run<V: VSmartCard>(mut self, card: &mut V) {
        while self.poll(card) {}
    }

    pub fn poll<V: VSmartCard>(&mut self, card: &mut V) -> bool {
        let mut size = [0, 0];
        let byte_read = self.stream.read(&mut size).expect("VirtualPCD shut down.");
        let size = usize::from(u16::from_be_bytes(size));
        if byte_read == 0 {
            return false;
        }

        let mut msg = vec![0u8; size];
        self.stream.read_exact(&mut msg).expect("VirtualPCD shut down.");

        trace!("MSG : {:?}", msg);

        if size == VPCD_CTRL_LEN {
            match msg[0] {
                VPCD_CTRL_OFF => card.power_off(),
                VPCD_CTRL_ON => card.power_on(),
                VPCD_CTRL_RESET => card.reset(),
                VPCD_CTRL_ATR => {
                    debug!("Sending ATR");
                    self.send(&card.atr());
                },
                _ => warn!("Unknown command"),
            }
        } else if size == 0 {
            info!("Virtual PCD Shut down.");
            return false;
        } else {
            debug!("APDU received");
            trace!("received: {:x?}", msg);
            let response = card.execute(&msg);
            trace!("response: {:X?}", response);
            self.send(&response);
        }

        true
    }

    fn send(&mut self, data: &[u8]) {
        let size = (data.len() as u16).to_be_bytes();
        let msg = &[&size[..], data].concat();
        let bytes_written = self.stream.write(msg).expect("Unable to send the message. Virtual PCD shut down,");
        if bytes_written < msg.len() {
            panic!("failed to write all data");
        }
    }
}

impl From<TcpStream> for Connection {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}

pub struct DummySmartCard;

impl VSmartCard for DummySmartCard {
    fn power_on(&mut self) { println!("Power On");}
    fn power_off(&mut self) {println!("Power Off");}
    fn reset(&mut self) {println!("Reset");}
    fn execute(&mut self, msg: &[u8]) -> Vec<u8> {
        info!("Received APDU Comand : {:?}", msg);
        vec![0x90, 0x00]
    }
}
