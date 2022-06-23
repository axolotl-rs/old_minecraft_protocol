use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketClientCommand ; impl Packet for SbPacketClientCommand { type PacketIDType = i32 ; type PacketContent = PacketClientCommandContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 4 } } pub struct PacketClientCommandContent { action_id: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketClientCommandContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.action_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let action_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { action_id } ) } }
