use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketUnlockRecipes ; impl Packet for CbPacketUnlockRecipes { type PacketIDType = i32 ; type PacketContent = PacketUnlockRecipesContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 57 } } pub struct PacketUnlockRecipesContent { pub action: minecraft_protocol::data::VarInt ,

pub crafting_book_open: bool ,

pub filtering_craftable: bool ,

pub smelting_book_open: bool ,

pub filtering_smeltable: bool ,

pub blast_furnace_open: bool ,

pub filtering_blast_furnace: bool ,

pub smoker_book_open: bool ,

pub filtering_smoker: bool ,

pub recipes_1: PacketUnlockRecipesContentArray ,

pub recipes_2: PacketUnlockRecipesContentContent ,

 } impl PacketContent for PacketUnlockRecipesContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.action.write(writer)?;;

total_bytes += self.crafting_book_open.write(writer)?;;

total_bytes += self.filtering_craftable.write(writer)?;;

total_bytes += self.smelting_book_open.write(writer)?;;

total_bytes += self.filtering_smeltable.write(writer)?;;

total_bytes += self.blast_furnace_open.write(writer)?;;

total_bytes += self.filtering_blast_furnace.write(writer)?;;

total_bytes += self.smoker_book_open.write(writer)?;;

total_bytes += self.filtering_smoker.write(writer)?;;

total_bytes += self.recipes_1.write(writer)?;;

total_bytes += self.recipes_2.switch_write(false,writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let action : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let crafting_book_open : bool = PacketContent :: read ( reader ) ?;;

let filtering_craftable : bool = PacketContent :: read ( reader ) ?;;

let smelting_book_open : bool = PacketContent :: read ( reader ) ?;;

let filtering_smeltable : bool = PacketContent :: read ( reader ) ?;;

let blast_furnace_open : bool = PacketContent :: read ( reader ) ?;;

let filtering_blast_furnace : bool = PacketContent :: read ( reader ) ?;;

let smoker_book_open : bool = PacketContent :: read ( reader ) ?;;

let filtering_smoker : bool = PacketContent :: read ( reader ) ?;;

let recipes_1 : PacketUnlockRecipesContentArray = PacketContent :: read ( reader ) ?;;

let recipes_2 : PacketUnlockRecipesContentContent = PacketSwitch::switch_read(&action,reader)?;;

 Ok ( Self { action, crafting_book_open, filtering_craftable, smelting_book_open, filtering_smeltable, blast_furnace_open, filtering_blast_furnace, smoker_book_open, filtering_smoker, recipes_1, recipes_2 } ) } } pub type PacketUnlockRecipesContentArray = Vec <String >;

pub enum PacketUnlockRecipesContentContent { } impl PacketSwitch for PacketUnlockRecipesContentContent { type CompareType = minecraft_protocol::data::VarInt ; fn switch_read < Reader : BufRead > ( compare_to : & Self :: CompareType , reader : & mut Reader ) -> std :: io :: Result < Self > where Self : Sized { todo ! ( ) } fn switch_write < Writer : Write > ( self , write_compare : bool , writer : & mut Writer ) -> std :: io :: Result < usize > where Self : Sized { todo ! ( ) } }
