// TODO: maybe this is GameAdapter?

/// Implement a game game using the library

pub type UserId = u16;

///
/// Game logic
///

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ObjId = u32;

#[derive(Debug, Clone)]
pub enum Message {
    StartGame,
    CreateObj { id: ObjId },
    MoveObj { obj_id: ObjId, x: f32, y: f32 },
    SetInputAxis { hor: f32, ver: f32 },
}

#[derive(Debug)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn connect(&mut self) -> UserId {
        0
    }

    pub fn take(&mut self, user_id: UserId) -> Vec<Message> {
        Vec::new()
    }
}
