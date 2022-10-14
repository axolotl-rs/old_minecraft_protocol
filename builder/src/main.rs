use crate::data::git::GitFiles;
use crate::data::MinecraftData;
use crate::error::GenResult;
use crate::version_generator::{Version, VersionGenerator};
use std::path::PathBuf;

pub mod code_gen;
pub mod configs;
pub mod data;
pub mod error;
pub mod version_generator;

use clap::Parser;
use log::LevelFilter;
use rust_embed::RustEmbed;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/configs"]
struct Configs;

#[derive(Parser)]
#[clap(author, about, long_about = None)]
pub struct MinecraftDataBuilder {
    #[clap(
        long,
        default_value = "https://github.com/PrismarineJS/minecraft-data.git"
    )]
    pub git_repo: String,
    #[clap(long)]
    pub output_directory: String,
    #[clap(long, default_value = "minecraft-data")]
    pub git_directory: String,
    #[clap(long, value_enum)]
    pub version: Version,
}

fn main() -> GenResult<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
    let args: MinecraftDataBuilder = MinecraftDataBuilder::parse();
    let git_files = GitFiles::clone_repo(&args.git_repo, &args.git_directory)?;

    let minecraft_data = MinecraftData::new(git_files);
    let mut version_generator =
        VersionGenerator::new(minecraft_data, PathBuf::from(args.output_directory));

    version_generator.generate(args.version, true).unwrap();

    Ok(())
}
