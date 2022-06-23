use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBlockAction ; impl Packet for CbPacketBlockAction { type PacketIDType = i32 ; type PacketContent = PacketBlockActionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 11 } } pub struct PacketBlockActionContent { pub location: minecraft_data::data::position::Position ,

pub byte_1: u8 ,

pub byte_2: u8 ,

pub block_id: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketBlockActionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.byte_1.write(writer)?;;

total_bytes += self.byte_2.write(writer)?;;

total_bytes += self.block_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let byte_1 : u8 = PacketContent :: read ( reader ) ?;;

let byte_2 : u8 = PacketContent :: read ( reader ) ?;;

let block_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, byte_1, byte_2, block_id } ) } }
