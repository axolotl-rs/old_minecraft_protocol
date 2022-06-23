mod cb_packet_world_event { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldEvent ; impl Packet for CbPacketWorldEvent { type PacketIDType = i32 ; type PacketContent = PacketWorldEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 35 } } pub struct PacketWorldEventContent { effect_id: i32 ,

location: minecraft_data::common::data::position::Position ,

data: i32 ,

global: bool ,

 } impl PacketContent for PacketWorldEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.effect_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.data.write(writer)?;;

total_bytes += self.global.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let effect_id : i32 = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::common::data::position::Position = PacketContent :: read ( reader ) ?;;

let data : i32 = PacketContent :: read ( reader ) ?;;

let global : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { effect_id, location, data, global } ) } }

 }

 pub use cb_packet_world_event ::*;
