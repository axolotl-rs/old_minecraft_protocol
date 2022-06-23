mod cb_packet_scoreboard_score { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketScoreboardScore ; impl Packet for CbPacketScoreboardScore { type PacketIDType = i32 ; type PacketContent = PacketScoreboardScoreContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 86 } } pub struct PacketScoreboardScoreContent { item_name: String ,

action: i8 ,

score_name: String ,

value: PacketScoreboardScoreContentContent ,

 } impl PacketContent for PacketScoreboardScoreContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.item_name.write(writer)?;;

total_bytes += self.action.write(writer)?;;

total_bytes += self.score_name.write(writer)?;;

total_bytes += self.value.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let item_name : String = PacketContent :: read ( reader ) ?;;

let action : i8 = PacketContent :: read ( reader ) ?;;

let score_name : String = PacketContent :: read ( reader ) ?;;

let value : PacketScoreboardScoreContentContent = PacketSwitch::switch_read(&action,reader)?;;

 Ok ( Self { item_name, action, score_name, value } ) } } pub enum PacketScoreboardScoreContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (minecraft_data::common::data::Void ) ,

 } impl PacketSwitch for PacketScoreboardScoreContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

 }

 pub use cb_packet_scoreboard_score ::*;
