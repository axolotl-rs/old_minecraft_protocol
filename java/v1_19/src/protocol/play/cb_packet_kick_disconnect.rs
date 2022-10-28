use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketKickDisconnect ; impl Packet for CbPacketKickDisconnect { type PacketIDType = i32 ; type PacketContent = PacketKickDisconnectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 26 } } pub struct PacketKickDisconnectContent { pub reason: String ,

 } impl PacketContent for PacketKickDisconnectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.reason.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let reason : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { reason } ) } }
