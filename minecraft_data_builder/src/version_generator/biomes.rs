use std::fs::write;
use std::path::PathBuf;
use convert_case::{Case, Casing};
use crate::error::GenError;
use crate::{GenResult, Version};
use serde::Deserialize;
use serde::Serialize;


pub type Root = Vec<Biome>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biome {
    pub id: u32,
    pub name: String,
    pub name_legacy: Option<String>,
    pub category: String,
    pub temperature: f64,
    pub precipitation: String,
    pub dimension: String,
    pub display_name: String,
    pub color: u32,
    pub rainfall: f64,
    pub depth: Option<f64>,
    pub climates: Option<Vec<Climate>>
}

pub struct Root2 {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub temperature: f64,
    pub precipitation: String,
    pub depth: f64,
    pub dimension: String,
    pub display_name: String,
    pub color: i64,
    pub rainfall: f64,
    pub climates: Option<Vec<Climate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Climate {
    pub temperature: u64,
    pub humidity: u64,
    pub altitude: u64,
    pub weirdness: u64,
    pub offset: u64,
}

pub fn generate_biomes(version: Version, file: PathBuf, json: Root) -> GenResult<()> {
    let mut string = String::new();

    string += "pub enum Biome {\n";
    for biome in &json {
        let name = biome.name.as_str().to_case(Case::Pascal);

        string += "    ";
        string += name.as_str();
        string += ",\n";
    }
    string += "}\n\n";

    if version.is_1_8() {
        string += "pub struct Climate {\n";
        string += "    pub temperature: u64,\n";
        string += "    pub humidity: u64,\n";
        string += "    pub altitude: u64,\n";
        string += "    pub weirdness: u64,\n";
        string += "    pub offset: u64,\n";
        string += "}\n\n";
        string += "impl crate::common::biomes::Climate for Climate {\n";
        string += "    type Temperature = u64;\n";
        string += "    type Humidity = u64;\n";
        string += "    type Altitude = u64;\n";
        string += "    type Weirdness = u64;\n";
        string += "    type Offset = u64;\n\n";
        string += "    fn temperature(&self) -> Self::Temperature { self.temperature }\n";
        string += "    fn humidity(&self) -> Self::Humidity { self.humidity }\n";
        string += "    fn altitude(&self) -> Self::Altitude { self.altitude }\n";
        string += "    fn weirdness(&self) -> Self::Weirdness { self.weirdness }\n";
        string += "    fn offset(&self) -> Self::Offset { self.offset }\n";
        string += "}\n\n";
    }

    string += "impl crate::common::biomes::Biome for Biome {\n";
    string += "    type Id = u64;\n";
    string += "    type Name = String;\n";
    string += "    type Category = String;\n";
    string += "    type TemperatureLevel = f64;\n";
    string += "    type PrecipitationType = String;\n";
    string += "    type DimensionType = String;\n";
    string += "    type DisplayName = String;\n";
    string += "    type Color = u32;\n";
    string += "    type RainfallLevel = f64;\n";
    if version.is_1_8() {
        string += "    type NameLegacy = String;\n";
        string += "    type Depth = f64;\n";
        string += "    type Climates = Vec<Climate>;\n\n";
    } else {
        string += "    type NameLegacy = ();\n";
        string += "    type Depth = ();\n";
        string += "    type Climates = ();\n\n";
    }

    generate_property(&mut string, version, None, "id", &json, |biome| biome.id.to_string());
    generate_property(&mut string, version, None, "name", &json, |biome| format!("\"{}\".to_string()", biome.name));
    generate_property(&mut string, version, None, "category", &json, |biome| format!("\"{}\".to_string()", biome.category));
    generate_property(&mut string, version, None, "temperature_level", &json, |biome| biome.temperature.to_string());
    generate_property(&mut string, version, None, "precipitation_type", &json, |biome| format!("\"{}\".to_string()", biome.precipitation));
    generate_property(&mut string, version, None, "dimension_type", &json, |biome| format!("\"{}\".to_string()", biome.dimension));
    generate_property(&mut string, version, None, "color", &json, |biome| biome.color.to_string());
    generate_property(&mut string, version, None, "rainfall_level", &json, |biome| biome.rainfall.to_string());
    generate_property(&mut string, version, Some(Version::One8), "name_legacy", &json, |biome| format!("\"{}\".to_string()", biome.name_legacy.as_ref().unwrap_or(&String::new())));
    generate_property(&mut string, version, Some(Version::One8), "depth", &json, |biome| biome.depth.unwrap_or(0.0).to_string());
    generate_property(&mut string, version, Some(Version::One8), "climates", &json, |biome| {
        let mut string = String::from("vec![");

        if let Some(climates) = &biome.climates {
            string += climates.iter()
                .map(|climate| format!("Climate {{ temperature: {}, humidity: {}, altitude: {}, weirdness: {}, offset: {} }}",
                                       climate.temperature, climate.humidity, climate.altitude, climate.weirdness, climate.offset))
                .collect::<Vec<_>>()
                .join(",\n            ")
                .as_str();
        }

        string += "]";

        string
    });

    string += "}";

    write(file, string).map_err(GenError::Io)
}

pub fn generate_property<T: Fn(&Biome) -> String>(
    string: &mut String,
    current_version: Version,
    required_version: Option<Version>,
    name: &str,
    json: &Root,
    prop: T
) {
    let name_pascal = name.to_case(Case::Pascal);

    *string += "    fn ";
    *string += name;
    *string += "(&self) -> Self::";
    *string += name_pascal.as_str();
    *string += " {\n";

    if required_version == Some(current_version) || required_version == None {
        *string += "        match self {\n";
        for biome in json {
            let name = biome.name.as_str().to_case(Case::Pascal);
            *string += "            Biome::";
            *string += name.as_str();
            *string += " => ";
            *string += prop(biome).as_str();
            *string += ",\n";
        }
        *string += "        }\n";
    } else {
        *string += "        ()\n";
    }

    *string += "    }\n\n";
}