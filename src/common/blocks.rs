pub trait Block {
    type Id;
    type DisplayName;
    type Name;
    type Hardness;
    type Resistance;
    type MinStateId;
    type MaxStateId;
    type States;
    type Drops;
    type Diggable;
    type Transparent;
    type FilterLight;
    type EmitLight;
    type BoundingBox;
    type StackSize;
    type Material;
    type HarvestTools;
    type DefaultState;
    type Variations;

    fn id(&self) -> Self::Id;

    fn display_name(&self) -> Self::DisplayName;

    fn name(&self) -> Self::Name;

    fn hardness(&self) -> Self::Hardness;

    fn resistance(&self) -> Self::Resistance;

    fn min_state_id(&self) -> Self::MinStateId;

    fn max_state_id(&self) -> Self::MaxStateId;

    fn states(&self) -> Self::States;

    fn drops(&self) -> Self::Drops;

    fn diggable(&self) -> Self::Diggable;

    fn transparent(&self) -> Self::Transparent;

    fn filter_light(&self) -> Self::FilterLight;

    fn emit_light(&self) -> Self::EmitLight;

    fn bounding_box(&self) -> Self::BoundingBox;

    fn stack_size(&self) -> Self::StackSize;

    fn material(&self) -> Self::Material;

    fn harvest_tools(&self) -> Self::HarvestTools;

    fn default_state(&self) -> Self::DefaultState;

    fn variations(&self) -> Self::Variations;
}
