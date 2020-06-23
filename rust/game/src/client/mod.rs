use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};

/// Read and write from a socket in non blocking mode with buffers
pub struct Client {
    stream: TcpStream,
    /// bytes to be send
    output_buffer: Vec<u8>,
    /// bytes to be taken
    input_buffer: Vec<u8>,
    /// buffer to read bytes
    tmp_input_buffer: Vec<u8>,
}

impl Client {
    pub fn connect(address: &str) -> std::io::Result<Self> {
        let mut stream = TcpStream::connect(address)?;
        stream.set_nonblocking(true)?;

        let mut buffer = Vec::with_capacity(1024);
        for _ in 0..1024 {
            buffer.push(0);
        }

        Ok(Client {
            stream,
            output_buffer: vec![],
            input_buffer: vec![],
            tmp_input_buffer: buffer,
        })
    }

    pub fn push(&mut self, bytes: Vec<u8>) {
        self.output_buffer.extend(bytes);
    }

    pub fn tick(&mut self) -> std::io::Result<()> {
        if !self.output_buffer.is_empty() {
            let size = self.stream.write(&self.output_buffer)?;
            self.output_buffer.drain(0..size);
            self.stream.flush()?;
        }

        match self.stream.read(&mut self.tmp_input_buffer) {
            Ok(size) if size > 0 => {
               self.input_buffer.extend_from_slice(&self.tmp_input_buffer[0..size]);
               Ok(())
            },
            Ok(_) => Ok(()),
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(()),
            Err(e) =>
               Err(e),
        }
    }

    pub fn take(&mut self) -> Option<Vec<u8>> {
        if self.input_buffer.is_empty() {
            None
        } else {
            Some(std::mem::replace(&mut self.input_buffer, Vec::new()))
        }
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)
    }
}

