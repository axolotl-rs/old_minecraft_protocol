use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntitySoundEffect ; impl Packet for CbPacketEntitySoundEffect { type PacketIDType = i32 ; type PacketContent = PacketEntitySoundEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 92 } } pub struct PacketEntitySoundEffectContent { sound_id: minecraft_data::data::VarInt ,

sound_category: minecraft_data::data::VarInt ,

entity_id: minecraft_data::data::VarInt ,

volume: f32 ,

pitch: f32 ,

 } impl PacketContent for PacketEntitySoundEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sound_id.write(writer)?;;

total_bytes += self.sound_category.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.volume.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let sound_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let sound_category : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let volume : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sound_id, sound_category, entity_id, volume, pitch } ) } }
