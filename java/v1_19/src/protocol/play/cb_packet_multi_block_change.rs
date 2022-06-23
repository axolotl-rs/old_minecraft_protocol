use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketMultiBlockChange ; impl Packet for CbPacketMultiBlockChange { type PacketIDType = i32 ; type PacketContent = PacketMultiBlockChangeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 63 } } pub struct PacketMultiBlockChangeContent { chunk_coordinates: minecraft_data::data::bitfield::BitField ,

not_trust_edges: bool ,

records: PacketMultiBlockChangeContentArray ,

 } impl PacketContent for PacketMultiBlockChangeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_coordinates.write(writer)?;;

total_bytes += self.not_trust_edges.write(writer)?;;

total_bytes += self.records.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let chunk_coordinates : minecraft_data::data::bitfield::BitField = PacketContent :: read ( reader ) ?;;

let not_trust_edges : bool = PacketContent :: read ( reader ) ?;;

let records : PacketMultiBlockChangeContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_coordinates, not_trust_edges, records } ) } } pub type PacketMultiBlockChangeContentArray = Vec <minecraft_data::data::VarInt >;
