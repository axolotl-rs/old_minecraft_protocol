pub trait Window {
    type Id;
    type Name;
    type Slots;
    type OpenedWith;
    type Properties;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;

    fn slots(&self) -> Self::Slots;

    fn opened_with(&self) -> Self::OpenedWith;

    fn properties(&self) -> Self::Properties;
}

pub trait Slot {
    type Name;
    type Index;
    type Size;

    fn name(&self) -> Self::Name;

    fn index(&self) -> Self::Index;

    fn size(&self) -> Self::Size;
}

pub trait OpenedWith {
    type Type;
    type Id;

    fn ty(&self) -> Self::Type;

    fn id(&self) -> Self::Id;
}
