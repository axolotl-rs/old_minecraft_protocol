// TODO: unsupported versions: 1.8
pub trait BlockCollisionShapes {
    type Blocks;
    type Shapes;

    fn blocks(&self) -> Self::Blocks;

    fn shapes(&self) -> Self::Shapes;
}
