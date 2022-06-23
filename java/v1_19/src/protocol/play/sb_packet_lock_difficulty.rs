use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketLockDifficulty ; impl Packet for SbPacketLockDifficulty { type PacketIDType = i32 ; type PacketContent = PacketLockDifficultyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 16 } } pub struct PacketLockDifficultyContent { pub locked: bool ,

 } impl PacketContent for PacketLockDifficultyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.locked.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let locked : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { locked } ) } }
