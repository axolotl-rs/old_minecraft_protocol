mod cb_packet_spawn_position { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSpawnPosition ; impl Packet for CbPacketSpawnPosition { type PacketIDType = i32 ; type PacketContent = PacketSpawnPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 75 } } pub struct PacketSpawnPositionContent { location: minecraft_data::common::data::position::Position ,

angle: f32 ,

 } impl PacketContent for PacketSpawnPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.angle.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let angle : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, angle } ) } }

 }

 pub use cb_packet_spawn_position ::*;
