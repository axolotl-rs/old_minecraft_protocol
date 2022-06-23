use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAbilities ; impl Packet for CbPacketAbilities { type PacketIDType = i32 ; type PacketContent = PacketAbilitiesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 50 } } pub struct PacketAbilitiesContent { flags: i8 ,

flying_speed: f32 ,

walking_speed: f32 ,

 } impl PacketContent for PacketAbilitiesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

total_bytes += self.flying_speed.write(writer)?;;

total_bytes += self.walking_speed.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let flags : i8 = PacketContent :: read ( reader ) ?;;

let flying_speed : f32 = PacketContent :: read ( reader ) ?;;

let walking_speed : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { flags, flying_speed, walking_speed } ) } }
