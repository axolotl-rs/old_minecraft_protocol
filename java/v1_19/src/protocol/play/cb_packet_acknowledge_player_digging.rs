use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketAcknowledgePlayerDigging ; impl Packet for CbPacketAcknowledgePlayerDigging { type PacketIDType = i32 ; type PacketContent = PacketAcknowledgePlayerDiggingContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 8 } } pub struct PacketAcknowledgePlayerDiggingContent { pub location: minecraft_data::data::position::Position ,

pub block: minecraft_data::data::VarInt ,

pub status: minecraft_data::data::VarInt ,

pub successful: bool ,

 } impl PacketContent for PacketAcknowledgePlayerDiggingContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.block.write(writer)?;;

total_bytes += self.status.write(writer)?;;

total_bytes += self.successful.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let block : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let status : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let successful : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, block, status, successful } ) } }
