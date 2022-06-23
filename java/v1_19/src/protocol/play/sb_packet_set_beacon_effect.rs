use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetBeaconEffect ; impl Packet for SbPacketSetBeaconEffect { type PacketIDType = i32 ; type PacketContent = PacketSetBeaconEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 36 } } pub struct PacketSetBeaconEffectContent { pub primary_effect: minecraft_data::data::VarInt ,

pub secondary_effect: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketSetBeaconEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.primary_effect.write(writer)?;;

total_bytes += self.secondary_effect.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let primary_effect : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let secondary_effect : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { primary_effect, secondary_effect } ) } }
