#[derive(Debug, Clone, PartialEq)]
pub enum ServerBoundPlay {
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    }
}