use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketExplosion ; impl Packet for CbPacketExplosion { type PacketIDType = i32 ; type PacketContent = PacketExplosionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 28 } } pub struct PacketExplosionContent { x: f32 ,

y: f32 ,

z: f32 ,

radius: f32 ,

affected_block_offsets: PacketExplosionContentArray ,

player_motion_x: f32 ,

player_motion_y: f32 ,

player_motion_z: f32 ,

 } impl PacketContent for PacketExplosionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.radius.write(writer)?;;

total_bytes += self.affected_block_offsets.write(writer)?;;

total_bytes += self.player_motion_x.write(writer)?;;

total_bytes += self.player_motion_y.write(writer)?;;

total_bytes += self.player_motion_z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : f32 = PacketContent :: read ( reader ) ?;;

let y : f32 = PacketContent :: read ( reader ) ?;;

let z : f32 = PacketContent :: read ( reader ) ?;;

let radius : f32 = PacketContent :: read ( reader ) ?;;

let affected_block_offsets : PacketExplosionContentArray = PacketContent :: read ( reader ) ?;;

let player_motion_x : f32 = PacketContent :: read ( reader ) ?;;

let player_motion_y : f32 = PacketContent :: read ( reader ) ?;;

let player_motion_z : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, y, z, radius, affected_block_offsets, player_motion_x, player_motion_y, player_motion_z } ) } } pub type PacketExplosionContentArray = Vec <PacketExplosionContentArrayContent >; pub struct PacketExplosionContentArrayContent { x: i8 ,

y: i8 ,

z: i8 ,

 } impl PacketContent for PacketExplosionContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : i8 = PacketContent :: read ( reader ) ?;;

let y : i8 = PacketContent :: read ( reader ) ?;;

let z : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, y, z } ) } }
