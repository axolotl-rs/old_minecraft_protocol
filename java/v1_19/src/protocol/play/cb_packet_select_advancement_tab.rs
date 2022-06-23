mod cb_packet_select_advancement_tab { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSelectAdvancementTab ; impl Packet for CbPacketSelectAdvancementTab { type PacketIDType = i32 ; type PacketContent = PacketSelectAdvancementTabContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 64 } } pub struct PacketSelectAdvancementTabContent { id: void ,

 } impl PacketContent for PacketSelectAdvancementTabContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let id : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { id } ) } }

 }

 pub use cb_packet_select_advancement_tab ::*;
