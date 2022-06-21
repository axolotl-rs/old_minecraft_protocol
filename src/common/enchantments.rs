pub trait Enchantment {
    type Id;
    type Name;
    type DisplayName;
    type MaxLevel;
    type MinCost;
    type MaxCost;
    type TreasureOnly;
    type Curse;
    type Exclude;
    type Category;
    type Weight;
    type Tradeable;
    type Discoverable;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;

    fn display_name(&self) -> Self::DisplayName;

    fn max_level(&self) -> Self::MaxLevel;

    fn min_cost(&self) -> Self::MinCost;

    fn max_cost(&self) -> Self::MaxCost;

    fn treasure_only(&self) -> Self::TreasureOnly;

    fn curse(&self) -> Self::Curse;

    fn exclude(&self) -> Self::Exclude;

    fn category(&self) -> Self::Category;

    fn weight(&self) -> Self::Weight;

    fn tradeable(&self) -> Self::Tradeable;

    fn discoverable(&self) -> Self::Discoverable;
}