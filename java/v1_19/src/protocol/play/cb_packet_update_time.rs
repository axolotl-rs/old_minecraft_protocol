use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateTime ; impl Packet for CbPacketUpdateTime { type PacketIDType = i32 ; type PacketContent = PacketUpdateTimeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 89 } } pub struct PacketUpdateTimeContent { pub age: i64 ,

pub time: i64 ,

 } impl PacketContent for PacketUpdateTimeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.age.write(writer)?;;

total_bytes += self.time.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let age : i64 = PacketContent :: read ( reader ) ?;;

let time : i64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { age, time } ) } }
