use crate::login::LoginPackets;
use crate::play::PlayPacket;
use crate::status::StatusPacket;
use minecraft_protocol::java::handshake::HandshakePackets;
use minecraft_protocol::{PacketHandler, Version};

pub mod protocol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod login;
pub mod play;
pub mod status;

#[derive(Debug)]
pub struct V119packetHandler;

impl PacketHandler for V119packetHandler {
    type HandleShakePacket = HandshakePackets;
    type LoginPacket = LoginPackets;
    type StatusPacket = StatusPacket;
    type PlayPacket = PlayPacket;

    fn get_version() -> Version {
        Version::Java(760)
    }
}
