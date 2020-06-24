use game::{schemas::RawMsgBuffer, Error, Result};
use game::client::SocketClient;
use game::packages::package_buffer::PackageBuffer;

#[derive(Debug)]
pub struct Client {
    buffer: PackageBuffer,
    socket: SocketClient,
    queue: Vec<RawMsgBuffer>,
}

impl Client {
    pub fn connect(address: &str) -> Result<Self> {
        let socket = SocketClient::connect(address)?;

        Ok(Client {
            buffer: PackageBuffer::new(),
            socket,
            queue: Default::default(),
        })
    }

    pub fn take_responses(&mut self) -> Result<Option<RawMsgBuffer>> {
        self.socket.tick();

        if let Some(vec) = self.socket.take() {
            let packages = self.buffer.push(vec);
            self.queue.extend(packages);
        }

        if self.queue.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.queue.remove(0)))
        }
    }
}
