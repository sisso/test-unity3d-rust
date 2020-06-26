pub mod ffi_utils;

use crate::client::Client;
use crate::game::GameEvent::CreateObj;
use crate::game::{Game, GameEvent, Result, UserId};
use flatbuffers::FlatBufferBuilder;
use game::schemas::{ffi_requests, ffi_responses, RawMsg, RawMsgBuffer};

#[derive(Debug)]
enum RunMode {
    Embedded { game: Game },
    Server { client: Client },
}

#[derive(Debug)]
pub struct FfiContext {
    mode: RunMode,
}

impl<'a> FfiContext {
    pub fn new(address: Option<&str>) -> Self {
        let mode = match address {
            Some(address) => {
                // TODO: should not throw errors here
                let client = Client::new(address);
                RunMode::Server { client }
            },
            None => RunMode::Embedded { game: Game::new() },
        };

        FfiContext { mode }
    }

    pub fn push(&mut self, bytes: &RawMsg) -> Result<()> {
        unimplemented!()
    }

    // TODO: receive a closure?
    pub fn take(&mut self) -> Result<Option<RawMsgBuffer>> {
        match &mut self.mode {
            RunMode::Embedded { game } => {
                let game_responses = game.take();
                game::schemas::serialize_game_events(game_responses)
                    .map(|bytes| Some(bytes))
            }

            RunMode::Server { client } =>
                client.take_responses()
            ,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
