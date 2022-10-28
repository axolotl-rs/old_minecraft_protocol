use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketEntityAction ; impl Packet for SbPacketEntityAction { type PacketIDType = i32 ; type PacketContent = PacketEntityActionContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 27 } } pub struct PacketEntityActionContent { pub entity_id: minecraft_protocol::data::VarInt ,

pub action_id: minecraft_protocol::data::VarInt ,

pub jump_boost: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketEntityActionContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.action_id.write(writer)?;;

total_bytes += self.jump_boost.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let action_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let jump_boost : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, action_id, jump_boost } ) } }
