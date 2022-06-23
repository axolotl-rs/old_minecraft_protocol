mod cb_packet_world_particles { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldParticles ; impl Packet for CbPacketWorldParticles { type PacketIDType = i32 ; type PacketContent = PacketWorldParticlesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 36 } } pub struct PacketWorldParticlesContent { particle_id: i32 ,

long_distance: bool ,

x: f64 ,

y: f64 ,

z: f64 ,

offset_x: f32 ,

offset_y: f32 ,

offset_z: f32 ,

particle_data: f32 ,

particles: i32 ,

data: crate::protocol::types::particle_data::ParticleData ,

 } impl PacketContent for PacketWorldParticlesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.particle_id.write(writer)?;;

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

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let particle_id : i32 = PacketContent :: read ( reader ) ?;;

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

 }

 pub use cb_packet_world_particles ::*;
