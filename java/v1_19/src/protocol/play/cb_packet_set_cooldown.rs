use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetCooldown ; impl Packet for CbPacketSetCooldown { type PacketIDType = i32 ; type PacketContent = PacketSetCooldownContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 23 } } pub struct PacketSetCooldownContent { pub item_id: minecraft_data::data::VarInt ,

pub cooldown_ticks: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketSetCooldownContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.item_id.write(writer)?;;

total_bytes += self.cooldown_ticks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let item_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let cooldown_ticks : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { item_id, cooldown_ticks } ) } }
