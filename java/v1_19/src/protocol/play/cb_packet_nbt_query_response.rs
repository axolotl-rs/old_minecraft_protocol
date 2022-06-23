use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketNbtQueryResponse ; impl Packet for CbPacketNbtQueryResponse { type PacketIDType = i32 ; type PacketContent = PacketNbtQueryResponseContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 96 } } pub struct PacketNbtQueryResponseContent { pub transaction_id: minecraft_data::data::VarInt ,

pub nbt: minecraft_data::data::nbt::OptionalNbt ,

 } impl PacketContent for PacketNbtQueryResponseContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.nbt.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let transaction_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let nbt : minecraft_data::data::nbt::OptionalNbt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, nbt } ) } }
