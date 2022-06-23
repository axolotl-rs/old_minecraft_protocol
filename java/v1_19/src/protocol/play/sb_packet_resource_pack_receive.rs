mod sb_packet_resource_pack_receive { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketResourcePackReceive ; impl Packet for SbPacketResourcePackReceive { type PacketIDType = i32 ; type PacketContent = PacketResourcePackReceiveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 33 } } pub struct PacketResourcePackReceiveContent { result: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketResourcePackReceiveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.result.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let result : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { result } ) } }

 }

 pub use sb_packet_resource_pack_receive ::*;
