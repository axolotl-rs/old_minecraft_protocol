mod cb_packet_sound_effect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSoundEffect ; impl Packet for CbPacketSoundEffect { type PacketIDType = i32 ; type PacketContent = PacketSoundEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 93 } } pub struct PacketSoundEffectContent { sound_id: minecraft_data::common::data::VarInt ,

sound_category: minecraft_data::common::data::VarInt ,

x: i32 ,

y: i32 ,

z: i32 ,

volume: f32 ,

pitch: f32 ,

 } impl PacketContent for PacketSoundEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sound_id.write(writer)?;;

total_bytes += self.sound_category.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.volume.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let sound_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let sound_category : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : i32 = PacketContent :: read ( reader ) ?;;

let y : i32 = PacketContent :: read ( reader ) ?;;

let z : i32 = PacketContent :: read ( reader ) ?;;

let volume : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sound_id, sound_category, x, y, z, volume, pitch } ) } }

 }

 pub use cb_packet_sound_effect ::*;
