use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSetTitleTime ; impl Packet for CbPacketSetTitleTime { type PacketIDType = i32 ; type PacketContent = PacketSetTitleTimeContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 91 } } pub struct PacketSetTitleTimeContent { fade_in: i32 ,

stay: i32 ,

fade_out: i32 ,

 } impl PacketContent for PacketSetTitleTimeContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.fade_in.write(writer)?;;

total_bytes += self.stay.write(writer)?;;

total_bytes += self.fade_out.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let fade_in : i32 = PacketContent :: read ( reader ) ?;;

let stay : i32 = PacketContent :: read ( reader ) ?;;

let fade_out : i32 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { fade_in, stay, fade_out } ) } }
