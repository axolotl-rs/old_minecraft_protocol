use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketGameStateChange ; impl Packet for CbPacketGameStateChange { type PacketIDType = i32 ; type PacketContent = PacketGameStateChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 30 } } pub struct PacketGameStateChangeContent { pub reason: u8 ,

pub game_mode: f32 ,

 } impl PacketContent for PacketGameStateChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reason.write(writer)?;;

total_bytes += self.game_mode.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let reason : u8 = PacketContent :: read ( reader ) ?;;

let game_mode : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reason, game_mode } ) } }
