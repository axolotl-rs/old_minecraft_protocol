pub trait Biome {
    type Id;
    type Name;
    type NameLegacy;
    type Category;
    type TemperatureLevel;
    type PrecipitationType;
    type DimensionType;
    type DisplayName;
    type Color;
    // TODO: supported versions: 1.8
    type Depth;
    type RainfallLevel;
    type Climates;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;

    fn name_legacy(&self) -> Self::NameLegacy;

    fn category(&self) -> Self::Category;

    fn temperature_level(&self) -> Self::TemperatureLevel;

    fn precipitation_type(&self) -> Self::PrecipitationType;

    fn dimension_type(&self) -> Self::DimensionType;

    fn display_name(&self) -> Self::DisplayName;

    fn color(&self) -> Self::Color;

    // TODO: supported versions: 1.8
    fn depth(&self) -> Self::Depth;

    fn rainfall_level(&self) -> Self::RainfallLevel;

    fn climates(&self) -> Self::Climates;
}

pub trait Climate {
    type Temperature;
    type Humidity;
    type Altitude;
    type Weirdness;
    type Offset;

    fn temperature(&self) -> Self::Temperature;

    fn humidity(&self) -> Self::Humidity;

    fn altitude(&self) -> Self::Altitude;

    fn weirdness(&self) -> Self::Weirdness;

    fn offset(&self) -> Self::Offset;
}