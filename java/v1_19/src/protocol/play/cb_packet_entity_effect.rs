mod cb_packet_entity_effect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityEffect ; impl Packet for CbPacketEntityEffect { type PacketIDType = i32 ; type PacketContent = PacketEntityEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 101 } } pub struct PacketEntityEffectContent { entity_id: minecraft_data::common::data::VarInt ,

effect_id: minecraft_data::common::data::VarInt ,

amplifier: i8 ,

duration: minecraft_data::common::data::VarInt ,

hide_particles: i8 ,

 } impl PacketContent for PacketEntityEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.effect_id.write(writer)?;;

total_bytes += self.amplifier.write(writer)?;;

total_bytes += self.duration.write(writer)?;;

total_bytes += self.hide_particles.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let effect_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let amplifier : i8 = PacketContent :: read ( reader ) ?;;

let duration : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let hide_particles : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, effect_id, amplifier, duration, hide_particles } ) } }

 }

 pub use cb_packet_entity_effect ::*;
