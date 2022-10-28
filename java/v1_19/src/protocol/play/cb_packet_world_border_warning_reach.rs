use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderWarningReach ; impl Packet for CbPacketWorldBorderWarningReach { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderWarningReachContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 70 } } pub struct PacketWorldBorderWarningReachContent { pub warning_blocks: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketWorldBorderWarningReachContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.warning_blocks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let warning_blocks : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { warning_blocks } ) } }
