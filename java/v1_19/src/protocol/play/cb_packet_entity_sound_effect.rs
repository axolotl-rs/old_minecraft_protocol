mod cb_packet_entity_sound_effect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntitySoundEffect ; impl Packet for CbPacketEntitySoundEffect { type PacketIDType = i32 ; type PacketContent = PacketEntitySoundEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 92 } } pub struct PacketEntitySoundEffectContent { sound_id: minecraft_data::common::data::VarInt ,

sound_category: minecraft_data::common::data::VarInt ,

entity_id: minecraft_data::common::data::VarInt ,

volume: f32 ,

pitch: f32 ,

 } impl PacketContent for PacketEntitySoundEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sound_id.write(writer)?;;

total_bytes += self.sound_category.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.volume.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let sound_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let sound_category : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let volume : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sound_id, sound_category, entity_id, volume, pitch } ) } }

 }

 pub use cb_packet_entity_sound_effect ::*;
