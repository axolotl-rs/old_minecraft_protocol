mod cb_packet_set_slot { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetSlot ; impl Packet for CbPacketSetSlot { type PacketIDType = i32 ; type PacketContent = PacketSetSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 22 } } pub struct PacketSetSlotContent { window_id: i8 ,

state_id: minecraft_data::common::data::VarInt ,

slot: i16 ,

item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketSetSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.state_id.write(writer)?;;

total_bytes += self.slot.write(writer)?;;

total_bytes += self.item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let state_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let slot : i16 = PacketContent :: read ( reader ) ?;;

let item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, state_id, slot, item } ) } }

 }

 pub use cb_packet_set_slot ::*;
