use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketDisplayedRecipe ; impl Packet for SbPacketDisplayedRecipe { type PacketIDType = i32 ; type PacketContent = PacketDisplayedRecipeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 31 } } pub struct PacketDisplayedRecipeContent { pub recipe_id: String ,

 } impl PacketContent for PacketDisplayedRecipeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.recipe_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let recipe_id : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { recipe_id } ) } }
