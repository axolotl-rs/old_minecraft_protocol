use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketSelectAdvancementTab ; impl Packet for CbPacketSelectAdvancementTab { type PacketIDType = i32 ; type PacketContent = PacketSelectAdvancementTabContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 64 } } pub struct PacketSelectAdvancementTabContent { id: void ,

 } impl PacketContent for PacketSelectAdvancementTabContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.id.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let id : void = PacketContent :: read ( reader ) ?;;

 Ok ( Self { id } ) } }
