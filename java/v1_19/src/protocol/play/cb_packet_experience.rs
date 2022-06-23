use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketExperience ; impl Packet for CbPacketExperience { type PacketIDType = i32 ; type PacketContent = PacketExperienceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 81 } } pub struct PacketExperienceContent { pub experience_bar: f32 ,

pub level: minecraft_data::data::VarInt ,

pub total_experience: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketExperienceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.experience_bar.write(writer)?;;

total_bytes += self.level.write(writer)?;;

total_bytes += self.total_experience.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let experience_bar : f32 = PacketContent :: read ( reader ) ?;;

let level : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let total_experience : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { experience_bar, level, total_experience } ) } }
