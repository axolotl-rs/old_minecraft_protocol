// TODO: unsupported versions: 1.8
pub trait Effect {
    type Id;
    type Name;
    type DisplayName;
    type Type;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;

    fn display_name(&self) -> Self::DisplayName;

    fn ty(&self) -> Self::Type;
}
