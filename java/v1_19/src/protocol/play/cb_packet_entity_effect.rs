use minecraft_protocol :: protocol :: PacketContent ; use minecraft_protocol :: protocol :: PacketSwitch ; use minecraft_protocol :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketEntityEffect ; impl Packet for CbPacketEntityEffect { type PacketIDType = i32 ; type PacketContent = PacketEntityEffectContent ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 101 } } pub struct PacketEntityEffectContent { pub entity_id: minecraft_protocol::data::VarInt ,

pub effect_id: minecraft_protocol::data::VarInt ,

pub amplifier: i8 ,

pub duration: minecraft_protocol::data::VarInt ,

pub hide_particles: i8 ,

 } impl PacketContent for PacketEntityEffectContent { fn write < Writer : Write > ( self , writer : & mut Writer ) -> std :: io :: Result < usize > { let mut total_bytes = 0 ; total_bytes += self.entity_id.write(writer)?;;

total_bytes += self.effect_id.write(writer)?;;

total_bytes += self.amplifier.write(writer)?;;

total_bytes += self.duration.write(writer)?;;

total_bytes += self.hide_particles.write(writer)?;;

 Ok ( total_bytes ) } fn read < Reader : BufRead > ( reader : & mut Reader ) -> std :: io :: Result < Self > { let entity_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let effect_id : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let amplifier : i8 = PacketContent :: read ( reader ) ?;;

let duration : minecraft_protocol::data::VarInt = PacketContent :: read ( reader ) ?;;

let hide_particles : i8 = PacketContent :: read ( reader ) ?;;

 Ok ( Self { entity_id, effect_id, amplifier, duration, hide_particles } ) } }
