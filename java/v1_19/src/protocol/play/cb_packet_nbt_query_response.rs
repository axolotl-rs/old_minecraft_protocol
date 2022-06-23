mod cb_packet_nbt_query_response { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketNbtQueryResponse ; impl Packet for CbPacketNbtQueryResponse { type PacketIDType = i32 ; type PacketContent = PacketNbtQueryResponseContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 96 } } pub struct PacketNbtQueryResponseContent { transaction_id: minecraft_data::common::data::VarInt ,

nbt: minecraft_data::common::data::nbt::OptionalNbt ,

 } impl PacketContent for PacketNbtQueryResponseContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.nbt.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let transaction_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let nbt : minecraft_data::common::data::nbt::OptionalNbt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, nbt } ) } }

 }

 pub use cb_packet_nbt_query_response ::*;
