mod cb_packet_block_action { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockAction ; impl Packet for CbPacketBlockAction { type PacketIDType = i32 ; type PacketContent = PacketBlockActionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 11 } } pub struct PacketBlockActionContent { location: minecraft_data::common::data::position::Position ,

byte_1: u8 ,

byte_2: u8 ,

block_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketBlockActionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.byte_1.write(writer)?;;

total_bytes += self.byte_2.write(writer)?;;

total_bytes += self.block_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let byte_1 : u8 = PacketContent :: read ( reader ) ?;;

let byte_2 : u8 = PacketContent :: read ( reader ) ?;;

let block_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, byte_1, byte_2, block_id } ) } }

 }

 pub use cb_packet_block_action ::*;
