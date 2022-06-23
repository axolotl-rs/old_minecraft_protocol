/// The common version types
pub mod common;


mod protocol_verison;
/// Contains Version types that can not be auto generated.
pub mod versions;

pub use protocol_verison::{JavaProtocolVersion, ProtocolVersion};
