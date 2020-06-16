use crate::schemas::server_generated::server::PackageKind;

pub type UserId = u16;

///
/// Game logic
///

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Package {
    kind: PackageKind,
    event: Event,
}

pub enum Event {}

#[derive(Debug, Clone)]
pub struct Server {
    state: u32,
}

impl Server {
    pub fn connect(&mut self) -> UserId {
        0
    }

    pub fn take(&mut self, user_id: UserId) -> Vec<Event> {
        Vec::new()
    }
}

impl Default for Server {
    fn default() -> Self {
        Server { state: 0 }
    }
}
