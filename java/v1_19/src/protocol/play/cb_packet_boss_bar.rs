mod cb_packet_boss_bar { use super ::*; use crate :: common :: protocol :: PacketContent ; use crate :: common :: protocol :: PacketSwitch ; use crate :: common :: protocol :: Packet ; use std :: io :: { BufRead , Error , ErrorKind , Result , Write } ; use std :: str :: FromStr ;

 pub struct CbPacketBossBar ; impl Packet for CbPacketBossBar { type PacketIDType = i32 ; type PacketContent = void ; fn packet_id ( ) -> Self :: PacketIDType where Self : Sized { 13 } } }

 pub use cb_packet_boss_bar ::*;
