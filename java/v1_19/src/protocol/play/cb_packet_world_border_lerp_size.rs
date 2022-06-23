mod cb_packet_world_border_lerp_size { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderLerpSize ; impl Packet for CbPacketWorldBorderLerpSize { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderLerpSizeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 67 } } pub struct PacketWorldBorderLerpSizeContent { old_diameter: f64 ,

new_diameter: f64 ,

speed: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketWorldBorderLerpSizeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.old_diameter.write(writer)?;;

total_bytes += self.new_diameter.write(writer)?;;

total_bytes += self.speed.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let old_diameter : f64 = PacketContent :: read ( reader ) ?;;

let new_diameter : f64 = PacketContent :: read ( reader ) ?;;

let speed : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { old_diameter, new_diameter, speed } ) } }

 }

 pub use cb_packet_world_border_lerp_size ::*;
