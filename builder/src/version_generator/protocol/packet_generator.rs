use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::mem;
use std::num::ParseIntError;
use git2::SubmoduleUpdate::Default;
use log::warn;
use minecraft_data_rs::models::protocol::{Packet, PacketGrouping, PacketTypes};
use crate::code_gen::{CompareTo, DataType, GenerateType};
use crate::error::GenError;
use crate::version_generator::protocol::types::{SubTypeResponse, TypesGenerator};

#[derive(Debug, Clone)]
pub enum PacketGroup {
    ClientBound,
    ServerBound,
}

impl Display for PacketGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketGroup::ClientBound => write!(f, "cb"),
            PacketGroup::ServerBound => write!(f, "sb"),
        }
    }
}

pub struct PacketToBuild {
    pub packet: Packet,
    pub grouping: PacketGroup,
    pub packet_id: i32,
}

impl PacketToBuild {
    pub fn into_inner(self) -> (Packet, PacketGroup, i32) {
        (self.packet, self.grouping, self.packet_id)
    }
}

pub struct PacketGenerator<'types> {
    pub type_generator: &'types TypesGenerator,
    pub queue: Vec<PacketToBuild>,
}

impl<'types> PacketGenerator<'types> {
    pub fn new(type_generator: &'types TypesGenerator, packet_grouping: PacketGrouping) -> Result<Self, GenError> {
        let mut queue = Vec::with_capacity(packet_grouping.to_client.types.len() + packet_grouping.to_server.types.len());

        Self::add_types(&mut queue, packet_grouping.to_client, PacketGroup::ClientBound)?;
        Self::add_types(&mut queue, packet_grouping.to_server, PacketGroup::ServerBound)?;


        Ok(Self {
            type_generator,
            queue,
        })
    }
    pub fn generate(&mut self) -> Result<Vec<GenerateType>, GenError> {
        let mut packets = Vec::with_capacity(self.queue.len());
        let mut packets_to_build = mem::take(&mut self.queue);
        for packet_to_build in packets_to_build.into_iter() {
            let (packet, grouping, packet_id) = packet_to_build.into_inner();
            let result = self.type_generator.sub_type(packet.name.clone(), Box::new(packet.data), |d| {
                warn!("A switch was put directly in a packet {}", packet.name);
                CompareTo::Specified {
                    compare_to: packet.name.to_string(),
                    compare_to_type: Box::new(DataType::default()),
                }
            })?;
            let (data_type, children) = match result {
                SubTypeResponse::NotBuiltYet(value) => {
                    warn!("A type is missing {:#?}", value);
                    (DataType::default(), vec![])
                }
                SubTypeResponse::CanNotBuild(data) => {
                    warn!("A type could not be built {:#?}", data);

                    (DataType::default(), vec![])
                }
                SubTypeResponse::AlreadyBuilt(cool) => {
                    (cool, vec![])
                }
                SubTypeResponse::SubType { data_type, generate } => {
                    (data_type, vec![generate])
                }
            };
            packets.push(GenerateType::Packet {
                content_name: format!("{}_{}", grouping, packet.name),
                packet_id,
                data_type,
                children,
            })
        }
        Ok(packets)
    }
    fn add_types(queue: &mut Vec<PacketToBuild>, types: PacketTypes, grouping: PacketGroup) -> Result<(), GenError> {
        let map: HashMap<i32, String> = types.packet_mapper.mapper.try_into()?;
        for (id, name) in map.into_iter() {
            if let Some(value) = types.packet_mapper.switch.fields.get(&name) {
                if let Some(value) = types.types.iter().find(|x| x.name.as_bytes().eq(value.as_bytes())) {
                    queue.push(PacketToBuild {
                        packet: value.clone(),
                        grouping: grouping.clone(),
                        packet_id: id,
                    });
                } else {
                    warn!("Packet {} is not in the packet mapper", name);
                }
            } else {
                warn!("Packet {} is not in the packet mapper", name);
            }
        }
        Ok(())
    }
}