mod sb_packet_update_sign { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateSign ; impl Packet for SbPacketUpdateSign { type PacketIDType = i32 ; type PacketContent = PacketUpdateSignContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 43 } } pub struct PacketUpdateSignContent { location: minecraft_data::common::data::position::Position ,

text_1: String ,

text_2: String ,

text_3: String ,

text_4: String ,

 } impl PacketContent for PacketUpdateSignContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.text_1.write(writer)?;;

total_bytes += self.text_2.write(writer)?;;

total_bytes += self.text_3.write(writer)?;;

total_bytes += self.text_4.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let text_1 : String = PacketContent :: read ( reader ) ?;;

let text_2 : String = PacketContent :: read ( reader ) ?;;

let text_3 : String = PacketContent :: read ( reader ) ?;;

let text_4 : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, text_1, text_2, text_3, text_4 } ) } }

 }

 pub use sb_packet_update_sign ::*;
