use convert_case::{Case, Casing};
use genco::fmt;
use genco::prelude::*;
use minecraft_data_rs::models::protocol::types::TypeName;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CompareTo {
    Specified {
        compare_to: String,
        compare_to_type: Box<DataType>,
    },
    Generic {
        compare_to_type: Box<DataType>,
    },
}

impl Display for CompareTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareTo::Specified { compare_to, .. } => write!(f, "{}", compare_to),
            CompareTo::Generic { .. } => write!(f, "compare_to"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataType {
    pub minecraft_name: String,
    pub inner_type: InnerType,
    pub language_type: LanguageType,
}

impl Default for DataType {
    fn default() -> Self {
        DataType {
            minecraft_name: "void".to_string(),
            inner_type: InnerType::Native,
            language_type: LanguageType::Rust {
                absolute_path: "void".to_string(),
            },
        }
    }
}

impl DataType {
    pub fn new_generated_type(
        minecraft_name: String,
        inner_type: InnerType,
        language_type: LanguageType,
    ) -> Self {
        let string = minecraft_name.to_case(Case::UpperCamel);
        DataType {
            minecraft_name,
            inner_type,
            language_type,
        }
    }
    pub fn new_builtin_type(minecraft_name: String, inner_type: InnerType) -> Self {
        let string = minecraft_name.to_case(Case::UpperCamel);
        DataType {
            minecraft_name,
            inner_type,
            language_type: LanguageType::Rust {
                absolute_path: format!("crate::common::data::{}", string),
            },
        }
    }
    pub fn new_rust_type(minecraft_name: String, rust_type: String) -> Self {
        DataType {
            minecraft_name,
            inner_type: InnerType::Native,
            language_type: LanguageType::Rust {
                absolute_path: rust_type,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LanguageType {
    Rust {
        /// The absolute path of the type. Such as Uuid::Uuid or minecraft_data::generated::types::Slot
        absolute_path: String,
    },
}

impl Display for LanguageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageType::Rust { absolute_path } => write!(f, "{}", absolute_path),
        }
    }
}

impl<'data_type> FormatInto<Rust> for &'data_type DataType {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        match &self.language_type {
            LanguageType::Rust { absolute_path } => {
                tokens.append(quote! {
                    #absolute_path
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum InnerType {
    Container,
    Array,
    Switch {
        // The variable that should be passed into for the compare_to
        compare_to: CompareTo,
    },
    Native,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.to_case(Case::Snake))
    }
}

impl Field {
    pub fn new(name: &TypeName, data_type: DataType) -> Self {
        Field {
            name: Self::create_field_name(name),
            data_type,
        }
    }
    pub fn create_field_name(name_name: &TypeName) -> String {
        match name_name {
            TypeName::Anonymous => "content".to_string(),
            TypeName::Named(name) => {
                if name.eq("type") {
                    "data_type".to_string()
                } else if name.contains(":") {
                    name.replace(":", "_")
                } else {
                    name.to_string()
                }
            }
        }
    }
    pub fn generate_field_definition(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::Snake);
        let data_type = &self.data_type;
        let mut value = quote! {
            #name: #data_type,
        };
        value
    }
    pub fn generate_write(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::Snake);
        let data_type = &self.data_type;
        if let InnerType::Switch { compare_to } = &self.data_type.inner_type {
            quote! {
                total_bytes += #(format!("self.{}.switch_write(false,writer)?;",self))
            }
        } else {
            quote! {
               total_bytes += #(format!("self.{}.write(writer)?;",self))
            }
        }
    }
    pub fn generate_read(&self) -> (Tokens<Rust>, Tokens<Rust>) {
        let name = &self.name.to_case(Case::Snake);
        let data_type = &self.data_type;

        let v1 = if let InnerType::Switch { compare_to } = &self.data_type.inner_type {
            quote! {
                let #name: #data_type = #(format!("PacketSwitch::switch_read(&{},reader)?;",compare_to))
            }
        } else {
            quote! {
                let #name: #data_type = PacketContent::read(reader)?;
            }
        };
        (
            v1,
            quote! {
                #name
            },
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test() {
        let tokens = Field {
            name: "test".to_string(),
            data_type: DataType {
                minecraft_name: "".to_string(),
                inner_type: InnerType::Container,
                language_type: LanguageType::Rust {
                    absolute_path: "".to_string(),
                },
            },
        };

        let variant = SwitchVariant {
            name: "My Variant".to_string(),
            requirement: "12".to_string(),
            switch_variant_type: SwitchVariantType::Container(vec![tokens]),
        };
        println!("{}", variant.generate_variant_def().to_string().unwrap());
    }

    #[test]
    pub fn container_test() {
        // Generate 5 fields
        let mut fields = Vec::with_capacity(5);
        for i in 0..5 {
            fields.push(Field {
                name: format!("field_{}", i),
                data_type: DataType::new_generated_type(
                    format!("field_{}", i),
                    InnerType::Container,
                    LanguageType::Rust {
                        absolute_path: "field_one".to_string(),
                    },
                ),
            });
        }
        // Create a container
        let container = GenerateType::Container {
            content_name: "y Variant".to_string(),
            fields: fields,
            children: vec![],
        };
        println!("{}", container.generate_type().to_string().unwrap());
    }

    #[test]
    pub fn specified_switch_test() {
        // Generate 3 different variants with different requirements One that is Single One that is Container and one that is Void
        let mut variants = Vec::with_capacity(3);

        let mut fields = Vec::with_capacity(5);
        for j in 0..5 {
            fields.push(Field {
                name: format!("field_{}", j),
                data_type: DataType::new_generated_type(
                    format!("field_{}", j),
                    InnerType::Container,
                ),
            });
        }
        variants.push(SwitchVariant {
            name: format!("variant_container"),
            requirement: format!("\"variant_container\""),
            switch_variant_type: SwitchVariantType::Container(fields),
        });
        variants.push(SwitchVariant {
            name: format!("variant_single"),
            requirement: format!("\"variant_single\""),
            switch_variant_type: SwitchVariantType::Single(Field {
                name: format!("field"),
                data_type: DataType::new_generated_type(
                    format!("field"),
                    InnerType::Container,
                    LanguageType::Rust {
                        absolute_path: "field_one".to_string(),
                    },
                ),
            }),
        });
        variants.push(SwitchVariant {
            name: "variant_void".to_string(),
            requirement: "\"void\"".to_string(),
            switch_variant_type: SwitchVariantType::Void,
        });
        // Create a GenerateType::Switch
        let switch = GenerateType::SwitchEnum {
            content_name: "y Variant".to_string(),
            compare_to: CompareTo::Specified {
                compare_to: "test".to_string(),
                compare_to_type: Box::new(DataType::new_rust_type(
                    "string".to_string(),
                    "String".to_string(),
                )),
            },

            variants: variants,
            children: vec![],
        };
        println!("{}", switch.generate_type().to_string().unwrap());
    }
}

#[derive(Debug, Clone)]
pub struct SwitchVariant {
    pub name: String,
    pub requirement: String,
    pub switch_variant_type: SwitchVariantType,
}

impl SwitchVariant {
    pub fn new(requirement: String, variant_type: SwitchVariantType) -> Self {
        SwitchVariant {
            name: SwitchVariant::requirement_to_name(requirement.as_str()),
            requirement,
            switch_variant_type: variant_type,
        }
    }
    /// Converts the requirements to the name.
    pub fn requirement_to_name(requirement: &str) -> String {
        if let Ok(v) = bool::from_str(requirement) {
            format!("Switch{}", requirement)
        } else if i64::from_str(requirement).is_ok() {
            format!("Switch{}", requirement)
        } else if requirement.contains(":") {
            requirement.replace(":", "_")
        } else {
            requirement.to_string()
        }
            .to_case(Case::UpperCamel)
    }
    pub fn generate_variant_def(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::UpperCamel);
        let requirement = &self.requirement;
        let switch_variant_type = self.switch_variant_type.generate_variant_type();
        quote! {
            #(format!("/// This switch variant requires a value {requirement} in the compare_to field"))
            #<line>
            #name #switch_variant_type,
        }
    }
    pub fn generate_read_call(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::Snake);
        let requirement = &self.requirement;
        let read_def = self.switch_variant_type.generate_read_call(name.as_str());
        quote! {
            #requirement =>{
                #read_def
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SwitchVariantType {
    Void,
    Container(Vec<Field>),
    Single(Field),
}

impl SwitchVariantType {
    pub fn generate_variant_type(&self) -> Tokens<Rust> {
        match self {
            SwitchVariantType::Void => quote! {},
            SwitchVariantType::Container(fields) => {
                let fields: Vec<Tokens<Rust>> = fields
                    .iter()
                    .map(|field| field.generate_field_definition())
                    .collect();
                quote! {
                     {
                         #<line>
                         #(for my_field in fields => #my_field #<line>)
                     }
                }
            }
            SwitchVariantType::Single(data_type) => {
                let data_type = &data_type.data_type;
                quote! {

                    (#data_type)
                }
            }
        }
    }
    pub fn generate_read_call(&self, name: &str) -> Tokens<Rust> {
        match self {
            SwitchVariantType::Void => quote! {
                Ok(Self::#name)
            },
            SwitchVariantType::Container(fields) => {
                let reads: Vec<(Tokens<Rust>, Tokens<Rust>)> =
                    fields.iter().map(|field| field.generate_read()).collect();
                // Split the reads into two vectors, one for the reads and one for the names
                let mut read_calls: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                let mut reads_values: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                for (name, value) in reads {
                    read_calls.push(name);
                    reads_values.push(value);
                }
                quote! {
                     {
                         #(for read_call in read_calls => #read_call #<line>)
                         Ok(Self::#name {
                             #(for read_value in reads_values join (, )  =>  #read_value)
                         })
                     }
                }
            }
            SwitchVariantType::Single(data_type) => {
                let (call, variable) = &data_type.generate_read();
                quote! {
                    {
                        #call
                        #<line>
                        Ok(Self::#name(#variable))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum GenerateType {
    Container {
        content_name: String,
        fields: Vec<Field>,
        children: Vec<GenerateType>,
    },
    SwitchEnum {
        content_name: String,
        compare_to: CompareTo,
        variants: Vec<SwitchVariant>,
        children: Vec<GenerateType>,
    },
    Array {
        content_name: String,
        count_type: DataType,
        data_type: DataType,
        children: Vec<GenerateType>,
    },
    Packet {
        content_name: String,
        packet_id: i32,
        data_type: DataType,
        children: Vec<GenerateType>,
    },
}

impl GenerateType {
    pub fn content_name(&self) -> &str {
        match &self {
            GenerateType::Container { content_name, .. } => content_name,
            GenerateType::SwitchEnum { content_name, .. } => content_name,
            GenerateType::Array { content_name, .. } => content_name,
            GenerateType::Packet { content_name, .. } => content_name,
        }
    }
    pub fn generate_type_wrap_as_mod(&self) -> Tokens<Rust> {
        let mod_name = &self.content_name().to_case(Case::Snake);
        let tokens = self.generate_type();
        quote! {
            mod #mod_name {
                use super::*;
                use crate::common::protocol::PacketContent;
                use crate::common::protocol::PacketSwitch;
                use crate::common::protocol::Packet;
                use std::io::{BufRead, Error, ErrorKind, Result, Write};
                use std::str::FromStr;
                #<line>
                #tokens

            }
            #<line>
            pub use #mod_name::*;
        }
    }
    pub fn generate_type(&self) -> Tokens<Rust> {
        match self {
            GenerateType::Container {
                content_name,
                fields,
                children,
            } => {
                let content_name = &content_name.to_case(Case::UpperCamel);
                let fields_defs: Vec<Tokens<Rust>> = fields
                    .iter()
                    .map(|field| field.generate_field_definition())
                    .collect();
                let writes: Vec<Tokens<Rust>> =
                    fields.iter().map(|field| field.generate_write()).collect();
                let reads: Vec<(Tokens<Rust>, Tokens<Rust>)> =
                    fields.iter().map(|field| field.generate_read()).collect();
                // Split the reads into two vectors, one for the reads and one for the names
                let mut read_calls: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                let mut reads_values: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                for (name, value) in reads {
                    read_calls.push(name);
                    reads_values.push(value);
                }
                let children: Vec<Tokens<Rust>> =
                    children.iter().map(|child| child.generate_type()).collect();
                quote! {
                    pub struct #content_name {
                        #(for my_field in fields_defs => #my_field #<line>)
                    }

                    impl PacketContent for #content_name {
                        fn write<Writer: Write>(self,writer: &mut Writer) -> Result<usize> {
                            let mut total_bytes = 0;
                            #(for my_write in writes => #my_write; #<line>)
                            Ok(total_bytes)
                        }
                        fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self>  {
                            #(for my_read in read_calls =>  #my_read; #<line>)
                            Ok(Self{
                                #(for my_read in reads_values join (, )  =>  #my_read)
                            })
                        }
                    }
                    #(for my_child in children => #my_child #<line>)
                }
            }
            GenerateType::SwitchEnum {
                children,
                content_name,
                compare_to,
                variants,
            } => {
                let content_name = &content_name.to_case(Case::UpperCamel);
                let variants_defs: Vec<Tokens<Rust>> = variants
                    .iter()
                    .map(|variant| variant.generate_variant_def())
                    .collect();
                let variants_reads: Vec<Tokens<Rust>> = variants
                    .iter()
                    .map(|variant| variant.generate_read_call())
                    .collect();
                let children: Vec<Tokens<Rust>> =
                    children.iter().map(|child| child.generate_type()).collect();
                match compare_to {
                    CompareTo::Specified {
                        compare_to_type,
                        compare_to,
                    } => {
                        let compare_to_type = compare_to_type.as_ref();
                        let match_call = if compare_to_type
                            .minecraft_name
                            .eq_ignore_ascii_case("string")
                        {
                            format!("compare_to.as_str()")
                        } else {
                            format!("compare_to")
                        };
                        quote! {

                            pub enum #content_name {
                                #(for my_variant in variants_defs => #my_variant #<line>)
                            }
                            impl PacketSwitch for #content_name {
                                type CompareType = #compare_to_type;
                                fn switch_read<Reader: BufRead>(compare_to: &Self::CompareType, reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
                                    todo!()
                                }
                                fn switch_write<Writer: Write>(self, write_compare: bool, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
                                    todo!()
                                }
                            }
                            #(for my_child in children => #my_child #<line>)

                        }
                    }
                    CompareTo::Generic { compare_to_type } => {
                        let compare_to_type = compare_to_type.as_ref();

                        let match_call = if compare_to_type
                            .minecraft_name
                            .eq_ignore_ascii_case("string")
                        {
                            format!("compare_to.as_str()")
                        } else {
                            format!("compare_to")
                        };
                        quote! {
                            pub enum #content_name {
                                #(for my_variant in variants_defs => #my_variant #<line>)
                            }
                            impl PacketSwitch for #content_name {
                                type CompareType = #compare_to_type;

                                fn switch_read<Reader: BufRead>(compare_to: &Self::CompareType, reader: &mut Reader) -> std::io::Result<Self> where Self: Sized {
                                    todo!()
                                }
                                fn switch_write<Writer: Write>(self, write_compare: bool, writer: &mut Writer) -> std::io::Result<usize> where Self: Sized {
                                    todo!()
                                }
                            }
                            #(for my_child in children => #my_child #<line>)

                        }
                    }
                }
            }

            GenerateType::Array {
                count_type,
                data_type,
                children,
                content_name,
            } => {
                let children: Vec<Tokens<Rust>> =
                    children.iter().map(|child| child.generate_type()).collect();
                quote! {
                    pub type #content_name = Vec<#data_type>;

                    #(for my_child in children => #my_child #<line>)
                }
            }
            GenerateType::Packet {
                packet_id,
                content_name,
                data_type, children
            } => {
                let packet_id = packet_id.clone();
                let children: Vec<Tokens<Rust>> =
                    children.iter().map(|child| child.generate_type()).collect();
                quote! {
                pub struct #content_name;

                impl Packet for #content_name {
                    type PacketIDType = i32;
                    type PacketContent= #data_type;

                    fn packet_id() -> Self::PacketIDType
                        where
                            Self: Sized
                        {
                            #packet_id
                        }
                }
                #(for my_child in children => #my_child #<line>)

                }
            }
        }
    }
}
