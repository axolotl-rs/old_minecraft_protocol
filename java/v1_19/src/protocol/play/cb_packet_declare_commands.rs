use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketDeclareCommands ; impl Packet for CbPacketDeclareCommands { type PacketIDType = i32 ; type PacketContent = PacketDeclareCommandsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 18 } } pub struct PacketDeclareCommandsContent { pub nodes: PacketDeclareCommandsContentArray ,

pub root_index: minecraft_protocol::data::VarInt ,

 } impl PacketContent for PacketDeclareCommandsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.nodes.write(writer)?;;

total_bytes += self.root_index.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let nodes : PacketDeclareCommandsContentArray = PacketContent :: read ( reader ) ?;;

let root_index : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

 Ok ( Self { nodes, root_index } ) } } pub type PacketDeclareCommandsContentArray = Vec <PacketDeclareCommandsContentArrayContent >; pub struct PacketDeclareCommandsContentArrayContent { pub flags: minecraft_protocol::data::bitfield::BitField ,

pub children: PacketDeclareCommandsContentArrayContentArray ,

pub redirect_node: PacketDeclareCommandsContentArrayContentContent ,

pub extra_node_data: PacketDeclareCommandsContentArrayContentContent ,

 } impl PacketContent for PacketDeclareCommandsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.flags.write(writer)?;;

total_bytes += self.children.write(writer)?;;

total_bytes += self.redirect_node.switch_write(false,writer)?;;

total_bytes += self.extra_node_data.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let flags : minecraft_protocol::data::bitfield::BitField = PacketContent :: read ( reader ) ?;;

let children : PacketDeclareCommandsContentArrayContentArray = PacketContent :: read ( reader ) ?;;

let redirect_node : PacketDeclareCommandsContentArrayContentContent = PacketSwitch::switch_read(&not_found,reader)?;;

let extra_node_data : PacketDeclareCommandsContentArrayContentContent = PacketSwitch::switch_read(&not_found,reader)?;;

 Ok ( Self { flags, children, redirect_node, extra_node_data } ) } } pub type PacketDeclareCommandsContentArrayContentArray = Vec <minecraft_protocol::data::VarInt >;

pub enum PacketDeclareCommandsContentArrayContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (minecraft_protocol::data::VarInt ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContent { /// This switch variant requires a value 0 in the compare_to field

 Switch0 (minecraft_protocol::data::Void ) ,

/// This switch variant requires a value 1 in the compare_to field

 Switch1 (String ) ,

/// This switch variant requires a value 2 in the compare_to field

 Switch2 {

 name: String ,

parser: String ,

properties: PacketDeclareCommandsContentArrayContentContentContentContent ,

suggests: PacketDeclareCommandsContentArrayContentContentContentContent ,

 } ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } } pub enum PacketDeclareCommandsContentArrayContentContentContentContent { /// This switch variant requires a value minecraft:entity in the compare_to field

 MinecraftEntity (i8 ) ,

/// This switch variant requires a value brigadier:double in the compare_to field

 BrigadierDouble {

 flags: minecraft_protocol::data::bitfield::BitField ,

min: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

max: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

 } ,

/// This switch variant requires a value brigadier:float in the compare_to field

 BrigadierFloat {

 flags: minecraft_protocol::data::bitfield::BitField ,

min: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

max: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

 } ,

/// This switch variant requires a value brigadier:integer in the compare_to field

 BrigadierInteger {

 flags: minecraft_protocol::data::bitfield::BitField ,

min: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

max: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

 } ,

/// This switch variant requires a value brigadier:long in the compare_to field

 BrigadierLong {

 flags: minecraft_protocol::data::bitfield::BitField ,

min: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

max: PacketDeclareCommandsContentArrayContentContentContentContentContentContent ,

 } ,

/// This switch variant requires a value brigadier:string in the compare_to field

 BrigadierString (minecraft_protocol::data::VarInt ) ,

/// This switch variant requires a value minecraft:range in the compare_to field

 MinecraftRange (bool ) ,

/// This switch variant requires a value minecraft:score_holder in the compare_to field

 MinecraftScoreHolder (i8 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContent { type CompareType = String ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } } pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (f64 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (f64 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (f32 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (f32 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (i32 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (i32 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (i64 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (i64 ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }

pub enum PacketDeclareCommandsContentArrayContentContentContentContent { /// This switch variant requires a value 1 in the compare_to field

 Switch1 (String ) ,

 } impl PacketSwitch for PacketDeclareCommandsContentArrayContentContentContentContent { type CompareType = void ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
