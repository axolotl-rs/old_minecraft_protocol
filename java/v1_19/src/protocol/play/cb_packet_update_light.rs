use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateLight ; impl Packet for CbPacketUpdateLight { type PacketIDType = i32 ; type PacketContent = PacketUpdateLightContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 37 } } pub struct PacketUpdateLightContent { pub chunk_x: minecraft_data::data::VarInt ,

pub chunk_z: minecraft_data::data::VarInt ,

pub trust_edges: bool ,

pub sky_light_mask: PacketUpdateLightContentArray ,

pub block_light_mask: PacketUpdateLightContentArray ,

pub empty_sky_light_mask: PacketUpdateLightContentArray ,

pub empty_block_light_mask: PacketUpdateLightContentArray ,

pub sky_light: PacketUpdateLightContentArray ,

pub block_light: PacketUpdateLightContentArray ,

 } impl PacketContent for PacketUpdateLightContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.chunk_x.write(writer)?;;

total_bytes += self.chunk_z.write(writer)?;;

total_bytes += self.trust_edges.write(writer)?;;

total_bytes += self.sky_light_mask.write(writer)?;;

total_bytes += self.block_light_mask.write(writer)?;;

total_bytes += self.empty_sky_light_mask.write(writer)?;;

total_bytes += self.empty_block_light_mask.write(writer)?;;

total_bytes += self.sky_light.write(writer)?;;

total_bytes += self.block_light.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let chunk_x : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let chunk_z : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let trust_edges : bool = PacketContent :: read ( reader ) ?;;

let sky_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let block_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let empty_sky_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let empty_block_light_mask : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let sky_light : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

let block_light : PacketUpdateLightContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { chunk_x, chunk_z, trust_edges, sky_light_mask, block_light_mask, empty_sky_light_mask, empty_block_light_mask, sky_light, block_light } ) } } pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <i64 >;

pub type PacketUpdateLightContentArray = Vec <PacketUpdateLightContentArrayArray >; pub type PacketUpdateLightContentArrayArray = Vec <u8 >;

pub type PacketUpdateLightContentArray = Vec <PacketUpdateLightContentArrayArray >; pub type PacketUpdateLightContentArrayArray = Vec <u8 >;
