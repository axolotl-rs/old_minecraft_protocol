mod cb_packet_stop_sound { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketStopSound ; impl Packet for CbPacketStopSound { type PacketIDType = i32 ; type PacketContent = PacketStopSoundContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 94 } } pub struct PacketStopSoundContent { flags: i8 ,

source: PacketStopSoundContentContent ,

sound: PacketStopSoundContentContent ,

 } impl PacketContent for PacketStopSoundContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

total_bytes += self.source.switch_write(false,writer)?;;

total_bytes += self.sound.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let flags : i8 = PacketContent :: read ( reader ) ?;;

let source : PacketStopSoundContentContent = PacketSwitch::switch_read(&flags,reader)?;;

let sound : PacketStopSoundContentContent = PacketSwitch::switch_read(&flags,reader)?;;

 Ok ( Self { flags, source, sound } ) } } pub enum PacketStopSoundContentContent { /// This switch variant requires a value 3 in the compare_to field

 Switch3 (minecraft_data::common::data::VarInt ) ,

/// This switch variant requires a value 1 in the compare_to field

 Switch1 (minecraft_data::common::data::VarInt ) ,

 } impl PacketSwitch for PacketStopSoundContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketStopSoundContentContent { /// This switch variant requires a value 3 in the compare_to field

 Switch3 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

 } impl PacketSwitch for PacketStopSoundContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

 }

 pub use cb_packet_stop_sound ::*;
