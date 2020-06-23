use std::net::{TcpStream, Shutdown};
use std::io::Write;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect(address: &str) -> std::io::Result<Self> {
        let mut stream = TcpStream::connect(address)?;
        stream.set_nonblocking(true)?;

        Ok(Client {
            stream
        })
    }

    pub fn send(&mut self, bytes: Vec<u8>) -> std::io::Result<()> {
        let size = bytes.len();
        let amount = self.stream.write(&bytes)?;
        self.stream.flush()?;
        assert_eq!(amount, size);
        Ok(())
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)
    }
}

