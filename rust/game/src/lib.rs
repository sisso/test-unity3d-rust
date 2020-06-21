pub mod schemas;

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
pub enum GameEvent {
    CreateObj { id: ObjId, x: f32, y: f32 },
    MoveObj { obj_id: ObjId, x: f32, y: f32 },
}

#[derive(Debug, Clone)]
pub enum Requests {
    SetInputAxis { hor: f32, ver: f32 },
}

#[derive(Debug)]
pub struct Game {
    state: u32,
}

impl Game {
    pub fn new() -> Self {
        Game { state: 0 }
    }

    pub fn connect(&mut self) -> UserId {
        0
    }

    pub fn take(&mut self) -> Vec<GameEvent> {
        let mut result = vec![];

        self.state += 1;

        if (self.state == 50) {
            result.push(GameEvent::CreateObj {
                id: 0,
                x: 0.0,
                y: 0.0,
            })
        } else if (self.state > 50) {
            result.push(GameEvent::MoveObj {
                obj_id: 0,
                x: (self.state as f32 - 50.0) / 100.0,
                y: 0.0,
            });
        }

        result
    }
}
