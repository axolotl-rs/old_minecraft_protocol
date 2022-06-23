mod cb_packet_window_items { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWindowItems ; impl Packet for CbPacketWindowItems { type PacketIDType = i32 ; type PacketContent = PacketWindowItemsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 20 } } pub struct PacketWindowItemsContent { window_id: u8 ,

state_id: minecraft_data::common::data::VarInt ,

items: PacketWindowItemsContentArray ,

carried_item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketWindowItemsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.state_id.write(writer)?;;

total_bytes += self.items.write(writer)?;;

total_bytes += self.carried_item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

let state_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let items : PacketWindowItemsContentArray = PacketContent :: read ( reader ) ?;;

let carried_item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, state_id, items, carried_item } ) } } pub type PacketWindowItemsContentArray = Vec <crate::protocol::types::slot::Slot >;

 }

 pub use cb_packet_window_items ::*;
