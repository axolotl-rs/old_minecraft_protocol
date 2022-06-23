use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCollect ; impl Packet for CbPacketCollect { type PacketIDType = i32 ; type PacketContent = PacketCollectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 97 } } pub struct PacketCollectContent { collected_entity_id: minecraft_data::data::VarInt ,

collector_entity_id: minecraft_data::data::VarInt ,

pickup_item_count: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketCollectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.collected_entity_id.write(writer)?;;

total_bytes += self.collector_entity_id.write(writer)?;;

total_bytes += self.pickup_item_count.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let collected_entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let collector_entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let pickup_item_count : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { collected_entity_id, collector_entity_id, pickup_item_count } ) } }
