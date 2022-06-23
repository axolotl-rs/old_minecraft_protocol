use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketPosition ; impl Packet for CbPacketPosition { type PacketIDType = i32 ; type PacketContent = PacketPositionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 56 } } pub struct PacketPositionContent { x: f64 ,

y: f64 ,

z: f64 ,

yaw: f32 ,

pitch: f32 ,

flags: i8 ,

teleport_id: minecraft_data::data::VarInt ,

dismount_vehicle: bool ,

 } impl PacketContent for PacketPositionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.yaw.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

total_bytes += self.flags.write(writer)?;;

total_bytes += self.teleport_id.write(writer)?;;

total_bytes += self.dismount_vehicle.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let yaw : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

let flags : i8 = PacketContent :: read ( reader ) ?;;

let teleport_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let dismount_vehicle : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, y, z, yaw, pitch, flags, teleport_id, dismount_vehicle } ) } }