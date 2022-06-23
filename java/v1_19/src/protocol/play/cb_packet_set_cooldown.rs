mod cb_packet_set_cooldown { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetCooldown ; impl Packet for CbPacketSetCooldown { type PacketIDType = i32 ; type PacketContent = PacketSetCooldownContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 23 } } pub struct PacketSetCooldownContent { item_id: minecraft_data::common::data::VarInt ,

cooldown_ticks: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketSetCooldownContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.item_id.write(writer)?;;

total_bytes += self.cooldown_ticks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let item_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let cooldown_ticks : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { item_id, cooldown_ticks } ) } }

 }

 pub use cb_packet_set_cooldown ::*;
