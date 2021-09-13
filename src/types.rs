use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

use log::{info, warn};

use crate::constants::*;

pub struct SmartCard {
    host : &'static str,
    port : u16,
}


impl SmartCard {
    pub fn new() -> Self {
        SmartCard{
            host : "127.0.0.1",
            port : 35963,
        }
    }

    fn getATR(&self) -> [u8; 10] {
        // For now the ATR value is a constant, logic will be implemented later.
        return ATR_VALUE;
    }
    // Nothing to do
    fn powerOn(&self) { println!("Power On");}
    // Nothing to do
    fn powerOff(&self) {println!("Power Off");}
    // Nothing to do
    fn reset(&self) {println!("Reset");}
    // Not implemented
    fn execute(&self) { info!("Not implemented APDU commands");}
    pub fn run(&mut self) {
        let mut stream = TcpStream::connect((self.host, self.port)).expect("Unable to connect to VPCD");

        loop {
            let mut _size = [0, 0];
            stream.read(&mut _size).expect("VirtualPCD shut down.");
            let size = _size[1];
            let mut msg = [0];
            stream.read(&mut msg).expect("VirtualPCD shut down.");
            println!("{:?}", msg);

            if size == VPCD_CTRL_LEN {
                match msg[0] {
                    VPCD_CTRL_OFF => self.powerOff(),
                    VPCD_CTRL_ON => self.powerOn(),
                    VPCD_CTRL_RESET => self.reset(),
                    VPCD_CTRL_ATR => {
                        println!("Sending ATR");
                        let atr = self.getATR();
                        // [0x00, size, ;, atr
                        let msg_o = [&[0x00], &[11], &[59], &atr[..]].concat();
                        stream.write(&msg_o).expect("Unable to send the message. Virtual PCD shut down,");
                        println!("Finished sending ATR");
                    },
                    _ => warn!("Unknown command"),
                }
            } else if size == 0 {
                println!("Virtual PCD Shut down.");
                break;
            } else {
                println!("Wrong size received");
            }
        }
        stream.shutdown(Shutdown::Both).expect("Impossible to shut down");
    }
}
