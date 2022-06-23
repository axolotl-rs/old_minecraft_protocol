use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUpdateHealth ; impl Packet for CbPacketUpdateHealth { type PacketIDType = i32 ; type PacketContent = PacketUpdateHealthContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 82 } } pub struct PacketUpdateHealthContent { health: f32 ,

food: minecraft_data::data::VarInt ,

food_saturation: f32 ,

 } impl PacketContent for PacketUpdateHealthContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.health.write(writer)?;;

total_bytes += self.food.write(writer)?;;

total_bytes += self.food_saturation.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let health : f32 = PacketContent :: read ( reader ) ?;;

let food : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let food_saturation : f32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { health, food, food_saturation } ) } }
