mod cb_packet_named_entity_spawn { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketNamedEntitySpawn ; impl Packet for CbPacketNamedEntitySpawn { type PacketIDType = i32 ; type PacketContent = void ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 4 } } }

 pub use cb_packet_named_entity_spawn ::*;
