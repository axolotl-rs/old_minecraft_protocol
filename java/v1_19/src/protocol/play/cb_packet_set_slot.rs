use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetSlot ; impl Packet for CbPacketSetSlot { type PacketIDType = i32 ; type PacketContent = PacketSetSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 22 } } pub struct PacketSetSlotContent { pub window_id: i8 ,

pub state_id: minecraft_protocol::data::VarInt ,

pub slot: i16 ,

pub item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketSetSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.state_id.write(writer)?;;

total_bytes += self.slot.write(writer)?;;

total_bytes += self.item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let state_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let slot : i16 = PacketContent :: read ( reader ) ?;;

let item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, state_id, slot, item } ) } }
