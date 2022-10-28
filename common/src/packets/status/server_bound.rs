#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServerBoundStatus {
    Request,
    Ping {
        payload: i64,
    },
}