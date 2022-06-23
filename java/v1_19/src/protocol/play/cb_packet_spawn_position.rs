use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSpawnPosition ; impl Packet for CbPacketSpawnPosition { type PacketIDType = i32 ; type PacketContent = PacketSpawnPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 75 } } pub struct PacketSpawnPositionContent { location: minecraft_data::data::position::Position ,

angle: f32 ,

 } impl PacketContent for PacketSpawnPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.angle.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let angle : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, angle } ) } }
