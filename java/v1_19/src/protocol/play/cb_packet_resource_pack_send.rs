use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketResourcePackSend ; impl Packet for CbPacketResourcePackSend { type PacketIDType = i32 ; type PacketContent = PacketResourcePackSendContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 60 } } pub struct PacketResourcePackSendContent { pub url: String ,

pub hash: String ,

pub forced: bool ,

pub prompt_message: void ,

 } impl PacketContent for PacketResourcePackSendContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.url.write(writer)?;;

total_bytes += self.hash.write(writer)?;;

total_bytes += self.forced.write(writer)?;;

total_bytes += self.prompt_message.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let url : String = PacketContent :: read ( reader ) ?;;

let hash : String = PacketContent :: read ( reader ) ?;;

let forced : bool = PacketContent :: read ( reader ) ?;;

let prompt_message : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { url, hash, forced, prompt_message } ) } }
