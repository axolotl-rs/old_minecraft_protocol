mod cb_packet_spawn_entity_experience_orb { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSpawnEntityExperienceOrb ; impl Packet for CbPacketSpawnEntityExperienceOrb { type PacketIDType = i32 ; type PacketContent = PacketSpawnEntityExperienceOrbContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketSpawnEntityExperienceOrbContent { entity_id: minecraft_data::common::data::VarInt ,

x: f64 ,

y: f64 ,

z: f64 ,

count: i16 ,

 } impl PacketContent for PacketSpawnEntityExperienceOrbContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.count.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let count : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, x, y, z, count } ) } }

 }

 pub use cb_packet_spawn_entity_experience_orb ::*;
