extern crate game;

use game::client::*;
use std::io::stdin;

fn main() -> std::io::Result<()> {
    let mut client = Client::connect("localhost:3333")?;

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        if buffer.starts_with("quit") {
            client.close()?;
            return Ok(());
        }

        if buffer != "\n" {
            client.push(buffer.into_bytes().to_vec());
        }

        client.tick()?;

        if let Some(bytes) = client.take() {
            println!("> ${:?}", bytes);
        }
    }
}