mod cb_packet_map { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketMap ; impl Packet for CbPacketMap { type PacketIDType = i32 ; type PacketContent = PacketMapContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 39 } } pub struct PacketMapContent { item_damage: minecraft_data::common::data::VarInt ,

scale: i8 ,

locked: bool ,

icons: void ,

columns: u8 ,

rows: PacketMapContentContent ,

x: PacketMapContentContent ,

y: PacketMapContentContent ,

data: PacketMapContentContent ,

 } impl PacketContent for PacketMapContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.item_damage.write(writer)?;;

total_bytes += self.scale.write(writer)?;;

total_bytes += self.locked.write(writer)?;;

total_bytes += self.icons.write(writer)?;;

total_bytes += self.columns.write(writer)?;;

total_bytes += self.rows.switch_write(false,writer)?;;

total_bytes += self.x.switch_write(false,writer)?;;

total_bytes += self.y.switch_write(false,writer)?;;

total_bytes += self.data.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let item_damage : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let scale : i8 = PacketContent :: read ( reader ) ?;;

let locked : bool = PacketContent :: read ( reader ) ?;;

let icons : void = PacketContent :: read ( reader ) ?;;

let columns : u8 = PacketContent :: read ( reader ) ?;;

let rows : PacketMapContentContent = PacketSwitch::switch_read(&columns,reader)?;;

let x : PacketMapContentContent = PacketSwitch::switch_read(&columns,reader)?;;

let y : PacketMapContentContent = PacketSwitch::switch_read(&columns,reader)?;;

let data : PacketMapContentContent = PacketSwitch::switch_read(&columns,reader)?;;

 Ok ( Self { item_damage, scale, locked, icons, columns, rows, x, y, data } ) } } pub enum PacketMapContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketMapContentContent { type CompareType = u8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketMapContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketMapContentContent { type CompareType = u8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketMapContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketMapContentContent { type CompareType = u8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketMapContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketMapContentContent { type CompareType = u8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

 }

 pub use cb_packet_map ::*;
