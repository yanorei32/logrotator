use clap::Parser;
use validator::{Validate, ValidationError};
use std::num::NonZeroU64;
use std::path::PathBuf;

#[derive(Debug, Validate, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short)]
    #[validate(custom = "validate_dir")]
    pub dir: PathBuf,

    #[arg(long, short)]
    pub interval: NonZeroU64,

    #[arg(long)]
    pub gzip: bool,
}

fn validate_dir(dir: &PathBuf) -> Result<(), ValidationError> {
    if dir.is_dir() {
        Ok(())
    } else {
        Err(ValidationError::new("Is not a directory"))
    }
}
