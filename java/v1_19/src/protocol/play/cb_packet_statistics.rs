use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketStatistics ; impl Packet for CbPacketStatistics { type PacketIDType = i32 ; type PacketContent = PacketStatisticsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 7 } } pub struct PacketStatisticsContent { entries: PacketStatisticsContentArray ,

 } impl PacketContent for PacketStatisticsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entries.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entries : PacketStatisticsContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entries } ) } } pub type PacketStatisticsContentArray = Vec <PacketStatisticsContentArrayContent >; pub struct PacketStatisticsContentArrayContent { category_id: minecraft_data::data::VarInt ,

statistic_id: minecraft_data::data::VarInt ,

value: minecraft_data::data::VarInt ,

 } impl PacketContent for PacketStatisticsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.category_id.write(writer)?;;

total_bytes += self.statistic_id.write(writer)?;;

total_bytes += self.value.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let category_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let statistic_id : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let value : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { category_id, statistic_id, value } ) } }
