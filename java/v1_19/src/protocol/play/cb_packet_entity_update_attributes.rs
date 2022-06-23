mod cb_packet_entity_update_attributes { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityUpdateAttributes ; impl Packet for CbPacketEntityUpdateAttributes { type PacketIDType = i32 ; type PacketContent = void ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 100 } } }

 pub use cb_packet_entity_update_attributes ::*;
