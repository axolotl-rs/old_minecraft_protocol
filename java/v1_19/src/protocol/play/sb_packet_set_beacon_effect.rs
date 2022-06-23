mod sb_packet_set_beacon_effect { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetBeaconEffect ; impl Packet for SbPacketSetBeaconEffect { type PacketIDType = i32 ; type PacketContent = PacketSetBeaconEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 36 } } pub struct PacketSetBeaconEffectContent { primary_effect: minecraft_data::common::data::VarInt ,

secondary_effect: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketSetBeaconEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.primary_effect.write(writer)?;;

total_bytes += self.secondary_effect.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let primary_effect : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let secondary_effect : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { primary_effect, secondary_effect } ) } }

 }

 pub use sb_packet_set_beacon_effect ::*;
