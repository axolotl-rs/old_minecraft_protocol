use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSettings ; impl Packet for SbPacketSettings { type PacketIDType = i32 ; type PacketContent = PacketSettingsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 5 } } pub struct PacketSettingsContent { pub locale: String ,

pub view_distance: i8 ,

pub chat_flags: minecraft_data::data::VarInt ,

pub chat_colors: bool ,

pub skin_parts: u8 ,

pub main_hand: minecraft_data::data::VarInt ,

pub enable_text_filtering: bool ,

pub enable_server_listing: bool ,

 } impl PacketContent for PacketSettingsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.locale.write(writer)?;;

total_bytes += self.view_distance.write(writer)?;;

total_bytes += self.chat_flags.write(writer)?;;

total_bytes += self.chat_colors.write(writer)?;;

total_bytes += self.skin_parts.write(writer)?;;

total_bytes += self.main_hand.write(writer)?;;

total_bytes += self.enable_text_filtering.write(writer)?;;

total_bytes += self.enable_server_listing.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let locale : String = PacketContent :: read ( reader ) ?;;

let view_distance : i8 = PacketContent :: read ( reader ) ?;;

let chat_flags : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let chat_colors : bool = PacketContent :: read ( reader ) ?;;

let skin_parts : u8 = PacketContent :: read ( reader ) ?;;

let main_hand : minecraft_data::data::VarInt = PacketContent :: read ( reader ) ?;;

let enable_text_filtering : bool = PacketContent :: read ( reader ) ?;;

let enable_server_listing : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { locale, view_distance, chat_flags, chat_colors, skin_parts, main_hand, enable_text_filtering, enable_server_listing } ) } }
