#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientBoundStatus{
    Response {
        response: String,
    },
    Pong {
        payload: i64,
    },
}

