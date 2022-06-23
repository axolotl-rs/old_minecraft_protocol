mod cb_packet_vehicle_move { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketVehicleMove ; impl Packet for CbPacketVehicleMove { type PacketIDType = i32 ; type PacketContent = PacketVehicleMoveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 44 } } pub struct PacketVehicleMoveContent { x: f64 ,

y: f64 ,

z: f64 ,

yaw: f32 ,

pitch: f32 ,

 } impl PacketContent for PacketVehicleMoveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.yaw.write(writer)?;;

total_bytes += self.pitch.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let yaw : f32 = PacketContent :: read ( reader ) ?;;

let pitch : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, y, z, yaw, pitch } ) } }

 }

 pub use cb_packet_vehicle_move ::*;
