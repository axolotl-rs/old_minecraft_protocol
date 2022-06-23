use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketRelEntityMove ; impl Packet for CbPacketRelEntityMove { type PacketIDType = i32 ; type PacketContent = PacketRelEntityMoveContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 41 } } pub struct PacketRelEntityMoveContent { entity_id: minecraft_data::data::VarInt ,

d_x: i16 ,

d_y: i16 ,

d_z: i16 ,

on_ground: bool ,

 } impl PacketContent for PacketRelEntityMoveContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.d_x.write(writer)?;;

total_bytes += self.d_y.write(writer)?;;

total_bytes += self.d_z.write(writer)?;;

total_bytes += self.on_ground.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let d_x : i16 = PacketContent :: read ( reader ) ?;;

let d_y : i16 = PacketContent :: read ( reader ) ?;;

let d_z : i16 = PacketContent :: read ( reader ) ?;;

let on_ground : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, d_x, d_y, d_z, on_ground } ) } }
