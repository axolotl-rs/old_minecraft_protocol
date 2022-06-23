mod cb_packet_open_window { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenWindow ; impl Packet for CbPacketOpenWindow { type PacketIDType = i32 ; type PacketContent = PacketOpenWindowContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 46 } } pub struct PacketOpenWindowContent { window_id: minecraft_data::common::data::VarInt ,

inventory_type: minecraft_data::common::data::VarInt ,

window_title: String ,

 } impl PacketContent for PacketOpenWindowContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.inventory_type.write(writer)?;;

total_bytes += self.window_title.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let inventory_type : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let window_title : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, inventory_type, window_title } ) } }

 }

 pub use cb_packet_open_window ::*;
