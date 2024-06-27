use std::{net::{IpAddr, SocketAddr, TcpStream}, time::Duration};

use crate::protocol::*;


#[derive(Debug)]
pub struct WitcherConnection {
    stream: TcpStream
}

impl WitcherConnection {
    pub const GAME_PORT: u16 = 37001;

    pub fn connect(ip: IpAddr) -> anyhow::Result<Self> {
        let addr = SocketAddr::new(ip, Self::GAME_PORT);
        let stream = TcpStream::connect(addr)?;
        
        Ok(Self {
            stream
        })
    }

    pub fn connect_timeout(ip: IpAddr, timeout: Duration) -> anyhow::Result<Self> {
        let addr = SocketAddr::new(ip, Self::GAME_PORT);
        let stream = TcpStream::connect_timeout(&addr, timeout)?;

        Ok(Self {
            stream
        })
    }

    pub fn try_clone(&self) -> anyhow::Result<Self> {
        let cloned_stream = self.stream.try_clone()?;

        Ok(Self {
            stream: cloned_stream
        })
    }


    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) -> anyhow::Result<()> {
        self.stream.set_read_timeout(timeout)?;
        Ok(())
    }


    pub fn send(&mut self, packet: WitcherPacket) -> anyhow::Result<()> {
        packet.encode_into(&mut self.stream)?;
        Ok(())
    }

    pub fn receive(&mut self) -> anyhow::Result<WitcherPacket> {
        WitcherPacket::decode_from(&mut self.stream)
    }

    pub fn peek(&self) -> anyhow::Result<bool> {
        let mut peek_buffer = [0u8; WitcherPacket::min_encoded_size()];
        match self.stream.peek(&mut peek_buffer) {
            Ok(peeked) => {
                Ok(peeked >= peek_buffer.len())
            }
            Err(err) if matches!(err.kind(), std::io::ErrorKind::TimedOut) => {
                Ok(false)
            },
            Err(err) => {
                Err(err)?
            }
        }
    }


    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}