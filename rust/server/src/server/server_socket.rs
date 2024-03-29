use logs::*;
use std::io;
use std::io::{BufRead, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};

use super::*;
use game::schemas::RawMsg;
use std::collections::HashMap;

pub struct SocketServer {
    next_connection_id: u32,
    connections: HashMap<ConnectionId, Connection>,
    listener: Option<TcpListener>,
    pending_outputs: Option<Vec<ServerOutput>>,
}

struct Connection {
    id: ConnectionId,
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new(id: u32, stream: TcpStream,) -> Self {
        let mut buffer = Vec::with_capacity(1024);
        for _ in 0..1024 {
            buffer.push(0);
        }
        Connection { id, stream, buffer: buffer }
    }

    pub fn write(&mut self, msg: &RawMsg) -> io::Result<()> {
        // TODO: buffer big messages?
        self.stream.write(msg)?;
        self.stream.flush()
    }

    pub fn read(&mut self) -> io::Result<RawMsgBuffer> {
        let amount = self.stream.read(&mut self.buffer)?;
        if amount == 0 {
            Err(io::Error::from(ErrorKind::ConnectionAborted))
        } else {
            Ok(self.buffer[0..amount].to_vec())
        }
    }
}

impl Server for SocketServer {
    fn run(&mut self) -> ServerChanges {
        let outputs = self.pending_outputs.take().unwrap_or(vec![]);
        self.read_write(outputs)
    }

    fn output(&mut self, connection_id: ConnectionId, msg: RawMsgBuffer) {
        self.pending_outputs
            .get_or_insert(vec![])
            .push(ServerOutput { connection_id, msg });
    }

    fn disconnect(&mut self, _connection_id: ConnectionId) {
        unimplemented!()
    }
}

impl SocketServer {
    fn next_connection_id(&mut self) -> ConnectionId {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        id
    }

    pub fn new(port: u32) -> Self {
        let mut ins = SocketServer {
            next_connection_id: 0,
            connections: HashMap::new(),
            listener: None,
            pending_outputs: None,
        };

        ins.start(port);
        ins
    }

    fn start(&mut self, port: u32) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        listener.set_nonblocking(true).expect("non blocking failed");
        // accept connections and process them, spawning a new thread for each one
        info!("server - listening on port 3333");

        self.listener = Some(listener);
    }

    fn read_write(&mut self, pending_outputs: Vec<ServerOutput>) -> ServerChanges {
        let mut connects: Vec<ConnectionId> = vec![];
        let mut disconnects: Vec<ConnectionId> = vec![];
        let mut pending_inputs: Vec<ServerInput> = vec![];

        let listener = self.listener.as_ref().expect("server not started!");

        // accept new connections
        if let Ok((stream, addr)) = listener.accept() {
            let id = self.next_connection_id();

            info!(
                "new connection ({}) {:?}, total connections {}",
                addr,
                id,
                self.connections.len()
            );
            stream
                .set_nonblocking(true)
                .expect(format!("failed to set non_blocking stream for {:?}", id).as_str());

            // connection succeeded
            let connection = Connection::new(id, stream);

            connects.push(id);
            self.connections.insert(connection.id, connection);
        }

        // handle inputs
        for (connection_id, stream) in &mut self.connections {
            match stream.read() {
                Ok(msg) => pending_inputs.push(ServerInput {
                    connection_id: *connection_id,
                    msg,
                }),
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => (),
                Err(e) => {
                    warn!("{:?} failed: {}", connection_id, e);
                    disconnects.push(*connection_id)
                }
            }
        }

        // handle outputs
        for output in pending_outputs {
            trace!(
                "{:?} sending '{}'",
                output.connection_id,
                SocketServer::clean_output_to_log(&output.msg)
            );

            match self.connections.get_mut(&output.connection_id) {
                Some(connection) => {
                    if let Err(err) = connection.write(&output.msg) {
                        warn!("{:?} failed: {}", connection.id, err);
                        disconnects.push(connection.id);
                    }
                }
                None => error!("{:?} not found", output.connection_id),
            }
        }

        // remove broken connections
        for connection in &disconnects {
            self.connections.remove(connection);

            info!(
                "{:?} removed, total connections {}",
                connection,
                self.connections.len()
            );
        }

        ServerChanges {
            connects,
            disconnects,
            inputs: pending_inputs,
        }
    }

    fn clean_output_to_log(msg: &RawMsg) -> String {
        base64::encode(msg)
    }
}
