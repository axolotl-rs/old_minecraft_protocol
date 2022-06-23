mod sb_packet_update_command_block { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateCommandBlock ; impl Packet for SbPacketUpdateCommandBlock { type PacketIDType = i32 ; type PacketContent = PacketUpdateCommandBlockContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 38 } } pub struct PacketUpdateCommandBlockContent { location: minecraft_data::common::data::position::Position ,

command: String ,

mode: minecraft_data::common::data::VarInt ,

flags: u8 ,

 } impl PacketContent for PacketUpdateCommandBlockContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.command.write(writer)?;;

total_bytes += self.mode.write(writer)?;;

total_bytes += self.flags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let command : String = PacketContent :: read ( reader ) ?;;

let mode : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let flags : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, command, mode, flags } ) } }

 }

 pub use sb_packet_update_command_block ::*;
