mod cb_packet_trade_list { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTradeList ; impl Packet for CbPacketTradeList { type PacketIDType = i32 ; type PacketContent = PacketTradeListContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 40 } } pub struct PacketTradeListContent { window_id: minecraft_data::common::data::VarInt ,

trades: PacketTradeListContentArray ,

villager_level: minecraft_data::common::data::VarInt ,

experience: minecraft_data::common::data::VarInt ,

is_regular_villager: bool ,

can_restock: bool ,

 } impl PacketContent for PacketTradeListContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.trades.write(writer)?;;

total_bytes += self.villager_level.write(writer)?;;

total_bytes += self.experience.write(writer)?;;

total_bytes += self.is_regular_villager.write(writer)?;;

total_bytes += self.can_restock.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let trades : PacketTradeListContentArray = PacketContent :: read ( reader ) ?;;

let villager_level : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let experience : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let is_regular_villager : bool = PacketContent :: read ( reader ) ?;;

let can_restock : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, trades, villager_level, experience, is_regular_villager, can_restock } ) } } pub type PacketTradeListContentArray = Vec <PacketTradeListContentArrayContent >; pub struct PacketTradeListContentArrayContent { input_item_1: crate::protocol::types::slot::Slot ,

output_item: crate::protocol::types::slot::Slot ,

input_item_2: void ,

trade_disabled: bool ,

nb_trade_uses: i32 ,

maximum_nb_trade_uses: i32 ,

xp: i32 ,

special_price: i32 ,

price_multiplier: f32 ,

demand: i32 ,

 } impl PacketContent for PacketTradeListContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.input_item_1.write(writer)?;;

total_bytes += self.output_item.write(writer)?;;

total_bytes += self.input_item_2.write(writer)?;;

total_bytes += self.trade_disabled.write(writer)?;;

total_bytes += self.nb_trade_uses.write(writer)?;;

total_bytes += self.maximum_nb_trade_uses.write(writer)?;;

total_bytes += self.xp.write(writer)?;;

total_bytes += self.special_price.write(writer)?;;

total_bytes += self.price_multiplier.write(writer)?;;

total_bytes += self.demand.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let input_item_1 : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

let output_item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

let input_item_2 : void = PacketContent :: read ( reader ) ?;;

let trade_disabled : bool = PacketContent :: read ( reader ) ?;;

let nb_trade_uses : i32 = PacketContent :: read ( reader ) ?;;

let maximum_nb_trade_uses : i32 = PacketContent :: read ( reader ) ?;;

let xp : i32 = PacketContent :: read ( reader ) ?;;

let special_price : i32 = PacketContent :: read ( reader ) ?;;

let price_multiplier : f32 = PacketContent :: read ( reader ) ?;;

let demand : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { input_item_1, output_item, input_item_2, trade_disabled, nb_trade_uses, maximum_nb_trade_uses, xp, special_price, price_multiplier, demand } ) } }

 }

 pub use cb_packet_trade_list ::*;
