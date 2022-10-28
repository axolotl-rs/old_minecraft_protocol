
/// The Non Version Specific Packet Input
#[derive(Debug, Clone, PartialEq)]
pub enum ClientBoundPlay {
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    }
}