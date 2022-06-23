use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketBlockDig ; impl Packet for SbPacketBlockDig { type PacketIDType = i32 ; type PacketContent = PacketBlockDigContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 26 } } pub struct PacketBlockDigContent { pub status: i8 ,

pub location: minecraft_data::data::position::Position ,

pub face: i8 ,

 } impl PacketContent for PacketBlockDigContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.status.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.face.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let status : i8 = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let face : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { status, location, face } ) } }
