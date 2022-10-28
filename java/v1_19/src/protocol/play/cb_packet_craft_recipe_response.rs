use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCraftRecipeResponse ; impl Packet for CbPacketCraftRecipeResponse { type PacketIDType = i32 ; type PacketContent = PacketCraftRecipeResponseContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 49 } } pub struct PacketCraftRecipeResponseContent { pub window_id: i8 ,

pub recipe: String ,

 } impl PacketContent for PacketCraftRecipeResponseContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.recipe.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let recipe : String = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, recipe } ) } }
