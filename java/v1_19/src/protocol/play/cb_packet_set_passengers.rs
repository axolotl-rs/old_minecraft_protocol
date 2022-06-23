use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetPassengers ; impl Packet for CbPacketSetPassengers { type PacketIDType = i32 ; type PacketContent = PacketSetPassengersContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 84 } } pub struct PacketSetPassengersContent { pub entity_id: minecraft_data::data::VarInt ,

pub passengers: PacketSetPassengersContentArray ,

 } impl PacketContent for PacketSetPassengersContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.passengers.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let passengers : PacketSetPassengersContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, passengers } ) } } pub type PacketSetPassengersContentArray = Vec <minecraft_data::data::VarInt >;
