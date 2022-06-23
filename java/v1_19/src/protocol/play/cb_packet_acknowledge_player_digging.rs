mod cb_packet_acknowledge_player_digging { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAcknowledgePlayerDigging ; impl Packet for CbPacketAcknowledgePlayerDigging { type PacketIDType = i32 ; type PacketContent = PacketAcknowledgePlayerDiggingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 8 } } pub struct PacketAcknowledgePlayerDiggingContent { location: minecraft_data::common::data::position::Position ,

block: minecraft_data::common::data::VarInt ,

status: minecraft_data::common::data::VarInt ,

successful: bool ,

 } impl PacketContent for PacketAcknowledgePlayerDiggingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.block.write(writer)?;;

total_bytes += self.status.write(writer)?;;

total_bytes += self.successful.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let block : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let status : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let successful : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, block, status, successful } ) } }

 }

 pub use cb_packet_acknowledge_player_digging ::*;
