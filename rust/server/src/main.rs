extern crate base64;

use crate::server::{Server, ServerInput};
use game::packages::package_buffer::PackageBuffer;
use game::Game;
use std::collections::HashMap;
use std::io::repeat;

mod server;

fn main() {
    let port = 3333;
    let mut server = server::server_socket::SocketServer::new(port);
    let mut tick: u64 = 0;
    let mut game = Game::new();
    let mut users = HashMap::new();

    loop {
        let changes = server.run();

        for connection_id in changes.connects {
            println!("{:?} connects", connection_id);
            let user_id = game.connect();
            users.insert(connection_id, user_id);
        }

        for connection_id in changes.disconnects {
            println!("{:?} disconnects", connection_id);
            users.remove(&connection_id);
        }

        for ServerInput { connection_id, msg } in changes.inputs {
            println!("{:?} receive {:?}", connection_id, msg);
        }

        let events = game.take().unwrap();
        let bytes = game::schemas::serialize_game_events(events).unwrap();
        let bytes = PackageBuffer::pack(bytes);

        for (connection_id, _) in &users {
            server.output(*connection_id, bytes.clone());
        }

        std::thread::sleep(::std::time::Duration::from_millis(100));
        tick += 1;

        if tick % 100 == 0 {
            println!(".");
        }
    }
}
