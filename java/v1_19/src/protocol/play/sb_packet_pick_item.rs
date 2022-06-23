mod sb_packet_pick_item { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketPickItem ; impl Packet for SbPacketPickItem { type PacketIDType = i32 ; type PacketContent = PacketPickItemContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 23 } } pub struct PacketPickItemContent { slot: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketPickItemContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let slot : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot } ) } }

 }

 pub use sb_packet_pick_item ::*;
