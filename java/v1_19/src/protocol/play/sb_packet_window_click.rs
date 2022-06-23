mod sb_packet_window_click { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketWindowClick ; impl Packet for SbPacketWindowClick { type PacketIDType = i32 ; type PacketContent = PacketWindowClickContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 8 } } pub struct PacketWindowClickContent { window_id: u8 ,

state_id: minecraft_data::common::data::VarInt ,

slot: i16 ,

mouse_button: i8 ,

mode: minecraft_data::common::data::VarInt ,

changed_slots: PacketWindowClickContentArray ,

cursor_item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketWindowClickContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.state_id.write(writer)?;;

total_bytes += self.slot.write(writer)?;;

total_bytes += self.mouse_button.write(writer)?;;

total_bytes += self.mode.write(writer)?;;

total_bytes += self.changed_slots.write(writer)?;;

total_bytes += self.cursor_item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

let state_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let slot : i16 = PacketContent :: read ( reader ) ?;;

let mouse_button : i8 = PacketContent :: read ( reader ) ?;;

let mode : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let changed_slots : PacketWindowClickContentArray = PacketContent :: read ( reader ) ?;;

let cursor_item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, state_id, slot, mouse_button, mode, changed_slots, cursor_item } ) } } pub type PacketWindowClickContentArray = Vec <PacketWindowClickContentArrayContent >; pub struct PacketWindowClickContentArrayContent { location: i16 ,

item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketWindowClickContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : i16 = PacketContent :: read ( reader ) ?;;

let item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, item } ) } }

 }

 pub use sb_packet_window_click ::*;
