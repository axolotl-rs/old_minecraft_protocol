use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketGenerateStructure ; impl Packet for SbPacketGenerateStructure { type PacketIDType = i32 ; type PacketContent = PacketGenerateStructureContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 14 } } pub struct PacketGenerateStructureContent { pub location: minecraft_data::data::position::Position ,

pub levels: minecraft_data::data::VarInt ,

pub keep_jigsaws: bool ,

 } impl PacketContent for PacketGenerateStructureContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.levels.write(writer)?;;

total_bytes += self.keep_jigsaws.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let levels : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let keep_jigsaws : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, levels, keep_jigsaws } ) } }
