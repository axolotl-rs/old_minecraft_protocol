use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateJigsawBlock ; impl Packet for SbPacketUpdateJigsawBlock { type PacketIDType = i32 ; type PacketContent = PacketUpdateJigsawBlockContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 41 } } pub struct PacketUpdateJigsawBlockContent { pub location: minecraft_data::data::position::Position ,

pub name: String ,

pub target: String ,

pub pool: String ,

pub final_state: String ,

pub joint_type: String ,

 } impl PacketContent for PacketUpdateJigsawBlockContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.name.write(writer)?;;

total_bytes += self.target.write(writer)?;;

total_bytes += self.pool.write(writer)?;;

total_bytes += self.final_state.write(writer)?;;

total_bytes += self.joint_type.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let name : String = PacketContent :: read ( reader ) ?;;

let target : String = PacketContent :: read ( reader ) ?;;

let pool : String = PacketContent :: read ( reader ) ?;;

let final_state : String = PacketContent :: read ( reader ) ?;;

let joint_type : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, name, target, pool, final_state, joint_type } ) } }
