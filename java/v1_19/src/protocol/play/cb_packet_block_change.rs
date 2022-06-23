mod cb_packet_block_change { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockChange ; impl Packet for CbPacketBlockChange { type PacketIDType = i32 ; type PacketContent = PacketBlockChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 12 } } pub struct PacketBlockChangeContent { location: minecraft_data::common::data::position::Position ,

data_type: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketBlockChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.data_type.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let data_type : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, data_type } ) } }

 }

 pub use cb_packet_block_change ::*;
