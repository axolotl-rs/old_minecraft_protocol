use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketInitializeWorldBorder ; impl Packet for CbPacketInitializeWorldBorder { type PacketIDType = i32 ; type PacketContent = PacketInitializeWorldBorderContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 32 } } pub struct PacketInitializeWorldBorderContent { pub x: f64 ,

pub z: f64 ,

pub old_diameter: f64 ,

pub new_diameter: f64 ,

pub speed: minecraft_protocol::data::VarInt ,

pub portal_teleport_boundary: minecraft_protocol::data::VarInt ,

pub warning_blocks: minecraft_protocol::data::VarInt ,

pub warning_time: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketInitializeWorldBorderContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.old_diameter.write(writer)?;;

total_bytes += self.new_diameter.write(writer)?;;

total_bytes += self.speed.write(writer)?;;

total_bytes += self.portal_teleport_boundary.write(writer)?;;

total_bytes += self.warning_blocks.write(writer)?;;

total_bytes += self.warning_time.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let old_diameter : f64 = PacketContent :: read ( reader ) ?;;

let new_diameter : f64 = PacketContent :: read ( reader ) ?;;

let speed : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let portal_teleport_boundary : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let warning_blocks : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let warning_time : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, z, old_diameter, new_diameter, speed, portal_teleport_boundary, warning_blocks, warning_time } ) } }
