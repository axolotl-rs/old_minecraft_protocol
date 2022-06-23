mod cb_packet_encryption_begin { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEncryptionBegin ; impl Packet for CbPacketEncryptionBegin { type PacketIDType = i32 ; type PacketContent = PacketEncryptionBeginContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 1 } } pub struct PacketEncryptionBeginContent { server_id: String ,

public_key: void ,

verify_token: void ,

 } impl PacketContent for PacketEncryptionBeginContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.server_id.write(writer)?;;

total_bytes += self.public_key.write(writer)?;;

total_bytes += self.verify_token.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let server_id : String = PacketContent :: read ( reader ) ?;;

let public_key : void = PacketContent :: read ( reader ) ?;;

let verify_token : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { server_id, public_key, verify_token } ) } }

 }

 pub use cb_packet_encryption_begin ::*;
