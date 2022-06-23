use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct SbPacketSetCreativeSlot ; impl Packet for SbPacketSetCreativeSlot { type PacketIDType = i32 ; type PacketContent = PacketSetCreativeSlotContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 40 } } pub struct PacketSetCreativeSlotContent { slot: i16 ,

item: crate::protocol::types::slot::Slot ,

 } impl PacketContent for PacketSetCreativeSlotContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.slot.write(writer)?;;

total_bytes += self.item.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let slot : i16 = PacketContent :: read ( reader ) ?;;

let item : crate::protocol::types::slot::Slot = PacketContent :: read ( reader ) ?;;

 Ok ( Self { slot, item } ) } }
