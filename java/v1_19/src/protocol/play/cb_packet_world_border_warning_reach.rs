mod cb_packet_world_border_warning_reach { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderWarningReach ; impl Packet for CbPacketWorldBorderWarningReach { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderWarningReachContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 70 } } pub struct PacketWorldBorderWarningReachContent { warning_blocks: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketWorldBorderWarningReachContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.warning_blocks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let warning_blocks : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { warning_blocks } ) } }

 }

 pub use cb_packet_world_border_warning_reach ::*;
