mod sb_packet_update_jigsaw_block { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateJigsawBlock ; impl Packet for SbPacketUpdateJigsawBlock { type PacketIDType = i32 ; type PacketContent = PacketUpdateJigsawBlockContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 41 } } pub struct PacketUpdateJigsawBlockContent { location: minecraft_data::common::data::position::Position ,

name: String ,

target: String ,

pool: String ,

final_state: String ,

joint_type: String ,

 } impl PacketContent for PacketUpdateJigsawBlockContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.name.write(writer)?;;

total_bytes += self.target.write(writer)?;;

total_bytes += self.pool.write(writer)?;;

total_bytes += self.final_state.write(writer)?;;

total_bytes += self.joint_type.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let name : String = PacketContent :: read ( reader ) ?;;

let target : String = PacketContent :: read ( reader ) ?;;

let pool : String = PacketContent :: read ( reader ) ?;;

let final_state : String = PacketContent :: read ( reader ) ?;;

let joint_type : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, name, target, pool, final_state, joint_type } ) } }

 }

 pub use sb_packet_update_jigsaw_block ::*;
