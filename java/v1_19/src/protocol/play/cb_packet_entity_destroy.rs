use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityDestroy ; impl Packet for CbPacketEntityDestroy { type PacketIDType = i32 ; type PacketContent = PacketEntityDestroyContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 58 } } pub struct PacketEntityDestroyContent { pub entity_ids: PacketEntityDestroyContentArray ,

 } impl PacketContent for PacketEntityDestroyContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_ids.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_ids : PacketEntityDestroyContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_ids } ) } } pub type PacketEntityDestroyContentArray = Vec <minecraft_data::data::VarInt >;
