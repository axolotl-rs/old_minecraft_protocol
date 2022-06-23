mod cb_packet_enter_combat_event { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEnterCombatEvent ; impl Packet for CbPacketEnterCombatEvent { type PacketIDType = i32 ; type PacketContent = PacketEnterCombatEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 52 } } pub struct PacketEnterCombatEventContent { } impl PacketContent for PacketEnterCombatEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { Ok ( Self { } ) } }

 }

 pub use cb_packet_enter_combat_event ::*;
