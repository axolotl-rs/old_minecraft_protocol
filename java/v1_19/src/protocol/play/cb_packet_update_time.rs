mod cb_packet_update_time { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateTime ; impl Packet for CbPacketUpdateTime { type PacketIDType = i32 ; type PacketContent = PacketUpdateTimeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 89 } } pub struct PacketUpdateTimeContent { age: i64 ,

time: i64 ,

 } impl PacketContent for PacketUpdateTimeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.age.write(writer)?;;

total_bytes += self.time.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let age : i64 = PacketContent :: read ( reader ) ?;;

let time : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { age, time } ) } }

 }

 pub use cb_packet_update_time ::*;
