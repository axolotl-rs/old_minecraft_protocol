use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetDifficulty ; impl Packet for SbPacketSetDifficulty { type PacketIDType = i32 ; type PacketContent = PacketSetDifficultyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 2 } } pub struct PacketSetDifficultyContent { pub new_difficulty: u8 ,

 } impl PacketContent for PacketSetDifficultyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.new_difficulty.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let new_difficulty : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { new_difficulty } ) } }
