use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketCraftRecipeRequest ; impl Packet for SbPacketCraftRecipeRequest { type PacketIDType = i32 ; type PacketContent = PacketCraftRecipeRequestContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 24 } } pub struct PacketCraftRecipeRequestContent { pub window_id: i8 ,

pub recipe: String ,

pub make_all: bool ,

 } impl PacketContent for PacketCraftRecipeRequestContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.recipe.write(writer)?;;

total_bytes += self.make_all.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let recipe : String = PacketContent :: read ( reader ) ?;;

let make_all : bool = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, recipe, make_all } ) } }
