use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateCommandBlockMinecart ; impl Packet for SbPacketUpdateCommandBlockMinecart { type PacketIDType = i32 ; type PacketContent = PacketUpdateCommandBlockMinecartContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 39 } } pub struct PacketUpdateCommandBlockMinecartContent { entity_id: minecraft_data::data::VarInt ,

command: String ,

track_output: bool ,

 } impl PacketContent for PacketUpdateCommandBlockMinecartContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.command.write(writer)?;;

total_bytes += self.track_output.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let command : String = PacketContent :: read ( reader ) ?;;

let track_output : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, command, track_output } ) } }
