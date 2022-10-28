use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSpawnEntityExperienceOrb ; impl Packet for CbPacketSpawnEntityExperienceOrb { type PacketIDType = i32 ; type PacketContent = PacketSpawnEntityExperienceOrbContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketSpawnEntityExperienceOrbContent { pub entity_id: minecraft_protocol::data::VarInt ,

pub x: f64 ,

pub y: f64 ,

pub z: f64 ,

pub count: i16 ,

 } impl PacketContent for PacketSpawnEntityExperienceOrbContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.count.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let count : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, x, y, z, count } ) } }
