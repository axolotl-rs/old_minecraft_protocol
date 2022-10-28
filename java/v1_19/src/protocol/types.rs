mod slot {

    use minecraft_protocol::protocol::PacketContent;
    use minecraft_protocol::protocol::PacketSwitch;
    use std::io::{BufRead, Write};
    use axolotl_nbt::value::Value;

    pub struct Slot {
        pub present: bool,

        pub content: SlotContent,
    }
    impl PacketContent for Slot {
        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
            let mut total_bytes = 0;
            total_bytes += self.present.write(writer)?;

            total_bytes += self.content.switch_write(false, writer)?;

            Ok(total_bytes)
        }
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
            let present: bool = PacketContent::read(reader)?;

            let content: SlotContent = PacketSwitch::switch_read(&present, reader)?;

            Ok(Self { present, content })
        }
    }
    pub enum SlotContent {
        /// This switch variant requires a value true in the compare_to field
        Switchtrue {
            item_id: minecraft_protocol::data::VarInt,

            item_count: i8,

            nbt_data: minecraft_protocol::data::nbt::OptionalNbt<Value>,
        },

        /// This switch variant requires a value false in the compare_to field
        Switchfalse(minecraft_protocol::data::Void),
    }
    impl PacketSwitch for SlotContent {
        type CompareType = bool;
        fn switch_read<Reader: BufRead>(
            _compare_to: &Self::CompareType,
            _reader: &mut Reader,
        ) -> std::io::Result<Self>
        where
            Self: Sized,
        {
            todo!()
        }
        fn switch_write<Writer: Write>(
            self,
            _write_compare: bool,
            _writer: &mut Writer,
        ) -> std::io::Result<usize>
        where
            Self: Sized,
        {
            todo!()
        }
    }
}

pub use slot::*;

mod particle_data {

    use minecraft_protocol::protocol::PacketSwitch;
    use std::io::{BufRead, Write};

    pub enum ParticleData {
        /// This switch variant requires a value 15 in the compare_to field
        Switch15 {
            from_red: f32,

            from_green: f32,

            from_blue: f32,

            scale: f32,

            to_red: f32,

            to_green: f32,

            to_blue: f32,
        },

        /// This switch variant requires a value 24 in the compare_to field
        Switch24 {
            block_state: minecraft_protocol::data::VarInt,
        },

        /// This switch variant requires a value 3 in the compare_to field
        Switch3 {
            block_state: minecraft_protocol::data::VarInt,
        },

        /// This switch variant requires a value 35 in the compare_to field
        Switch35 {
            item: crate::protocol::types::slot::Slot,
        },

        /// This switch variant requires a value 14 in the compare_to field
        Switch14 {
            red: f32,

            green: f32,

            blue: f32,

            scale: f32,
        },

        /// This switch variant requires a value 2 in the compare_to field
        Switch2 {
            block_state: minecraft_protocol::data::VarInt,
        },

        /// This switch variant requires a value 36 in the compare_to field
        Switch36 {
            origin: minecraft_protocol::data::position::Position,

            position_type: String,

            destination: ParticleDataContentContent,

            ticks: minecraft_protocol::data::VarInt,
        },
    }
    impl PacketSwitch for ParticleData {
        type CompareType = minecraft_protocol::data::VarInt;
        fn switch_read<Reader: BufRead>(
            _compare_to: &Self::CompareType,
            _reader: &mut Reader,
        ) -> std::io::Result<Self>
        where
            Self: Sized,
        {
            todo!()
        }
        fn switch_write<Writer: Write>(
            self,
            _write_compare: bool,
            _writer: &mut Writer,
        ) -> std::io::Result<usize>
        where
            Self: Sized,
        {
            todo!()
        }
    }
    pub enum ParticleDataContentContent {
        /// This switch variant requires a value minecraft:entity in the compare_to field
        MinecraftEntity(minecraft_protocol::data::VarInt),

        /// This switch variant requires a value minecraft:block in the compare_to field
        MinecraftBlock(minecraft_protocol::data::position::Position),
    }
    impl PacketSwitch for ParticleDataContentContent {
        type CompareType = String;
        fn switch_read<Reader: BufRead>(
            _compare_to: &Self::CompareType,
            _reader: &mut Reader,
        ) -> std::io::Result<Self>
        where
            Self: Sized,
        {
            todo!()
        }
        fn switch_write<Writer: Write>(
            self,
            _write_compare: bool,
            _writer: &mut Writer,
        ) -> std::io::Result<usize>
        where
            Self: Sized,
        {
            todo!()
        }
    }
}

pub use particle_data::*;

mod ingredient {

    pub type Ingredient = Vec<crate::protocol::types::slot::Slot>;
}

pub use ingredient::*;

mod minecraft_smelting_format {

    use minecraft_protocol::protocol::PacketContent;

    use std::io::{BufRead, Write};

    pub struct MinecraftSmeltingFormat {
        pub group: String,

        pub ingredient: crate::protocol::types::ingredient::Ingredient,

        pub result: crate::protocol::types::slot::Slot,

        pub experience: f32,

        pub cook_time: minecraft_protocol::data::VarInt,
    }
    impl PacketContent for MinecraftSmeltingFormat {
        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
            let mut total_bytes = 0;
            total_bytes += self.group.write(writer)?;

            total_bytes += self.ingredient.write(writer)?;

            total_bytes += self.result.write(writer)?;

            total_bytes += self.experience.write(writer)?;

            total_bytes += self.cook_time.write(writer)?;

            Ok(total_bytes)
        }
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
            let group: String = PacketContent::read(reader)?;

            let ingredient: crate::protocol::types::ingredient::Ingredient =
                PacketContent::read(reader)?;

            let result: crate::protocol::types::slot::Slot = PacketContent::read(reader)?;

            let experience: f32 = PacketContent::read(reader)?;

            let cook_time: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

            Ok(Self {
                group,
                ingredient,
                result,
                experience,
                cook_time,
            })
        }
    }
}

pub use minecraft_smelting_format::*;

mod tags {

    use minecraft_protocol::protocol::PacketContent;

    use std::io::{BufRead, Write};

    pub type Tags = Vec<TagsContent>;
    pub struct TagsContent {
        pub tag_name: String,

        pub entries: TagsContentArray,
    }
    impl PacketContent for TagsContent {
        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
            let mut total_bytes = 0;
            total_bytes += self.tag_name.write(writer)?;

            total_bytes += self.entries.write(writer)?;

            Ok(total_bytes)
        }
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
            let tag_name: String = PacketContent::read(reader)?;

            let entries: TagsContentArray = PacketContent::read(reader)?;

            Ok(Self { tag_name, entries })
        }
    }
    pub type TagsContentArray = Vec<minecraft_protocol::data::VarInt>;
}

pub use tags::*;

mod chunk_block_entity {

    use minecraft_protocol::protocol::PacketContent;

    use std::io::{BufRead, Write};
    use axolotl_nbt::value::Value;

    pub struct ChunkBlockEntity {
        pub content: minecraft_protocol::data::bitfield::BitField,

        pub y: i16,

        pub data_type: minecraft_protocol::data::VarInt,

        pub nbt_data: minecraft_protocol::data::nbt::OptionalNbt<Value>,
    }
    impl PacketContent for ChunkBlockEntity {
        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
            let mut total_bytes = 0;
            total_bytes += self.content.write(writer)?;

            total_bytes += self.y.write(writer)?;

            total_bytes += self.data_type.write(writer)?;

            total_bytes += self.nbt_data.write(writer)?;

            Ok(total_bytes)
        }
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
            let content: minecraft_protocol::data::bitfield::BitField =
                PacketContent::read(reader)?;

            let y: i16 = PacketContent::read(reader)?;

            let data_type: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

            let nbt_data: minecraft_protocol::data::nbt::OptionalNbt<Value> = PacketContent::read(reader)?;

            Ok(Self {
                content,
                y,
                data_type,
                nbt_data,
            })
        }
    }
}

pub use chunk_block_entity::*;

mod particle {

    use minecraft_protocol::protocol::PacketContent;
    use minecraft_protocol::protocol::PacketSwitch;
    use std::io::{BufRead, Write};

    pub struct Particle {
        pub particle_id: minecraft_protocol::data::VarInt,

        pub data: crate::protocol::types::particle_data::ParticleData,
    }
    impl PacketContent for Particle {
        fn write<Writer: Write>(self, writer: &mut Writer) -> std::io::Result<usize> {
            let mut total_bytes = 0;
            total_bytes += self.particle_id.write(writer)?;

            total_bytes += self.data.switch_write(false, writer)?;

            Ok(total_bytes)
        }
        fn read<Reader: BufRead>(reader: &mut Reader) -> std::io::Result<Self> {
            let particle_id: minecraft_protocol::data::VarInt = PacketContent::read(reader)?;

            let data: crate::protocol::types::particle_data::ParticleData =
                PacketSwitch::switch_read(&particle_id, reader)?;

            Ok(Self { particle_id, data })
        }
    }
}

pub use particle::*;
