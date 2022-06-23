mod cb_packet_spawn_entity_living { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSpawnEntityLiving ; impl Packet for CbPacketSpawnEntityLiving { type PacketIDType = i32 ; type PacketContent = void ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 2 } } }

 pub use cb_packet_spawn_entity_living ::*;
