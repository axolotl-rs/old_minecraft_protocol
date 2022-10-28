use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenWindow ; impl Packet for CbPacketOpenWindow { type PacketIDType = i32 ; type PacketContent = PacketOpenWindowContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 46 } } pub struct PacketOpenWindowContent { pub window_id: minecraft_protocol::data::VarInt ,

pub inventory_type: minecraft_protocol::data::VarInt ,

pub window_title: String ,

 } impl PacketContent for PacketOpenWindowContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.inventory_type.write(writer)?;;

total_bytes += self.window_title.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let inventory_type : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let window_title : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, inventory_type, window_title } ) } }
