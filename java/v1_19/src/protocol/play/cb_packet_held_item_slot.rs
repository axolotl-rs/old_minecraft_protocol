use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketHeldItemSlot ; impl Packet for CbPacketHeldItemSlot { type PacketIDType = i32 ; type PacketContent = PacketHeldItemSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 72 } } pub struct PacketHeldItemSlotContent { slot: i8 ,

 } impl PacketContent for PacketHeldItemSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let slot : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot } ) } }
