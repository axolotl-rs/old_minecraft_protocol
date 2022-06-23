pub trait TintData {
    type Keys;
    type Color;

    fn keys(&self) -> Self::Keys;

    fn color(&self) -> Self::Color;
}

pub trait Tint {
    type Name;
    type Data;

    fn name(&self) -> Self::Name;

    fn data(&self) -> Self::Data;
}
