use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldBorderWarningReach ; impl Packet for CbPacketWorldBorderWarningReach { type PacketIDType = i32 ; type PacketContent = PacketWorldBorderWarningReachContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 70 } } pub struct PacketWorldBorderWarningReachContent { warning_blocks: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketWorldBorderWarningReachContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.warning_blocks.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let warning_blocks : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { warning_blocks } ) } }
