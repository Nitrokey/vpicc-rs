use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

use log::{warn, info, debug, trace};

use crate::constants::*;

pub trait VSmartCard {
    fn get_atr(&self) -> [u8; 11] {
        // For now the ATR value is a constant, logic will be implemented later.
        ATR_VALUE
    }
    // Nothing to do
    fn power_on(&mut self);
    // Nothing to do
    fn power_off(&mut self);
    // Nothing to do
    fn reset(&mut self);
    // Not implemented
    fn execute(&mut self, msg: &[u8]) -> Vec<u8>;
}

pub struct SmartCard<C: VSmartCard> {
    host : &'static str,
    port : u16,
    card: C,
}

impl Default for SmartCard<DummySmartCard> {
    fn default() -> SmartCard<DummySmartCard> {
        Self::with_card(DummySmartCard)
    }
}

impl<C: VSmartCard> SmartCard<C> {
    pub fn new(host: &'static str, port: u16, card: C) -> Self {
        Self {
            host,
            port,
            card,
        }
    }

    pub fn with_card(card: C) -> Self {
        Self {
            host : "127.0.0.1",
            port : 35963,
            card: card,
        }
    }

    pub fn run(&mut self) {
        let mut connection = self.connect();
        let mut cont = true;
        while cont {
            cont = connection.poll();
        }
        connection.shutdown();
    }

    pub fn connect(&mut self) -> Connection<'_, C> {
        info!("Connecting to vpcd on {}:{}", self.host, self.port);
        let stream = TcpStream::connect((self.host, self.port)).expect("Unable to connect to VPCD");
        Connection {
            stream,
            card: &mut self.card,
        }
    }
}

pub struct Connection<'a, C: VSmartCard> {
    stream: TcpStream,
    card: &'a mut C,
}

impl<'a, C: VSmartCard> Connection<'a, C> {
    pub fn poll(&mut self) -> bool {
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
                VPCD_CTRL_OFF => self.card.power_off(),
                VPCD_CTRL_ON => self.card.power_on(),
                VPCD_CTRL_RESET => self.card.reset(),
                VPCD_CTRL_ATR => {
                    debug!("Sending ATR");
                    self.send(&self.card.get_atr());
                },
                _ => warn!("Unknown command"),
            }
        } else if size == 0 {
            info!("Virtual PCD Shut down.");
            return false;
        } else {
            debug!("APDU received");
            trace!("received: {:x?}", msg);
            let response = self.card.execute(&msg);
            trace!("response: {:X?}", response);
            self.send(&response);
        }

        true
    }

    pub fn shutdown(&self) {
        self.stream.shutdown(Shutdown::Both).expect("Impossible to shut down");
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

pub struct DummySmartCard;

impl VSmartCard for DummySmartCard {
    // Nothing to do
    fn power_on(&mut self) { println!("Power On");}
    // Nothing to do
    fn power_off(&mut self) {println!("Power Off");}
    // Nothing to do
    fn reset(&mut self) {println!("Reset");}
    // Not implemented
    fn execute(&mut self, msg: &[u8]) -> Vec<u8> {
        info!("Received APDU Comand : {:?}", msg);
        vec![0x90, 0x00]
    }
}
