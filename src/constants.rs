// Some useful constants for connection with VPCD

// The VPCD control commands.
pub const VPCD_CTRL_LEN : usize = 1;
pub const VPCD_CTRL_OFF : u8 = 0;
pub const VPCD_CTRL_ON : u8= 1;
pub const VPCD_CTRL_RESET : u8= 2;
pub const VPCD_CTRL_ATR : u8= 4;

// The hardcoded ATR value until the logic is implemented.
pub const ATR_VALUE : [u8; 11]= [0x3b, 0x95, 0x13, 0x81, 0x01, 0x80, 0x73, 0xff, 0x01, 0x00, 0x0B];
