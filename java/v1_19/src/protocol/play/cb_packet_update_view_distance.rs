use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateViewDistance ; impl Packet for CbPacketUpdateViewDistance { type PacketIDType = i32 ; type PacketContent = PacketUpdateViewDistanceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 74 } } pub struct PacketUpdateViewDistanceContent { pub view_distance: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketUpdateViewDistanceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.view_distance.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let view_distance : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { view_distance } ) } }
