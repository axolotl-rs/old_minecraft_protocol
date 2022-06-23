use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSteerVehicle ; impl Packet for SbPacketSteerVehicle { type PacketIDType = i32 ; type PacketContent = PacketSteerVehicleContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 28 } } pub struct PacketSteerVehicleContent { pub sideways: f32 ,

pub forward: f32 ,

pub jump: u8 ,

 } impl PacketContent for PacketSteerVehicleContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.sideways.write(writer)?;;

total_bytes += self.forward.write(writer)?;;

total_bytes += self.jump.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let sideways : f32 = PacketContent :: read ( reader ) ?;;

let forward : f32 = PacketContent :: read ( reader ) ?;;

let jump : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { sideways, forward, jump } ) } }
