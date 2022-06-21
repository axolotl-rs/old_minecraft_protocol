pub trait Particle {
    type Id;
    type Name;

    fn id(&self) -> Self::Id;

    fn name(&self) -> Self::Name;
}