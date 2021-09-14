use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

use log::warn;

use crate::constants::*;

pub trait VSmartCard {
    fn get_atr(&self) -> [u8; 10];
    // Nothing to do
    fn power_on(&self);
    // Nothing to do
    fn power_off(&self);
    // Nothing to do
    fn reset(&self);
    // Not implemented
    fn execute(&self, size : u8, msg: [u8; 128]);
}

pub struct SmartCard {
    host : &'static str,
    port : u16,
}

impl Default for SmartCard {
    fn default() -> SmartCard {
        SmartCard{
            host : "127.0.0.1",
            port : 35963,
        }
    }
}

impl SmartCard {
    pub fn new(host: &'static str, port: u16) -> Self {
        SmartCard{
            host,
            port,
        }
    }
    pub fn run(&mut self) {
        let mut stream = TcpStream::connect((self.host, self.port)).expect("Unable to connect to VPCD");

        loop {
            let mut _size = [0, 0];
            let byte_read = stream.read(&mut _size).expect("VirtualPCD shut down.");
            let size = _size[1];
            if byte_read == 0 {
                break;
            }
            let mut msg = [0; 128];
            let byte_read =stream.read(&mut msg).expect("VirtualPCD shut down.");
            if byte_read == 0 {
                break;
            }

            if size == VPCD_CTRL_LEN {
                match msg[0] {
                    VPCD_CTRL_OFF => self.power_off(),
                    VPCD_CTRL_ON => self.power_on(),
                    VPCD_CTRL_RESET => self.reset(),
                    VPCD_CTRL_ATR => {
                        println!("Sending ATR");
                        let atr = self.get_atr();
                        // [0x00, size, ;, atr
                        let msg_o = [&[0x00], &[11], &[59], &atr[..]].concat();
                        let bytes_written = stream.write(&msg_o).expect("Unable to send the message. Virtual PCD shut down,");
                        if bytes_written == 0 {
                            println!("Error sending ATR");
                            break;
                        }
                        println!("Finished sending ATR");
                    },
                    _ => warn!("Unknown command"),
                }
            } else if size == 0 {
                println!("Virtual PCD Shut down.");
                break;
            } else {
                self.execute(size, msg);
                println!("Wrong size received");
            }
        }
        stream.shutdown(Shutdown::Both).expect("Impossible to shut down");
    }
}

impl VSmartCard for SmartCard {
    fn get_atr(&self) -> [u8; 10] {
        // For now the ATR value is a constant, logic will be implemented later.
        ATR_VALUE
    }
    // Nothing to do
    fn power_on(&self) { println!("Power On");}
    // Nothing to do
    fn power_off(&self) {println!("Power Off");}
    // Nothing to do
    fn reset(&self) {println!("Reset");}
    // Not implemented
    fn execute(&self, size : u8, msg: [u8; 128]) {
        let mut buf = vec![0; size.into()];
        buf.copy_from_slice(&msg);
        println!("Received APDU Comand : {:?}", buf);
    }
}
