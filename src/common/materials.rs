pub trait Material {
    type Name;
    type ToolSpeedMultipliers;

    fn name(&self) -> Self::Name;

    fn tool_speed_multipliers(&self) -> Self::ToolSpeedMultipliers;
}
