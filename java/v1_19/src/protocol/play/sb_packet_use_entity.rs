use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketUseEntity ; impl Packet for SbPacketUseEntity { type PacketIDType = i32 ; type PacketContent = PacketUseEntityContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 13 } } pub struct PacketUseEntityContent { pub target: minecraft_protocol::data::VarInt ,

pub mouse: minecraft_protocol::data::VarInt ,

pub x: PacketUseEntityContentContent ,

pub y: PacketUseEntityContentContent ,

pub z: PacketUseEntityContentContent ,

pub hand: PacketUseEntityContentContent ,

pub sneaking: bool ,

 } impl PacketContent for PacketUseEntityContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.target.write(writer)?;;

total_bytes += self.mouse.write(writer)?;;

total_bytes += self.x.switch_write(false,writer)?;;

total_bytes += self.y.switch_write(false,writer)?;;

total_bytes += self.z.switch_write(false,writer)?;;

total_bytes += self.hand.switch_write(false,writer)?;;

total_bytes += self.sneaking.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let target : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let mouse : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let x : PacketUseEntityContentContent = PacketSwitch::switch_read(&mouse,reader)?;;

let y : PacketUseEntityContentContent = PacketSwitch::switch_read(&mouse,reader)?;;

let z : PacketUseEntityContentContent = PacketSwitch::switch_read(&mouse,reader)?;;

let hand : PacketUseEntityContentContent = PacketSwitch::switch_read(&mouse,reader)?;;

let sneaking : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { target, mouse, x, y, z, hand, sneaking } ) } } pub enum PacketUseEntityContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (f32 ) ,

 } impl PacketSwitch for PacketUseEntityContentContent { type CompareType = minecraft_protocol::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketUseEntityContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (f32 ) ,

 } impl PacketSwitch for PacketUseEntityContentContent { type CompareType = minecraft_protocol::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketUseEntityContentContent { /// This switch variant requires a value 2 in the compare_to field

 Switch2 (f32 ) ,

 } impl PacketSwitch for PacketUseEntityContentContent { type CompareType = minecraft_protocol::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketUseEntityContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_protocol::data::VarInt ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 (minecraft_protocol::data::VarInt ) ,

 } impl PacketSwitch for PacketUseEntityContentContent { type CompareType = minecraft_protocol::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
