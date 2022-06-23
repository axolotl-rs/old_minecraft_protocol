mod cb_packet_craft_progress_bar { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketCraftProgressBar ; impl Packet for CbPacketCraftProgressBar { type PacketIDType = i32 ; type PacketContent = PacketCraftProgressBarContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 21 } } pub struct PacketCraftProgressBarContent { window_id: u8 ,

property: i16 ,

value: i16 ,

 } impl PacketContent for PacketCraftProgressBarContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.window_id.write(writer)?;;

total_bytes += self.property.write(writer)?;;

total_bytes += self.value.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let window_id : u8 = PacketContent :: read ( reader ) ?;;

let property : i16 = PacketContent :: read ( reader ) ?;;

let value : i16 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { window_id, property, value } ) } }

 }

 pub use cb_packet_craft_progress_bar ::*;
