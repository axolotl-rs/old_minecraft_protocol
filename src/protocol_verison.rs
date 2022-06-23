pub enum ProtocolVersion {
    Java(JavaProtocolVersion),
    Bedrock(),
}
#[allow(non_snake_case)]
pub enum JavaProtocolVersion {
    v1_19_1,
}
