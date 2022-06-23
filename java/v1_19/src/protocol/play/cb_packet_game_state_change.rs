mod cb_packet_game_state_change { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketGameStateChange ; impl Packet for CbPacketGameStateChange { type PacketIDType = i32 ; type PacketContent = PacketGameStateChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 30 } } pub struct PacketGameStateChangeContent { reason: u8 ,

game_mode: f32 ,

 } impl PacketContent for PacketGameStateChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reason.write(writer)?;;

total_bytes += self.game_mode.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let reason : u8 = PacketContent :: read ( reader ) ?;;

let game_mode : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reason, game_mode } ) } }

 }

 pub use cb_packet_game_state_change ::*;
