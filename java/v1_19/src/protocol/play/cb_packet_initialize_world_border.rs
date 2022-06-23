mod cb_packet_initialize_world_border { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketInitializeWorldBorder ; impl Packet for CbPacketInitializeWorldBorder { type PacketIDType = i32 ; type PacketContent = PacketInitializeWorldBorderContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 32 } } pub struct PacketInitializeWorldBorderContent { x: f64 ,

z: f64 ,

old_diameter: f64 ,

new_diameter: f64 ,

speed: minecraft_data::common::data::VarInt ,

portal_teleport_boundary: minecraft_data::common::data::VarInt ,

warning_blocks: minecraft_data::common::data::VarInt ,

warning_time: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketInitializeWorldBorderContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.old_diameter.write(writer)?;;

total_bytes += self.new_diameter.write(writer)?;;

total_bytes += self.speed.write(writer)?;;

total_bytes += self.portal_teleport_boundary.write(writer)?;;

total_bytes += self.warning_blocks.write(writer)?;;

total_bytes += self.warning_time.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let old_diameter : f64 = PacketContent :: read ( reader ) ?;;

let new_diameter : f64 = PacketContent :: read ( reader ) ?;;

let speed : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let portal_teleport_boundary : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let warning_blocks : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let warning_time : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, z, old_diameter, new_diameter, speed, portal_teleport_boundary, warning_blocks, warning_time } ) } }

 }

 pub use cb_packet_initialize_world_border ::*;
