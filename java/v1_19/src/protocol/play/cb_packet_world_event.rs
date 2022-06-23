use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketWorldEvent ; impl Packet for CbPacketWorldEvent { type PacketIDType = i32 ; type PacketContent = PacketWorldEventContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 35 } } pub struct PacketWorldEventContent { pub effect_id: i32 ,

pub location: minecraft_data::data::position::Position ,

pub data: i32 ,

pub global: bool ,

 } impl PacketContent for PacketWorldEventContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.effect_id.write(writer)?;;

total_bytes += self.location.write(writer)?;;

total_bytes += self.data.write(writer)?;;

total_bytes += self.global.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let effect_id : i32 = PacketContent :: read ( reader ) ?;;

let location : minecraft_data::data::position::Position = PacketContent :: read ( reader ) ?;;

let data : i32 = PacketContent :: read ( reader ) ?;;

let global : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { effect_id, location, data, global } ) } }
