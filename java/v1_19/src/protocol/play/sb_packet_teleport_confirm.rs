use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketTeleportConfirm ; impl Packet for SbPacketTeleportConfirm { type PacketIDType = i32 ; type PacketContent = PacketTeleportConfirmContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 0 } } pub struct PacketTeleportConfirmContent { pub teleport_id: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketTeleportConfirmContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.teleport_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let teleport_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { teleport_id } ) } }
