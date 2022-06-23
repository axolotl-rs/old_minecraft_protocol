mod sb_packet_teleport_confirm { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketTeleportConfirm ; impl Packet for SbPacketTeleportConfirm { type PacketIDType = i32 ; type PacketContent = PacketTeleportConfirmContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketTeleportConfirmContent { teleport_id: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketTeleportConfirmContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.teleport_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let teleport_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { teleport_id } ) } }

 }

 pub use sb_packet_teleport_confirm ::*;
