use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketQueryBlockNbt ; impl Packet for SbPacketQueryBlockNbt { type PacketIDType = i32 ; type PacketContent = PacketQueryBlockNbtContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketQueryBlockNbtContent { pub transaction_id: minecraft_protocol::data::VarInt ,

pub location: minecraft_protocol::data::position::Position ,

 } impl PacketContent for PacketQueryBlockNbtContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let transaction_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let location : minecraft_protocol::data::position::Position = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, location } ) } }
