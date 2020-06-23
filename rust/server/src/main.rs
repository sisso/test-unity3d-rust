extern crate base64;

use crate::server::{Server, ServerInput};
use std::io::repeat;

mod server;

fn main() {
    let mut server = server::server_socket::SocketServer::new();

    let mut tick: u64 = 0;

    loop {
        let changes = server.run();

        for connection_id in changes.connects {
            println!("{:?} connects", connection_id);
        }

        for connection_id in changes.disconnects {
            println!("{:?} disconnects", connection_id);
        }

        for ServerInput { connection_id, msg } in changes.inputs {
            println!("{:?} sends {:?}", connection_id, msg);
        }

        std::thread::sleep(::std::time::Duration::from_millis(100));
        tick += 1;

        if tick % 100 == 0 {
            println!(".");
        }
    }
}
