mod sb_packet_settings { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSettings ; impl Packet for SbPacketSettings { type PacketIDType = i32 ; type PacketContent = PacketSettingsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 5 } } pub struct PacketSettingsContent { locale: String ,

view_distance: i8 ,

chat_flags: minecraft_data::common::data::VarInt ,

chat_colors: bool ,

skin_parts: u8 ,

main_hand: minecraft_data::common::data::VarInt ,

enable_text_filtering: bool ,

enable_server_listing: bool ,

 } impl PacketContent for PacketSettingsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.locale.write(writer)?;;

total_bytes += self.view_distance.write(writer)?;;

total_bytes += self.chat_flags.write(writer)?;;

total_bytes += self.chat_colors.write(writer)?;;

total_bytes += self.skin_parts.write(writer)?;;

total_bytes += self.main_hand.write(writer)?;;

total_bytes += self.enable_text_filtering.write(writer)?;;

total_bytes += self.enable_server_listing.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let locale : String = PacketContent :: read ( reader ) ?;;

let view_distance : i8 = PacketContent :: read ( reader ) ?;;

let chat_flags : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let chat_colors : bool = PacketContent :: read ( reader ) ?;;

let skin_parts : u8 = PacketContent :: read ( reader ) ?;;

let main_hand : minecraft_data::common::data::VarInt = PacketContent :: read ( reader ) ?;;

let enable_text_filtering : bool = PacketContent :: read ( reader ) ?;;

let enable_server_listing : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { locale, view_distance, chat_flags, chat_colors, skin_parts, main_hand, enable_text_filtering, enable_server_listing } ) } }

 }

 pub use sb_packet_settings ::*;
