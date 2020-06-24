use crate::GameEvent;
use crate::packages::package_buffer::Package;

/// Serialize a game event to be transfer in network compact format
pub fn serialize(e: GameEvent) -> Package {
    unimplemented!()
}

/// Parse a network compact format into a game event
pub fn parse(package: Package) -> GameEvent {
    unimplemented!()
}