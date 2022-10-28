use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateStructureBlock ; impl Packet for SbPacketUpdateStructureBlock { type PacketIDType = i32 ; type PacketContent = PacketUpdateStructureBlockContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 42 } } pub struct PacketUpdateStructureBlockContent { pub location: minecraft_protocol::data::position::Position ,

pub action: minecraft_protocol::data::VarInt ,

pub mode: minecraft_protocol::data::VarInt ,

pub name: String ,

pub offset_x: u8 ,

pub offset_y: u8 ,

pub offset_z: u8 ,

pub size_x: u8 ,

pub size_y: u8 ,

pub size_z: u8 ,

pub mirror: minecraft_protocol::data::VarInt ,

pub rotation: minecraft_protocol::data::VarInt ,

pub metadata: String ,

pub integrity: f32 ,

pub seed: minecraft_protocol::data::VarInt ,

pub flags: u8 ,

 } impl PacketContent for PacketUpdateStructureBlockContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.action.write(writer)?;;

total_bytes += self.mode.write(writer)?;;

total_bytes += self.name.write(writer)?;;

total_bytes += self.offset_x.write(writer)?;;

total_bytes += self.offset_y.write(writer)?;;

total_bytes += self.offset_z.write(writer)?;;

total_bytes += self.size_x.write(writer)?;;

total_bytes += self.size_y.write(writer)?;;

total_bytes += self.size_z.write(writer)?;;

total_bytes += self.mirror.write(writer)?;;

total_bytes += self.rotation.write(writer)?;;

total_bytes += self.metadata.write(writer)?;;

total_bytes += self.integrity.write(writer)?;;

total_bytes += self.seed.write(writer)?;;

total_bytes += self.flags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let action : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let mode : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let name : String = PacketContent :: read ( reader ) ?;;

let offset_x : u8 = PacketContent :: read ( reader ) ?;;

let offset_y : u8 = PacketContent :: read ( reader ) ?;;

let offset_z : u8 = PacketContent :: read ( reader ) ?;;

let size_x : u8 = PacketContent :: read ( reader ) ?;;

let size_y : u8 = PacketContent :: read ( reader ) ?;;

let size_z : u8 = PacketContent :: read ( reader ) ?;;

let mirror : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let rotation : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let metadata : String = PacketContent :: read ( reader ) ?;;

let integrity : f32 = PacketContent :: read ( reader ) ?;;

let seed : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let flags : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, action, mode, name, offset_x, offset_y, offset_z, size_x, size_y, size_z, mirror, rotation, metadata, integrity, seed, flags } ) } }
