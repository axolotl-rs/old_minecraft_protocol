use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCamera ; impl Packet for CbPacketCamera { type PacketIDType = i32 ; type PacketContent = PacketCameraContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 71 } } pub struct PacketCameraContent { pub camera_id: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketCameraContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.camera_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let camera_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { camera_id } ) } }
