use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketEnchantItem ; impl Packet for SbPacketEnchantItem { type PacketIDType = i32 ; type PacketContent = PacketEnchantItemContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 7 } } pub struct PacketEnchantItemContent { window_id: i8 ,

enchantment: i8 ,

 } impl PacketContent for PacketEnchantItemContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.enchantment.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let window_id : i8 = PacketContent :: read ( reader ) ?;;

let enchantment : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, enchantment } ) } }
