mod sb_packet_displayed_recipe { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketDisplayedRecipe ; impl Packet for SbPacketDisplayedRecipe { type PacketIDType = i32 ; type PacketContent = PacketDisplayedRecipeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 31 } } pub struct PacketDisplayedRecipeContent { recipe_id: String ,

 } impl PacketContent for PacketDisplayedRecipeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.recipe_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let recipe_id : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { recipe_id } ) } }

 }

 pub use sb_packet_displayed_recipe ::*;
