use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAnimation ; impl Packet for CbPacketAnimation { type PacketIDType = i32 ; type PacketContent = PacketAnimationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 6 } } pub struct PacketAnimationContent { entity_id: minecraft_data::data::VarInt ,

animation: u8 ,

 } impl PacketContent for PacketAnimationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.animation.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let animation : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, animation } ) } }
