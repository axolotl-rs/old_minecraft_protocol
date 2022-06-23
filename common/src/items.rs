pub trait Item {
    type Id;
    type DisplayName;
    type Name;
    type StackSize;
    type MaxDurability;
    type RepairWith;
    type EnchantCategories;
    type Variations;

    fn id(&self) -> Self::Id;

    fn display_name(&self) -> Self::DisplayName;

    fn name(&self) -> Self::Name;

    fn stack_size(&self) -> Self::StackSize;

    fn max_durability(&self) -> Self::MaxDurability;

    fn repair_with(&self) -> Self::RepairWith;

    fn enchant_categories(&self) -> Self::EnchantCategories;

    fn variations(&self) -> Self::Variations;
}
