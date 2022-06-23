use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSimulationDistance ; impl Packet for CbPacketSimulationDistance { type PacketIDType = i32 ; type PacketContent = PacketSimulationDistanceContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 87 } } pub struct PacketSimulationDistanceContent { pub distance: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketSimulationDistanceContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.distance.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let distance : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { distance } ) } }
