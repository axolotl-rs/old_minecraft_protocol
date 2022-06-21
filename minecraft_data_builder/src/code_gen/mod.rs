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
        let data_type = &self.data_type.to_case(Case::UpperCamel);
        let mut value = quote! {
            pub #name: #data_type,
        };
        value
    }
    pub fn generate_read(&self) -> Tokens<Rust> {
        let name = &self.name;
        let data_type = &self.data_type;
        if let Some(switch) = &self.switch {
            quote! {
                    let {{case field.name "snake"}}: {{field.type}}= PacketSwitch::switch_read({{field.switch}},::reader)?;
                }
        } else {
             quote! {
                #name = buf.read_#data_type()?;
            }
        }
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
        }.generate_field_definition();
        println!("{}", tokens.to_string().unwrap());
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SwitchVariant {
    pub name: String,
    pub requirement: String,
    pub switch_variant_type: SwitchVariantType,
}


#[derive(Debug, Serialize, Clone)]
pub enum SwitchVariantType {
    Void,
    Container(Vec<Field>),
    Single(String),
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
