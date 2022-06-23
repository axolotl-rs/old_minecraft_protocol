mod sb_packet_update_command_block_minecart { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateCommandBlockMinecart ; impl Packet for SbPacketUpdateCommandBlockMinecart { type PacketIDType = i32 ; type PacketContent = PacketUpdateCommandBlockMinecartContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 39 } } pub struct PacketUpdateCommandBlockMinecartContent { entity_id: minecraft_data::common::data::VarInt ,

command: String ,

track_output: bool ,

 } impl PacketContent for PacketUpdateCommandBlockMinecartContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.command.write(writer)?;;

total_bytes += self.track_output.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let command : String = PacketContent :: read ( reader ) ?;;

let track_output : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, command, track_output } ) } }

 }

 pub use sb_packet_update_command_block_minecart ::*;
