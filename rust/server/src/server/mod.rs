use game::schemas::RawMsgBuffer;

pub mod server_socket;

pub type ConnectionId = u32;

/// From the server to the connection
#[derive(Debug, Clone)]
pub struct ServerOutput {
    pub connection_id: ConnectionId,
    pub msg: RawMsgBuffer,
}

/// From the connection into the server
#[derive(Debug, Clone)]
pub struct ServerInput {
    pub connection_id: ConnectionId,
    pub msg: RawMsgBuffer,
}

#[derive(Debug, Clone)]
pub struct ServerChanges {
    pub connects: Vec<ConnectionId>,
    pub disconnects: Vec<ConnectionId>,
    pub inputs: Vec<ServerInput>,
}

pub trait Server {
    fn run(&mut self) -> ServerChanges;
    fn output(&mut self, connection_id: ConnectionId, msg: RawMsgBuffer);
    fn disconnect(&mut self, connection_id: ConnectionId);
}
