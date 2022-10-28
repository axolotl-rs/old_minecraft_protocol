use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateViewPosition ; impl Packet for CbPacketUpdateViewPosition { type PacketIDType = i32 ; type PacketContent = PacketUpdateViewPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 73 } } pub struct PacketUpdateViewPositionContent { pub chunk_x: minecraft_protocol::data::VarInt ,

pub chunk_z: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketUpdateViewPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_x.write(writer)?;;

total_bytes += self.chunk_z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let chunk_x : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let chunk_z : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_x, chunk_z } ) } }
