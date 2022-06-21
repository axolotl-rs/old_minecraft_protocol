pub trait Attribute {
    type Name;
    type Resource;

    fn name(&self) -> Self::Name;

    fn resource(&self) -> Self::Resource;
}