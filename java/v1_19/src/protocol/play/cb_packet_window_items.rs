use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWindowItems ; impl Packet for CbPacketWindowItems { type PacketIDType = i32 ; type PacketContent = PacketWindowItemsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 20 } } pub struct PacketWindowItemsContent { pub window_id: u8 ,

pub state_id: minecraft_data::data::VarInt ,

pub items: PacketWindowItemsContentArray ,

pub carried_item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketWindowItemsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.state_id.write(writer)?;;

total_bytes += self.items.write(writer)?;;

total_bytes += self.carried_item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

let state_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let items : PacketWindowItemsContentArray = PacketContent :: read ( reader ) ?;;

let carried_item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, state_id, items, carried_item } ) } } pub type PacketWindowItemsContentArray = Vec <crate::protocol::types::slot::Slot >;
