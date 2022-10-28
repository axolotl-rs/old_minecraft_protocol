use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketStopSound ; impl Packet for CbPacketStopSound { type PacketIDType = i32 ; type PacketContent = PacketStopSoundContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 94 } } pub struct PacketStopSoundContent { pub flags: i8 ,

pub source: PacketStopSoundContentContent ,

pub sound: PacketStopSoundContentContent ,

 } impl PacketContent for PacketStopSoundContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

total_bytes += self.source.switch_write(false,writer)?;;

total_bytes += self.sound.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let flags : i8 = PacketContent :: read ( reader ) ?;;

let source : PacketStopSoundContentContent = PacketSwitch::switch_read(&flags,reader)?;;

let sound : PacketStopSoundContentContent = PacketSwitch::switch_read(&flags,reader)?;;

 Ok ( Self { flags, source, sound } ) } } pub enum PacketStopSoundContentContent { /// This switch variant requires a value 3 in the compare_to field

 Switch3 (minecraft_protocol::data::VarInt ) ,

/// This switch variant requires a value 1 in the compare_to field

 Switch1 (minecraft_protocol::data::VarInt ) ,

 } impl PacketSwitch for PacketStopSoundContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketStopSoundContentContent { /// This switch variant requires a value 3 in the compare_to field

 Switch3 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (String ) ,

 } impl PacketSwitch for PacketStopSoundContentContent { type CompareType = i8 ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
