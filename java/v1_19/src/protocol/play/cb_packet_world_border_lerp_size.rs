use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderLerpSize ; impl Packet for CbPacketWorldBorderLerpSize { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderLerpSizeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 67 } } pub struct PacketWorldBorderLerpSizeContent { pub old_diameter: f64 ,

pub new_diameter: f64 ,

pub speed: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketWorldBorderLerpSizeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.old_diameter.write(writer)?;;

total_bytes += self.new_diameter.write(writer)?;;

total_bytes += self.speed.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let old_diameter : f64 = PacketContent :: read ( reader ) ?;;

let new_diameter : f64 = PacketContent :: read ( reader ) ?;;

let speed : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { old_diameter, new_diameter, speed } ) } }
