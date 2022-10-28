use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockChange ; impl Packet for CbPacketBlockChange { type PacketIDType = i32 ; type PacketContent = PacketBlockChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 12 } } pub struct PacketBlockChangeContent { pub location: minecraft_protocol::data::position::Position ,

pub data_type: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketBlockChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.data_type.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let data_type : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, data_type } ) } }
