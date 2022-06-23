mod sb_packet_set_protocol { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetProtocol ; impl Packet for SbPacketSetProtocol { type PacketIDType = i32 ; type PacketContent = PacketSetProtocolContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketSetProtocolContent { protocol_version: minecraft_data::common::data::VarInt ,

server_host: String ,

server_port: u16 ,

next_state: minecraft_data::common::data::VarInt ,

 } impl PacketContent for PacketSetProtocolContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.protocol_version.write(writer)?;;

total_bytes += self.server_host.write(writer)?;;

total_bytes += self.server_port.write(writer)?;;

total_bytes += self.next_state.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let protocol_version : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let server_host : String = PacketContent :: read ( reader ) ?;;

let server_port : u16 = PacketContent :: read ( reader ) ?;;

let next_state : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { protocol_version, server_host, server_port, next_state } ) } }

 }

 pub use sb_packet_set_protocol ::*;
