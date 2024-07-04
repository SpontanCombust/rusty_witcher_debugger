use std::{io::Write, net::{IpAddr, SocketAddr, TcpStream}, time::Duration};

use anyhow::Context;

use crate::protocol::*;


#[derive(Debug)]
pub struct WitcherConnection {
    stream: TcpStream
}

impl WitcherConnection {
    pub const GAME_PORT: u16 = 37001;
    /// A read timeout is necessary to be able to shut down the connection
    /// Without it it would block infinitely until it would receive data or the connection was severed
    pub const DEFAULT_READ_TIMEOUT_MILLIS: u64 = 2000;

    pub fn connect(ip: IpAddr) -> anyhow::Result<Self> {
        let addr = SocketAddr::new(ip, Self::GAME_PORT);
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(std::time::Duration::from_millis(Self::DEFAULT_READ_TIMEOUT_MILLIS)))?;
        
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


    pub fn set_read_timeout(&mut self, timeout: Duration) -> anyhow::Result<()> {
        self.stream.set_read_timeout(Some(timeout))?;
        Ok(())
    }

    pub fn get_read_timeout(&self) -> anyhow::Result<Duration> {
        let timeout = self.stream.read_timeout()?.context("No timeout set")?;
        Ok(timeout)
    }


    pub fn send(&mut self, packet: WitcherPacket) -> anyhow::Result<()> {
        const BUFFER_SIZE: usize = 1024;
        let mut buf = Vec::<u8>::with_capacity(BUFFER_SIZE);
        packet.encode_into(&mut buf)?;
        // writing everything at once to make network debugging easier
        // these outgoing packets are never really big, so it doesn't make sense to chop them up so much
        self.stream.write_all(&buf)?;
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