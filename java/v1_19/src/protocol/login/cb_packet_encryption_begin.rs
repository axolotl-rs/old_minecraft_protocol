use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEncryptionBegin ; impl Packet for CbPacketEncryptionBegin { type PacketIDType = i32 ; type PacketContent = PacketEncryptionBeginContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketEncryptionBeginContent { server_id: String ,

public_key: bytes::Bytes ,

verify_token: bytes::Bytes ,

 } impl PacketContent for PacketEncryptionBeginContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.server_id.write(writer)?;;

total_bytes += self.public_key.write(writer)?;;

total_bytes += self.verify_token.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let server_id : String = PacketContent :: read ( reader ) ?;;

let public_key : bytes::Bytes = PacketContent :: read ( reader ) ?;;

let verify_token : bytes::Bytes = PacketContent :: read ( reader ) ?;;

 Ok ( Self { server_id, public_key, verify_token } ) } }
