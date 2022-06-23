use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUpdateCommandBlock ; impl Packet for SbPacketUpdateCommandBlock { type PacketIDType = i32 ; type PacketContent = PacketUpdateCommandBlockContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 38 } } pub struct PacketUpdateCommandBlockContent { pub location: minecraft_data::data::position::Position ,

pub command: String ,

pub mode: minecraft_data::data::VarInt ,

pub flags: u8 ,

 } impl PacketContent for PacketUpdateCommandBlockContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.location.write(writer)?;;

total_bytes += self.command.write(writer)?;;

total_bytes += self.mode.write(writer)?;;

total_bytes += self.flags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let command : String = PacketContent :: read ( reader ) ?;;

let mode : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let flags : u8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { location, command, mode, flags } ) } }
