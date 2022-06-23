mod sb_packet_block_dig { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketBlockDig ; impl Packet for SbPacketBlockDig { type PacketIDType = i32 ; type PacketContent = PacketBlockDigContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 26 } } pub struct PacketBlockDigContent { status: i8 ,

location: minecraft_data::common::data::position::Position ,

face: i8 ,

 } impl PacketContent for PacketBlockDigContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.status.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.face.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let status : i8 = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let face : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { status, location, face } ) } }

 }

 pub use sb_packet_block_dig ::*;
