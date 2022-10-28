use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderCenter ; impl Packet for CbPacketWorldBorderCenter { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderCenterContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 66 } } pub struct PacketWorldBorderCenterContent { pub x: f64 ,

pub z: f64 ,

 } impl PacketContent for PacketWorldBorderCenterContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, z } ) } }
