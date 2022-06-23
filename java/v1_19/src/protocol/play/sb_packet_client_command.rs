mod sb_packet_client_command { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketClientCommand ; impl Packet for SbPacketClientCommand { type PacketIDType = i32 ; type PacketContent = PacketClientCommandContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 4 } } pub struct PacketClientCommandContent { action_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketClientCommandContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.action_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let action_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { action_id } ) } }

 }

 pub use sb_packet_client_command ::*;
