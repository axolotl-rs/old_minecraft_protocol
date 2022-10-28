use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockBreakAnimation ; impl Packet for CbPacketBlockBreakAnimation { type PacketIDType = i32 ; type PacketContent = PacketBlockBreakAnimationContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 9 } } pub struct PacketBlockBreakAnimationContent { pub entity_id: minecraft_protocol::data::VarInt ,

pub location: minecraft_protocol::data::position::Position ,

pub destroy_stage: i8 ,

 } impl PacketContent for PacketBlockBreakAnimationContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.destroy_stage.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let destroy_stage : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, location, destroy_stage } ) } }
