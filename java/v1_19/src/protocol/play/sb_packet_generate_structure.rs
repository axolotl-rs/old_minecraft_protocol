mod sb_packet_generate_structure { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketGenerateStructure ; impl Packet for SbPacketGenerateStructure { type PacketIDType = i32 ; type PacketContent = PacketGenerateStructureContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 14 } } pub struct PacketGenerateStructureContent { location: minecraft_data::common::data::position::Position ,

levels: minecraft_data::common::data::VarInt ,

keep_jigsaws: bool ,

 } impl PacketContent for PacketGenerateStructureContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.levels.write(writer)?;;

total_bytes += self.keep_jigsaws.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let levels : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let keep_jigsaws : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, levels, keep_jigsaws } ) } }

 }

 pub use sb_packet_generate_structure ::*;
