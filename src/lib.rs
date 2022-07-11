use std::{
    fmt::Display,
    io::{Error, ErrorKind, Read, Result, Write},
    net::{Ipv4Addr, SocketAddr, TcpStream, ToSocketAddrs},
};

use log::{debug, info, trace};

pub const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const DEFAULT_PORT: u16 = 35963;
pub const DEFAULT_ATR: &[u8] = &[
    0x3b, 0x95, 0x13, 0x81, 0x01, 0x80, 0x73, 0xff, 0x01, 0x00, 0x0B,
];

pub fn connect() -> Result<Connection> {
    connect_socket(SocketAddr::new(DEFAULT_HOST.into(), DEFAULT_PORT))
}

pub fn connect_socket<A: ToSocketAddrs + Display>(addr: A) -> Result<Connection> {
    info!("Connecting to vpcd on {}", addr);
    TcpStream::connect(addr).map(Connection::from)
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
    pub fn run<V: VSmartCard>(mut self, card: &mut V) -> Result<()> {
        loop {
            self.poll(card)?;
        }
    }

    pub fn poll<V: VSmartCard>(&mut self, card: &mut V) -> Result<()> {
        let msg = self.read()?;
        if msg.is_empty() {
            return Err(Error::new(ErrorKind::Other, "received an empty message"));
        }

        if msg.len() == 1 {
            match Command::try_from(msg[0])? {
                Command::PowerOff => card.power_off(),
                Command::PowerOn => card.power_on(),
                Command::Reset => card.reset(),
                Command::GetAtr => {
                    debug!("Sending ATR");
                    self.send(card.atr())?;
                }
            }
        } else {
            debug!("APDU received");
            let response = card.execute(&msg);
            self.send(&response)?;
        }

        Ok(())
    }

    fn read(&mut self) -> Result<Vec<u8>> {
        let mut size = [0, 0];
        self.stream.read_exact(&mut size)?;
        let size = usize::from(u16::from_be_bytes(size));
        let mut msg = vec![0u8; size];
        self.stream.read_exact(&mut msg)?;
        trace!("received message: {:x?}", msg);
        Ok(msg)
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        trace!("sending message: {:x?}", data);
        let size = (data.len() as u16).to_be_bytes();
        let msg = &[&size[..], data].concat();
        self.stream.write_all(msg)?;
        Ok(())
    }
}

impl From<TcpStream> for Connection {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}

enum Command {
    PowerOff,
    PowerOn,
    Reset,
    GetAtr,
}

impl TryFrom<u8> for Command {
    type Error = Error;

    fn try_from(command: u8) -> Result<Self> {
        // https://frankmorgner.github.io/vsmartcard/virtualsmartcard/api.html
        match command {
            0 => Ok(Self::PowerOff),
            1 => Ok(Self::PowerOn),
            2 => Ok(Self::Reset),
            4 => Ok(Self::GetAtr),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("unsupported control command {}", command),
            )),
        }
    }
}

pub struct DummySmartCard;

impl VSmartCard for DummySmartCard {
    fn power_on(&mut self) {
        info!("Power On");
    }
    fn power_off(&mut self) {
        info!("Power Off");
    }
    fn reset(&mut self) {
        info!("Reset");
    }
    fn execute(&mut self, msg: &[u8]) -> Vec<u8> {
        info!("Received APDU Comand : {:?}", msg);
        vec![0x90, 0x00]
    }
}
