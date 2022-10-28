use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateSign ; impl Packet for SbPacketUpdateSign { type PacketIDType = i32 ; type PacketContent = PacketUpdateSignContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 43 } } pub struct PacketUpdateSignContent { pub location: minecraft_protocol::data::position::Position ,

pub text_1: String ,

pub text_2: String ,

pub text_3: String ,

pub text_4: String ,

 } impl PacketContent for PacketUpdateSignContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.text_1.write(writer)?;;

total_bytes += self.text_2.write(writer)?;;

total_bytes += self.text_3.write(writer)?;;

total_bytes += self.text_4.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let text_1 : String = PacketContent :: read ( reader ) ?;;

let text_2 : String = PacketContent :: read ( reader ) ?;;

let text_3 : String = PacketContent :: read ( reader ) ?;;

let text_4 : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, text_1, text_2, text_3, text_4 } ) } }
