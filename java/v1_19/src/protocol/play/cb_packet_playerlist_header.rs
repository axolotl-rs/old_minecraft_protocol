use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketPlayerlistHeader ; impl Packet for CbPacketPlayerlistHeader { type PacketIDType = i32 ; type PacketContent = PacketPlayerlistHeaderContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 95 } } pub struct PacketPlayerlistHeaderContent { pub header: String ,

pub footer: String ,

 } impl PacketContent for PacketPlayerlistHeaderContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.header.write(writer)?;;

total_bytes += self.footer.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let header : String = PacketContent :: read ( reader ) ?;;

let footer : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { header, footer } ) } }
