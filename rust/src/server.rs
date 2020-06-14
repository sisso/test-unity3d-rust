pub mod server_ffi;
pub mod server_sockets;

pub type UserId = u16;

pub type RawMsg = [u8];
pub type RawMsgBuffer = Vec<u8>;

#[derive(Debug)]
pub struct Msg<'a> {
    pub namespace: u16,
    pub kind: u16,
    pub body: &'a [u8],
}

#[derive(Debug)]
pub struct MsgBuffer {
    pub namespace: u16,
    pub kind: u16,
    pub body: Vec<u8>,
}

// pub const NAMESPACE_CONNECTION: u16 = 0;
// pub const NAMESPACE_APP: u16 = 1;
// pub const NAMESPACE_GAME: u16 = 2;
//
// pub const KIND_CONNECTION_LOGIN_REQUEST: u16 = 0;
// pub const KIND_CONNECTION_RESPONSE: u16 = 0;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ServerConnector {
    fn push(&mut self, user_id: UserId, msg: &RawMsg) -> Result<()>;
    fn take(&mut self, user_id: UserId) -> Result<Vec<RawMsgBuffer>>;
}

#[derive(Debug, Clone)]
pub struct Server {}

impl Server {
    fn push(&mut self, user_id: UserId, msg: &Msg) -> Result<()> {
        unimplemented!()
    }

    fn take(&mut self, user_id: UserId) -> Result<MsgBuffer> {
        unimplemented!()
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {}
    }
}
