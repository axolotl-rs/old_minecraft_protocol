use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketTabComplete ; impl Packet for SbPacketTabComplete { type PacketIDType = i32 ; type PacketContent = PacketTabCompleteContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 6 } } pub struct PacketTabCompleteContent { pub transaction_id: minecraft_protocol::data::VarInt ,

pub text: String ,

 } impl PacketContent for PacketTabCompleteContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.transaction_id.write(writer)?;;

total_bytes += self.text.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let transaction_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let text : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { transaction_id, text } ) } }
