use game::{schemas::RawMsgBuffer, Error, Result};

#[derive(Debug)]
pub struct Client {}

impl Client {
    pub fn new(address: &str) -> Self {
        Client {}
    }

    pub fn take_responses(&mut self) -> Result<RawMsgBuffer> {
        Err(Error::Unknown("Not implemented".to_string()))
    }
}
