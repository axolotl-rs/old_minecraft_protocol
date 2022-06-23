mod cb_packet_scoreboard_display_objective { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketScoreboardDisplayObjective ; impl Packet for CbPacketScoreboardDisplayObjective { type PacketIDType = i32 ; type PacketContent = PacketScoreboardDisplayObjectiveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 76 } } pub struct PacketScoreboardDisplayObjectiveContent { position: i8 ,

name: String ,

 } impl PacketContent for PacketScoreboardDisplayObjectiveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.position.write(writer)?;;

total_bytes += self.name.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let position : i8 = PacketContent :: read ( reader ) ?;;

let name : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { position, name } ) } }

 }

 pub use cb_packet_scoreboard_display_objective ::*;
