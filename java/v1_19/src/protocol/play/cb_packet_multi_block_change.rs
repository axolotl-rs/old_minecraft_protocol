use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketMultiBlockChange ; impl Packet for CbPacketMultiBlockChange { type PacketIDType = i32 ; type PacketContent = PacketMultiBlockChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 63 } } pub struct PacketMultiBlockChangeContent { pub chunk_coordinates: minecraft_protocol::data::bitfield::BitField ,

pub not_trust_edges: bool ,

pub records: PacketMultiBlockChangeContentArray ,

 } impl PacketContent for PacketMultiBlockChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_coordinates.write(writer)?;;

total_bytes += self.not_trust_edges.write(writer)?;;

total_bytes += self.records.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let chunk_coordinates : minecraft_protocol::data::bitfield::BitField = PacketContent :: read ( reader ) ?;;

let not_trust_edges : bool = PacketContent :: read ( reader ) ?;;

let records : PacketMultiBlockChangeContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_coordinates, not_trust_edges, records } ) } } pub type PacketMultiBlockChangeContentArray = Vec <minecraft_protocol::data::VarInt >;
