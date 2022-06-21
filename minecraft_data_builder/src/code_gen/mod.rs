use convert_case::{Case, Casing};
use serde::Serialize;
use crate::version_generator::protocol::switch_compare::CompareTo;
use genco::fmt;
use genco::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub switch: Option<String>,

}

impl Field {
    pub fn generate_field_definition(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::Snake);
        let data_type = &self.data_type;
        let mut value = quote! {
            #name: #data_type,
        };
        value
    }
    pub fn generate_write(&self) -> Tokens<Rust> {
        let name = &self.name;
        let data_type = &self.data_type;
        if let Some(switch) = &self.switch {
            quote! {
                    total_bytes += self.#name.switch_write(writer)?;
                }
        } else {
            quote! {
               total_bytes += self.#name.write(writer)?;
            }
        }
    }
    pub fn generate_read(&self) -> (Tokens<Rust>, Tokens<Rust>) {
        let name = &self.name;
        let data_type = &self.data_type;
        let v1 = if let Some(switch) = &self.switch {
            quote! {
                let #name: #data_type = PacketSwitch::switch_read(#switch,reader)?;
                }
        } else {
            quote! {
                let #name: #data_type = PacketContent::read(reader)?;
            }
        };
        (v1, quote! {
            #name
        })
    }
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test() {
        let tokens = Field {
            name: "test".to_string(),
            data_type: "i32".to_string(),
            switch: None,
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
        let tokens = Field {
            name: "test".to_string(),
            data_type: "i32".to_string(),
            switch: None,
        };
        let tokens2 = Field {
            name: "test2".to_string(),
            data_type: "i32".to_string(),
            switch: None,
        };
        let tokens3 = Field {
            name: "test3".to_string(),
            data_type: "i32".to_string(),
            switch: None,
        };
        // Create a container
        let container = GenerateType::Container {
            content_name: "y Variant".to_string(),
            fields: vec![tokens, tokens2, tokens3],
            children: vec![]
        };
        println!("{}", container.generate_type().to_string().unwrap());
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SwitchVariant {
    pub name: String,
    pub requirement: String,
    pub switch_variant_type: SwitchVariantType,
}

impl SwitchVariant {
    pub fn generate_variant_def(&self) -> Tokens<Rust> {
        let name = &self.name.to_case(Case::UpperCamel);
        let requirement = &self.requirement;
        let switch_variant_type = self.switch_variant_type.generate_variant_type();
        quote! {
            #[doc = "This switch variant requires a value #requirement in the compare_to field"]
            #<line>
            #name #switch_variant_type,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum SwitchVariantType {
    Void,
    Container(Vec<Field>),
    Single(Field),
}

impl SwitchVariantType {
    pub fn generate_variant_type(&self) -> Tokens<Rust> {
        match self {
            SwitchVariantType::Void => quote! {

            },
            SwitchVariantType::Container(fields) => {
                let fields: Vec<Tokens<Rust>> = fields.iter().map(|field| field.generate_field_definition()).collect();
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
}

impl GenerateType {
    pub fn generate_type(&self) -> Tokens<Rust> {
        match self {
            GenerateType::Container { content_name, fields, children } => {
                let content_name = &content_name.to_case(Case::UpperCamel);
                let fields_defs: Vec<Tokens<Rust>> = fields.iter().map(|field| field.generate_field_definition()).collect();
                let writes: Vec<Tokens<Rust>> = fields.iter().map(|field| field.generate_write()).collect();
                let reads: Vec<(Tokens<Rust>, Tokens<Rust>)> = fields.iter().map(|field| field.generate_read()).collect();
                // Split the reads into two vectors, one for the reads and one for the names
                let mut read_calls: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                let mut reads_values: Vec<Tokens<Rust>> = Vec::with_capacity(reads.len());
                for (name, value) in reads {
                    read_calls.push(name);
                    reads_values.push(value);
                }
                let children: Vec<Tokens<Rust>> = children.iter().map(|child| child.generate_type()).collect();
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
                            Self{
                                #(for my_read in reads_values join (, )  =>  #my_read)
                            }
                        }
                    }
                    #(for my_child in children => #my_child #<line>)
                }
            }
            GenerateType::SwitchEnum { .. } => {
                quote! {

                }
            }
        }
    }
}
