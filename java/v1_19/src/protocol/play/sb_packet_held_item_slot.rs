use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketHeldItemSlot ; impl Packet for SbPacketHeldItemSlot { type PacketIDType = i32 ; type PacketContent = PacketHeldItemSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 37 } } pub struct PacketHeldItemSlotContent { pub slot_id: i16 ,

 } impl PacketContent for PacketHeldItemSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot_id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let slot_id : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot_id } ) } }
