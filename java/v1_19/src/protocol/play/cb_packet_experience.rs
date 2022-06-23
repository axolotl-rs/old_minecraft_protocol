mod cb_packet_experience { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketExperience ; impl Packet for CbPacketExperience { type PacketIDType = i32 ; type PacketContent = PacketExperienceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 81 } } pub struct PacketExperienceContent { experience_bar: f32 ,

level: minecraft_data::common::data::VarInt ,

total_experience: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketExperienceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.experience_bar.write(writer)?;;

total_bytes += self.level.write(writer)?;;

total_bytes += self.total_experience.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let experience_bar : f32 = PacketContent :: read ( reader ) ?;;

let level : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let total_experience : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { experience_bar, level, total_experience } ) } }

 }

 pub use cb_packet_experience ::*;
