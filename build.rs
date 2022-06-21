fn main() {
    let option = std::env::var_os("CARGO_FEATURE_GENERIC_DATA");
    if option.is_some() {
        println!("Generating mod.rs for src/generated/");
        //TODO generate the mod.rs
    }

}