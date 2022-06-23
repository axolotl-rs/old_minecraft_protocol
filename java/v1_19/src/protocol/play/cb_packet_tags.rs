mod cb_packet_tags { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketTags ; impl Packet for CbPacketTags { type PacketIDType = i32 ; type PacketContent = PacketTagsContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 103 } } pub struct PacketTagsContent { tags: PacketTagsContentArray ,

 } impl PacketContent for PacketTagsContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.tags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let tags : PacketTagsContentArray = PacketContent :: read ( reader ) ?;;

 Ok ( Self { tags } ) } } pub type PacketTagsContentArray = Vec <PacketTagsContentArrayContent >; pub struct PacketTagsContentArrayContent { tag_type: String ,

tags: crate::protocol::types::tags::Tags ,

 } impl PacketContent for PacketTagsContentArrayContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> Result < usize > { let mut total_bytes = 0 ; total_bytes += self.tag_type.write(writer)?;;

total_bytes += self.tags.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> Result < Self > { let tag_type : String = PacketContent :: read ( reader ) ?;;

let tags : crate::protocol::types::tags::Tags = PacketContent :: read ( reader ) ?;;

 Ok ( Self { tag_type, tags } ) } }

 }

 pub use cb_packet_tags ::*;
