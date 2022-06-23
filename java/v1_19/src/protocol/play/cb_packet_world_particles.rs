use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldParticles ; impl Packet for CbPacketWorldParticles { type PacketIDType = i32 ; type PacketContent = PacketWorldParticlesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 36 } } pub struct PacketWorldParticlesContent { pub particle_id: i32 ,

pub long_distance: bool ,

pub x: f64 ,

pub y: f64 ,

pub z: f64 ,

pub offset_x: f32 ,

pub offset_y: f32 ,

pub offset_z: f32 ,

pub particle_data: f32 ,

pub particles: i32 ,

pub data: crate::protocol::types::particle_data::ParticleData ,

 } impl PacketContent for PacketWorldParticlesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.particle_id.write(writer)?;;

total_bytes += self.long_distance.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.offset_x.write(writer)?;;

total_bytes += self.offset_y.write(writer)?;;

total_bytes += self.offset_z.write(writer)?;;

total_bytes += self.particle_data.write(writer)?;;

total_bytes += self.particles.write(writer)?;;

total_bytes += self.data.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let particle_id : i32 = PacketContent :: read ( reader ) ?;;

let long_distance : bool = PacketContent :: read ( reader ) ?;;

let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let offset_x : f32 = PacketContent :: read ( reader ) ?;;

let offset_y : f32 = PacketContent :: read ( reader ) ?;;

let offset_z : f32 = PacketContent :: read ( reader ) ?;;

let particle_data : f32 = PacketContent :: read ( reader ) ?;;

let particles : i32 = PacketContent :: read ( reader ) ?;;

let data : crate::protocol::types::particle_data::ParticleData = PacketSwitch::switch_read(&particleId,reader)?;;

 Ok ( Self { particle_id, long_distance, x, y, z, offset_x, offset_y, offset_z, particle_data, particles, data } ) } }
