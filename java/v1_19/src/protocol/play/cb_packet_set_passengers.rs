mod cb_packet_set_passengers { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetPassengers ; impl Packet for CbPacketSetPassengers { type PacketIDType = i32 ; type PacketContent = PacketSetPassengersContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 84 } } pub struct PacketSetPassengersContent { entity_id: minecraft_data::common::data::VarInt ,

passengers: PacketSetPassengersContentArray ,

 } impl PacketContent for PacketSetPassengersContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.passengers.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let entity_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let passengers : PacketSetPassengersContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, passengers } ) } } pub type PacketSetPassengersContentArray = Vec <minecraft_data::common::data::VarInt >;

 }

 pub use cb_packet_set_passengers ::*;
