use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketOpenHorseWindow ; impl Packet for CbPacketOpenHorseWindow { type PacketIDType = i32 ; type PacketContent = PacketOpenHorseWindowContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 31 } } pub struct PacketOpenHorseWindowContent { pub window_id: u8 ,

pub nb_slots: minecraft_protocol::data::VarInt ,

pub entity_id: i32 ,

 } impl PacketContent for PacketOpenHorseWindowContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.nb_slots.write(writer)?;;

total_bytes += self.entity_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

let nb_slots : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let entity_id : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, nb_slots, entity_id } ) } }
