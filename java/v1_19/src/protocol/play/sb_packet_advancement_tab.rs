use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketAdvancementTab ; impl Packet for SbPacketAdvancementTab { type PacketIDType = i32 ; type PacketContent = PacketAdvancementTabContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 34 } } pub struct PacketAdvancementTabContent { pub action: minecraft_data::data::VarInt ,

pub tab_id: PacketAdvancementTabContentContent ,

 } impl PacketContent for PacketAdvancementTabContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.action.write(writer)?;;

total_bytes += self.tab_id.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let action : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let tab_id : PacketAdvancementTabContentContent = PacketSwitch::switch_read(&action,reader)?;;

 Ok ( Self { action, tab_id } ) } } pub enum PacketAdvancementTabContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

/// This switch variant requires a value 1 in the compare_to field

 Switch1 (minecraft_data::data::Void ) ,

 } impl PacketSwitch for PacketAdvancementTabContentContent { type CompareType = minecraft_data::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
