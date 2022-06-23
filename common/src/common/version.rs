pub trait Version {
    type MinecraftVersion;
    type Version;
    type MajorVersion;

    fn minecraft_version(&self) -> Self::MinecraftVersion;

    fn version(&self) -> Self::Version;

    fn major_version(&self) -> Self::MajorVersion;
}
