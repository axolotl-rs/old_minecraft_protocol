use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTeams ; impl Packet for CbPacketTeams { type PacketIDType = i32 ; type PacketContent = PacketTeamsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 85 } } pub struct PacketTeamsContent { team: String ,

mode: i8 ,

name: PacketTeamsContentContent ,

friendly_fire: PacketTeamsContentContent ,

name_tag_visibility: PacketTeamsContentContent ,

collision_rule: PacketTeamsContentContent ,

formatting: PacketTeamsContentContent ,

prefix: PacketTeamsContentContent ,

suffix: PacketTeamsContentContent ,

players: PacketTeamsContentContent ,

 } impl PacketContent for PacketTeamsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.team.write(writer)?;;

total_bytes += self.mode.write(writer)?;;

total_bytes += self.name.switch_write(false,writer)?;;

total_bytes += self.friendly_fire.switch_write(false,writer)?;;

total_bytes += self.name_tag_visibility.switch_write(false,writer)?;;

total_bytes += self.collision_rule.switch_write(false,writer)?;;

total_bytes += self.formatting.switch_write(false,writer)?;;

total_bytes += self.prefix.switch_write(false,writer)?;;

total_bytes += self.suffix.switch_write(false,writer)?;;

total_bytes += self.players.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let team : String = PacketContent :: read ( reader ) ?;;

let mode : i8 = PacketContent :: read ( reader ) ?;;

let name : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let friendly_fire : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let name_tag_visibility : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let collision_rule : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let formatting : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let prefix : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let suffix : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

let players : PacketTeamsContentContent = PacketSwitch::switch_read(&mode,reader)?;;

 Ok ( Self { team, mode, name, friendly_fire, name_tag_visibility, collision_rule, formatting, prefix, suffix, players } ) } } pub enum PacketTeamsContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (i8 ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (i8 ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

/// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_data::data::VarInt ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (minecraft_data::data::VarInt ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

/// This switch variant requires a value 0 in the compare_to field

 Switch0 (String ) ,

 } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketTeamsContentContent { } impl PacketSwitch for PacketTeamsContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
