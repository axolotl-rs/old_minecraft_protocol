mod cb_packet_update_health { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateHealth ; impl Packet for CbPacketUpdateHealth { type PacketIDType = i32 ; type PacketContent = PacketUpdateHealthContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 82 } } pub struct PacketUpdateHealthContent { health: f32 ,

food: minecraft_data::common::data::VarInt ,

food_saturation: f32 ,

 } impl PacketContent for PacketUpdateHealthContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.health.write(writer)?;;

total_bytes += self.food.write(writer)?;;

total_bytes += self.food_saturation.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let health : f32 = PacketContent :: read ( reader ) ?;;

let food : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let food_saturation : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { health, food, food_saturation } ) } }

 }

 pub use cb_packet_update_health ::*;
