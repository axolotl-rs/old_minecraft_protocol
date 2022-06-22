use std::collections::HashMap;
use minecraft_data_rs::models::protocol::types::{SwitchType, TypeName};
use crate::code_gen::CompareTo;
use crate::error::GenError;
use crate::version_generator::protocol::types::{GenerationResult, TypesGenerator};

pub fn generate_top_level_switch(name:  TypeName, compare_to: String,default: Option<String>, fields: HashMap<String, SwitchType>, state: &TypesGenerator) ->Result<GenerationResult, GenError>{

}