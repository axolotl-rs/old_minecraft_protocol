use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityTeleport ; impl Packet for CbPacketEntityTeleport { type PacketIDType = i32 ; type PacketContent = PacketEntityTeleportContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 98 } } pub struct PacketEntityTeleportContent { entity_id: minecraft_data::data::VarInt ,

x: f64 ,

y: f64 ,

z: f64 ,

yaw: i8 ,

pitch: i8 ,

on_ground: bool ,

 } impl PacketContent for PacketEntityTeleportContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.yaw.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let yaw : i8 = PacketContent :: read ( reader ) ?;;

let pitch : i8 = PacketContent :: read ( reader ) ?;;

let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, x, y, z, yaw, pitch, on_ground } ) } }
