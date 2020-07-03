use crate::Request::GameStatus;

pub mod client;
pub mod packages;
pub mod schemas;

/// Implement a game game using the library

pub type UserId = u16;

///
/// Game logic
///

#[derive(Debug)]
pub enum Error {
    Unknown(String),
    Disconnect,
    IOError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{:?}", e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Unknown(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ObjId = u32;

#[derive(Debug, Clone)]
pub enum GameEvent {
    GameStatusIdle,
    GameStatusRunning,
    GameStarted,
    FullStateResponse,
    CreateObj { id: ObjId, x: f32, y: f32 },
    MoveObj { obj_id: ObjId, x: f32, y: f32 },
}

#[derive(Debug, Clone)]
pub enum Request {
    StartGame,
    GameStatus,
    GetAll,
    SetInputAxis { hor: f32, ver: f32 },
}

#[derive(Debug)]
enum GameState {
    Idle,
    Running { tick: u64 },
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    pending_events: Vec<GameEvent>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::Idle,
            pending_events: Default::default(),
        }
    }

    pub fn start_game(&mut self) -> Result<()> {
        self.state = GameState::Running { tick: 0 };
        self.pending_events.push(GameEvent::GameStarted);
        Ok(())
    }

    pub fn connect(&mut self) -> UserId {
        0
    }

    pub fn handle_requests(&mut self, requests: Vec<Request>) -> Result<()> {
        for request in requests {
            match request {
                Request::GameStatus => match self.state {
                    GameState::Idle => self.pending_events.push(GameEvent::GameStatusIdle),
                    GameState::Running { .. } => {
                        self.pending_events.push(GameEvent::GameStatusRunning)
                    }
                },

                Request::StartGame => match self.state {
                    GameState::Idle => self.start_game()?,
                    GameState::Running { .. } => {
                        // TODO: return error to client
                    }
                },

                Request::GetAll => {
                    // TODO: implement
                }

                other => return Err(format!("unsupported request {:?}", other).into()),
            }
        }

        Ok(())
    }

    pub fn take(&mut self) -> Result<Vec<GameEvent>> {
        match &mut self.state {
            GameState::Idle => {}

            GameState::Running { tick } => {
                *tick += 1;

                if *tick == 25 {
                    self.pending_events.push(GameEvent::CreateObj {
                        id: 0,
                        x: 0.0,
                        y: 0.0,
                    })
                } else if *tick > 25 {
                    self.pending_events.push(GameEvent::MoveObj {
                        obj_id: 0,
                        x: (*tick as f32 - 20.0) / 10.0,
                        y: 0.0,
                    });
                }
            }
        }

        Ok(std::mem::replace(&mut self.pending_events, Vec::new()))
    }
}
