pub trait Entity {
    type Id;
    type InternalId;
    type Name;
    type DisplayName;
    type Width;
    type Height;
    type Type;
    type Category;

    fn id(&self) -> Self::Id;
    fn internal_id(&self) -> Self::InternalId;
    fn name(&self) -> Self::Name;
    fn display_name(&self) -> Self::DisplayName;
    fn width(&self) -> Self::Width;
    fn height(&self) -> Self::Height;
    fn ty(&self) -> Self::Type;
    fn category(&self) -> Self::Category;
}
