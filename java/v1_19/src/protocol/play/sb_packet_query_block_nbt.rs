mod sb_packet_query_block_nbt { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketQueryBlockNbt ; impl Packet for SbPacketQueryBlockNbt { type PacketIDType = i32 ; type PacketContent = PacketQueryBlockNbtContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketQueryBlockNbtContent { transaction_id: minecraft_data::common::data::VarInt ,

location: minecraft_data::common::data::position::Position ,

 } impl PacketContent for PacketQueryBlockNbtContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let transaction_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, location } ) } }

 }

 pub use sb_packet_query_block_nbt ::*;
