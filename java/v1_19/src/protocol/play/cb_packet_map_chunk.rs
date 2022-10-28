use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketMapChunk ; impl Packet for CbPacketMapChunk { type PacketIDType = i32 ; type PacketContent = PacketMapChunkContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 34 } } pub struct PacketMapChunkContent { pub x: i32 ,

pub z: i32 ,

pub heightmaps: minecraft_protocol::data::nbt::Nbt ,

pub chunk_data: void ,

pub block_entities: PacketMapChunkContentArray ,

pub trust_edges: bool ,

pub sky_light_mask: PacketMapChunkContentArray ,

pub block_light_mask: PacketMapChunkContentArray ,

pub empty_sky_light_mask: PacketMapChunkContentArray ,

pub empty_block_light_mask: PacketMapChunkContentArray ,

pub sky_light: PacketMapChunkContentArray ,

pub block_light: PacketMapChunkContentArray ,

 } impl PacketContent for PacketMapChunkContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.x.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.heightmaps.write(writer)?;;

total_bytes += self.chunk_data.write(writer)?;;

total_bytes += self.block_entities.write(writer)?;;

total_bytes += self.trust_edges.write(writer)?;;

total_bytes += self.sky_light_mask.write(writer)?;;

total_bytes += self.block_light_mask.write(writer)?;;

total_bytes += self.empty_sky_light_mask.write(writer)?;;

total_bytes += self.empty_block_light_mask.write(writer)?;;

total_bytes += self.sky_light.write(writer)?;;

total_bytes += self.block_light.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let x : i32 = PacketContent :: read ( reader ) ?;;

let z : i32 = PacketContent :: read ( reader ) ?;;

let heightmaps : minecraft_protocol::data::nbt::Nbt = PacketContent :: read ( reader ) ?;;

let chunk_data : void = PacketContent :: read ( reader ) ?;;

let block_entities : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let trust_edges : bool = PacketContent :: read ( reader ) ?;;

let sky_light_mask : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let block_light_mask : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let empty_sky_light_mask : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let empty_block_light_mask : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let sky_light : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

let block_light : PacketMapChunkContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { x, z, heightmaps, chunk_data, block_entities, trust_edges, sky_light_mask, block_light_mask, empty_sky_light_mask, empty_block_light_mask, sky_light, block_light } ) } } pub type PacketMapChunkContentArray = Vec <crate::protocol::types::chunk_block_entity::ChunkBlockEntity >;

pub type PacketMapChunkContentArray = Vec <i64 >;

pub type PacketMapChunkContentArray = Vec <i64 >;

pub type PacketMapChunkContentArray = Vec <i64 >;

pub type PacketMapChunkContentArray = Vec <i64 >;

pub type PacketMapChunkContentArray = Vec <PacketMapChunkContentArrayArray >; pub type PacketMapChunkContentArrayArray = Vec <u8 >;

pub type PacketMapChunkContentArray = Vec <PacketMapChunkContentArrayArray >; pub type PacketMapChunkContentArrayArray = Vec <u8 >;
