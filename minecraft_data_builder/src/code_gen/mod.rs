
use serde::Serialize;
use crate::version_generator::protocol::switch_compare::CompareTo;

#[derive(Debug, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub switch: Option<String>,

}


#[derive(Debug, Serialize, Clone)]
pub struct SwitchVariant {
    pub name: String,
    pub requirement: String,
    pub switch_variant_type: SwitchVariantType,
}


#[derive(Debug, Serialize, Clone)]
pub enum SwitchVariantType{
    Void,
    Container(Vec<Field>),
    Single(String)
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
