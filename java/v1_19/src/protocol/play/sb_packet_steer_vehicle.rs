mod sb_packet_steer_vehicle { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSteerVehicle ; impl Packet for SbPacketSteerVehicle { type PacketIDType = i32 ; type PacketContent = PacketSteerVehicleContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 28 } } pub struct PacketSteerVehicleContent { sideways: f32 ,

forward: f32 ,

jump: u8 ,

 } impl PacketContent for PacketSteerVehicleContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sideways.write(writer)?;;

total_bytes += self.forward.write(writer)?;;

total_bytes += self.jump.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let sideways : f32 = PacketContent :: read ( reader ) ?;;

let forward : f32 = PacketContent :: read ( reader ) ?;;

let jump : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sideways, forward, jump } ) } }

 }

 pub use sb_packet_steer_vehicle ::*;
