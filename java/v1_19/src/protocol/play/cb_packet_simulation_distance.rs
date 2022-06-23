mod cb_packet_simulation_distance { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSimulationDistance ; impl Packet for CbPacketSimulationDistance { type PacketIDType = i32 ; type PacketContent = PacketSimulationDistanceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 87 } } pub struct PacketSimulationDistanceContent { distance: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketSimulationDistanceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.distance.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let distance : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { distance } ) } }

 }

 pub use cb_packet_simulation_distance ::*;
