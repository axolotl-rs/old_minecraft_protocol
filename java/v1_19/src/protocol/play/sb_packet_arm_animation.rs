use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketArmAnimation ; impl Packet for SbPacketArmAnimation { type PacketIDType = i32 ; type PacketContent = PacketArmAnimationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 44 } } pub struct PacketArmAnimationContent { pub hand: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketArmAnimationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.hand.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let hand : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { hand } ) } }
