use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTileEntityData ; impl Packet for CbPacketTileEntityData { type PacketIDType = i32 ; type PacketContent = PacketTileEntityDataContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 10 } } pub struct PacketTileEntityDataContent { pub location: minecraft_protocol::data::position::Position ,

pub action: minecraft_protocol::data::VarInt ,

pub nbt_data: minecraft_protocol::data::nbt::OptionalNbt ,

 } impl PacketContent for PacketTileEntityDataContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.action.write(writer)?;;

total_bytes += self.nbt_data.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

let action : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let nbt_data : minecraft_protocol::data::nbt::OptionalNbt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, action, nbt_data } ) } }
