mod cb_packet_scoreboard_objective { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketScoreboardObjective ; impl Packet for CbPacketScoreboardObjective { type PacketIDType = i32 ; type PacketContent = PacketScoreboardObjectiveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 83 } } pub struct PacketScoreboardObjectiveContent { name: String ,

action: i8 ,

display_text: PacketScoreboardObjectiveContentContent ,

data_type: PacketScoreboardObjectiveContentContent ,

 } impl PacketContent for PacketScoreboardObjectiveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.name.write(writer)?;;

total_bytes += self.action.write(writer)?;;

total_bytes += self.display_text.switch_write(false,writer)?;;

total_bytes += self.data_type.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let name : String = PacketContent :: read ( reader ) ?;;

let action : i8 = PacketContent :: read ( reader ) ?;;

let display_text : PacketScoreboardObjectiveContentContent = PacketSwitch::switch_read(&action,reader)?;;

let data_type : PacketScoreboardObjectiveContentContent = PacketSwitch::switch_read(&action,reader)?;;

 Ok ( Self { name, action, display_text, data_type } ) } } pub enum PacketScoreboardObjectiveContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

/// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

 } impl PacketSwitch for PacketScoreboardObjectiveContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketScoreboardObjectiveContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::common::data::VarInt ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (minecraft_data::common::data::VarInt ) ,

 } impl PacketSwitch for PacketScoreboardObjectiveContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

 }

 pub use cb_packet_scoreboard_objective ::*;
