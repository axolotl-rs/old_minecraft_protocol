pub trait Recipe {
    type Id;
    type Options;

    fn id(&self) -> Self::Id;

    fn options(&self) -> Self::Options;
}
pub trait RecipeOption {
    type Ingredients;
    type Result;

    fn ingredients(&self) -> Self::Ingredients;

    fn result(&self) -> Self::Result;
}

pub trait Result {
    type Count;
    type Id;
    type Metadata;

    fn count(&self) -> Self::Count;

    fn id(&self) -> Self::Id;

    fn metadata(&self) -> Self::Metadata;
}