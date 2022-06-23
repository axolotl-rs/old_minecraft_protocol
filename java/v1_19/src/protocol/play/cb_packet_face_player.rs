mod cb_packet_face_player { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketFacePlayer ; impl Packet for CbPacketFacePlayer { type PacketIDType = i32 ; type PacketContent = PacketFacePlayerContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 55 } } pub struct PacketFacePlayerContent { feet_eyes: minecraft_data::common::data::VarInt ,

x: f64 ,

y: f64 ,

z: f64 ,

is_entity: bool ,

entity_id: PacketFacePlayerContentContent ,

entity_feet_eyes: PacketFacePlayerContentContent ,

 } impl PacketContent for PacketFacePlayerContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.feet_eyes.write(writer)?;;

total_bytes += self.x.write(writer)?;;

total_bytes += self.y.write(writer)?;;

total_bytes += self.z.write(writer)?;;

total_bytes += self.is_entity.write(writer)?;;

total_bytes += self.entity_id.switch_write(false,writer)?;;

total_bytes += self.entity_feet_eyes.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let feet_eyes : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : f64 = PacketContent :: read ( reader ) ?;;

let y : f64 = PacketContent :: read ( reader ) ?;;

let z : f64 = PacketContent :: read ( reader ) ?;;

let is_entity : bool = PacketContent :: read ( reader ) ?;;

let entity_id : PacketFacePlayerContentContent = PacketSwitch::switch_read(&isEntity,reader)?;;

let entity_feet_eyes : PacketFacePlayerContentContent = PacketSwitch::switch_read(&isEntity,reader)?;;

 Ok ( Self { feet_eyes, x, y, z, is_entity, entity_id, entity_feet_eyes } ) } } pub enum PacketFacePlayerContentContent { /// This switch variant requires a value true in the compare_to field

 Switchtrue (minecraft_data::common::data::VarInt ) ,

 } impl PacketSwitch for PacketFacePlayerContentContent { type CompareType = bool ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketFacePlayerContentContent { /// This switch variant requires a value true in the compare_to field

 Switchtrue (String ) ,

 } impl PacketSwitch for PacketFacePlayerContentContent { type CompareType = bool ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

 }

 pub use cb_packet_face_player ::*;
