use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityStatus ; impl Packet for CbPacketEntityStatus { type PacketIDType = i32 ; type PacketContent = PacketEntityStatusContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 27 } } pub struct PacketEntityStatusContent { pub entity_id: i32 ,

pub entity_status: i8 ,

 } impl PacketContent for PacketEntityStatusContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.entity_status.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : i32 = PacketContent :: read ( reader ) ?;;

let entity_status : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, entity_status } ) } }
