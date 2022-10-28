pub mod data;
pub mod packets;

pub mod protocol;
/// The common version types
mod protocol_verison;
pub use protocol_verison::{JavaProtocolVersion, ProtocolVersion};

pub trait Variation {
    type Metadata;
    type DisplayName;

    fn metadata(&self) -> Self::Metadata;

    fn display_name(&self) -> Self::DisplayName;
}

#[test]
pub fn test() {}
