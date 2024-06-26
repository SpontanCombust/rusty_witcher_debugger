use std::{net::{IpAddr, SocketAddr, TcpStream}, time::Duration};

use crate::{encoding::{Decode, Encode}, packet::WitcherPacket};


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
        let available = self.stream.peek(&mut peek_buffer)? >= peek_buffer.len();
        Ok(available)
    }


    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}