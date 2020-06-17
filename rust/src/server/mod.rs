#[derive(Debug, Clone)]
pub struct Server {
    state: u32,
}

impl Server {}

impl Default for Server {
    fn default() -> Self {
        Server { state: 0 }
    }
}
