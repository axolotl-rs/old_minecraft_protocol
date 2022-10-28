use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEndCombatEvent ; impl Packet for CbPacketEndCombatEvent { type PacketIDType = i32 ; type PacketContent = PacketEndCombatEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 51 } } pub struct PacketEndCombatEventContent { pub duration: minecraft_protocol::data::VarInt ,

pub entity_id: i32 ,

 } impl PacketContent for PacketEndCombatEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.duration.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let duration : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { duration, entity_id } ) } }
