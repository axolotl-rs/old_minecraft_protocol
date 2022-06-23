use minecraft_data :: protocol :: PacketContent ; use minecraft_data :: protocol :: PacketSwitch ; use minecraft_data :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTags ; impl Packet for CbPacketTags { type PacketIDType = i32 ; type PacketContent = PacketTagsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 103 } } pub struct PacketTagsContent { pub tags: PacketTagsContentArray ,

 } impl PacketContent for PacketTagsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.tags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let tags : PacketTagsContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { tags } ) } } pub type PacketTagsContentArray = Vec <PacketTagsContentArrayContent >; pub struct PacketTagsContentArrayContent { pub tag_type: String ,

pub tags: crate::protocol::types::tags::Tags ,

 } impl PacketContent for PacketTagsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.tag_type.write(writer)?;;

total_bytes += self.tags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let tag_type : String = PacketContent :: read ( reader ) ?;;

let tags : crate::protocol::types::tags::Tags = PacketContent :: read ( reader ) ?;;

 Ok ( Self { tag_type, tags } ) } }
