mod cb_packet_tab_complete { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTabComplete ; impl Packet for CbPacketTabComplete { type PacketIDType = i32 ; type PacketContent = PacketTabCompleteContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 17 } } pub struct PacketTabCompleteContent { transaction_id: minecraft_data::common::data::VarInt ,

start: minecraft_data::common::data::VarInt ,

length: minecraft_data::common::data::VarInt ,

matches: PacketTabCompleteContentArray ,

 } impl PacketContent for PacketTabCompleteContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.start.write(writer)?;;

total_bytes += self.length.write(writer)?;;

total_bytes += self.matches.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let transaction_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let start : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let length : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let matches : PacketTabCompleteContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, start, length, matches } ) } } pub type PacketTabCompleteContentArray = Vec <PacketTabCompleteContentArrayContent >; pub struct PacketTabCompleteContentArrayContent { match: String ,

tooltip: void ,

 } impl PacketContent for PacketTabCompleteContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.match.write(writer)?;;

total_bytes += self.tooltip.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let match : String = PacketContent :: read ( reader ) ?;;

let tooltip : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { match, tooltip } ) } }

 }

 pub use cb_packet_tab_complete ::*;
