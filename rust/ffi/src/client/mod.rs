use game::client::SocketClient;
use game::packages::package_buffer::PackageBuffer;
use game::{schemas::RawMsgBuffer, Error, Result};
use logs::*;
use std::io::ErrorKind;

#[derive(Debug)]
enum State {
    NotConnected,
    Connected {
        buffer: PackageBuffer,
        socket: SocketClient,
        queue: Vec<(u16, RawMsgBuffer)>,
    },
}

#[derive(Debug)]
pub struct Client {
    address: String,
    state: State,
}

impl Client {
    pub fn new(address: &str) -> Self {
        Client {
            address: address.to_string(),
            state: State::NotConnected,
        }
    }

    fn check_connection(&mut self) -> Result<()> {
        match self.state {
            State::NotConnected => {
                // println!("connecting");
                // TODO: this looks bad, what if connection hangout?
                let socket = SocketClient::connect(&self.address)?;

                self.state = State::Connected {
                    buffer: PackageBuffer::new(),
                    socket,
                    queue: vec![],
                };

                // println!("connected");
            }
            _ => {}
        }

        Ok(())
    }

    pub fn take_responses(&mut self) -> Result<Option<(u16, RawMsgBuffer)>> {
        self.check_connection()?;

        match &mut self.state {
            State::Connected {
                buffer,
                socket,
                queue,
            } => match socket.tick() {
                Ok(()) => {
                    if let Some(vec) = socket.take() {
                        let packages = buffer.push(vec);
                        queue.extend(packages);
                    }

                    if queue.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(queue.remove(0)))
                    }
                }

                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                    info!("switching to not connected");
                    self.state = State::NotConnected;
                    Err(Error::Disconnect)
                }

                Err(e) => Err(e.into()),
            },
            State::NotConnected => {
                // TODO: should we do something?
                Err(Error::Disconnect)
            }
        }
    }
}
