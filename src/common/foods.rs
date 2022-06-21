pub trait Food {
    type Id;
    type Name;
    type StackSize;
    type DisplayName;
    type FoodPoints;
    type Saturation;
    type EffectiveQuality;
    type SaturationRatio;
    type Variations;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;

    fn stack_size(&self) -> Self::StackSize;

    fn display_name(&self) -> Self::DisplayName;

    fn food_points(&self) -> Self::FoodPoints;

    fn saturation(&self) -> Self::Saturation;

    fn effective_quality(&self) -> Self::EffectiveQuality;

    fn saturation_ratio(&self) -> Self::SaturationRatio;

    fn variations(&self) -> Self::Variations;
}