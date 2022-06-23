use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketRemoveEntityEffect ; impl Packet for CbPacketRemoveEntityEffect { type PacketIDType = i32 ; type PacketContent = PacketRemoveEntityEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 59 } } pub struct PacketRemoveEntityEffectContent { entity_id: minecraft_data::data::VarInt ,

effect_id: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketRemoveEntityEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.effect_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let effect_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, effect_id } ) } }
