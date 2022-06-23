mod cb_packet_block_break_animation { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockBreakAnimation ; impl Packet for CbPacketBlockBreakAnimation { type PacketIDType = i32 ; type PacketContent = PacketBlockBreakAnimationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 9 } } pub struct PacketBlockBreakAnimationContent { entity_id: minecraft_data::common::data::VarInt ,

location: minecraft_data::common::data::position::Position ,

destroy_stage: i8 ,

 } impl PacketContent for PacketBlockBreakAnimationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.destroy_stage.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let destroy_stage : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, location, destroy_stage } ) } }

 }

 pub use cb_packet_block_break_animation ::*;
