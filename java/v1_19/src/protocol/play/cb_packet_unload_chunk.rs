mod cb_packet_unload_chunk { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUnloadChunk ; impl Packet for CbPacketUnloadChunk { type PacketIDType = i32 ; type PacketContent = PacketUnloadChunkContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 29 } } pub struct PacketUnloadChunkContent { chunk_x: i32 ,

chunk_z: i32 ,

 } impl PacketContent for PacketUnloadChunkContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_x.write(writer)?;;

total_bytes += self.chunk_z.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let chunk_x : i32 = PacketContent :: read ( reader ) ?;;

let chunk_z : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_x, chunk_z } ) } }

 }

 pub use cb_packet_unload_chunk ::*;
