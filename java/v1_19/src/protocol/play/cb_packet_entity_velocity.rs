mod cb_packet_entity_velocity { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityVelocity ; impl Packet for CbPacketEntityVelocity { type PacketIDType = i32 ; type PacketContent = PacketEntityVelocityContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 79 } } pub struct PacketEntityVelocityContent { entity_id: minecraft_data::common::data::VarInt ,

velocity_x: i16 ,

velocity_y: i16 ,

velocity_z: i16 ,

 } impl PacketContent for PacketEntityVelocityContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.velocity_x.write(writer)?;;

total_bytes += self.velocity_y.write(writer)?;;

total_bytes += self.velocity_z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let velocity_x : i16 = PacketContent :: read ( reader ) ?;;

let velocity_y : i16 = PacketContent :: read ( reader ) ?;;

let velocity_z : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, velocity_x, velocity_y, velocity_z } ) } }

 }

 pub use cb_packet_entity_velocity ::*;
