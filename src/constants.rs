// Some useful constants for connection with VPCD

use std::net::Ipv4Addr;

// The VPCD control commands.
pub const VPCD_CTRL_LEN: usize = 1;
pub const VPCD_CTRL_OFF: u8 = 0;
pub const VPCD_CTRL_ON: u8 = 1;
pub const VPCD_CTRL_RESET: u8 = 2;
pub const VPCD_CTRL_ATR: u8 = 4;

pub const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const DEFAULT_PORT: u16 = 35963;
pub const DEFAULT_ATR: &[u8]= &[0x3b, 0x95, 0x13, 0x81, 0x01, 0x80, 0x73, 0xff, 0x01, 0x00, 0x0B];
