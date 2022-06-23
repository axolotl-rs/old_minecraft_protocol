use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketAbilities ; impl Packet for SbPacketAbilities { type PacketIDType = i32 ; type PacketContent = PacketAbilitiesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 25 } } pub struct PacketAbilitiesContent { pub flags: i8 ,

 } impl PacketContent for PacketAbilitiesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let flags : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { flags } ) } }
