mod cb_packet_craft_recipe_response { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCraftRecipeResponse ; impl Packet for CbPacketCraftRecipeResponse { type PacketIDType = i32 ; type PacketContent = PacketCraftRecipeResponseContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 49 } } pub struct PacketCraftRecipeResponseContent { window_id: i8 ,

recipe: String ,

 } impl PacketContent for PacketCraftRecipeResponseContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.recipe.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let recipe : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, recipe } ) } }

 }

 pub use cb_packet_craft_recipe_response ::*;
