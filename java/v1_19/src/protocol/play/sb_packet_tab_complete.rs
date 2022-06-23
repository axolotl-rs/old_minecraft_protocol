mod sb_packet_tab_complete { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketTabComplete ; impl Packet for SbPacketTabComplete { type PacketIDType = i32 ; type PacketContent = PacketTabCompleteContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 6 } } pub struct PacketTabCompleteContent { transaction_id: minecraft_data::common::data::VarInt ,

text: String ,

 } impl PacketContent for PacketTabCompleteContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.text.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let transaction_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let text : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, text } ) } }

 }

 pub use sb_packet_tab_complete ::*;
