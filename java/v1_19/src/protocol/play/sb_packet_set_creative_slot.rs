mod sb_packet_set_creative_slot { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetCreativeSlot ; impl Packet for SbPacketSetCreativeSlot { type PacketIDType = i32 ; type PacketContent = PacketSetCreativeSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 40 } } pub struct PacketSetCreativeSlotContent { slot: i16 ,

item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketSetCreativeSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot.write(writer)?;;

total_bytes += self.item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let slot : i16 = PacketContent :: read ( reader ) ?;;

let item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot, item } ) } }

 }

 pub use sb_packet_set_creative_slot ::*;
