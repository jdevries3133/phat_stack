use clap::{Parser, ValueEnum};
use flate2::read::GzDecoder;
use std::path::PathBuf;
use tar::Archive;

mod err;
mod templates;

use err::*;

#[derive(ValueEnum, Clone, Debug)]
enum FeatureFlags {
    Containerization,
    GitHubWorkflows,
    TransactionalEmail,
    StripeIntegration,
}

#[derive(Parser, Debug)]
#[command(name = "create-phat-app", version)]
#[command(about = "Scaffold a PHAT app.")]
struct Args {
    /// The name of your new app.
    #[arg(short, long)]
    name: String,
    /// Where to create your app.
    #[arg(short, long, default_value = ".")]
    location: PathBuf,
    #[arg(short, long)]
    enable_features: Vec<FeatureFlags>,
    #[arg(short, long)]
    disable_features: Vec<FeatureFlags>,
    #[arg(short, long)]
    base_template: templates::BaseTemplate,
}

fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let template_tarball = args.base_template.tarball().map_err(|e| {
        e.push(ErrT::NotImplemented).ctx(format!(
            r#"Base template "{:?}" not supported yet."#,
            args.base_template
        ))
    })?;
    let tar = GzDecoder::new(template_tarball);
    let mut archive = Archive::new(tar);
    archive.unpack(&args.location)
        .map_err(|e| {
            ErrStack::new(ErrT::TarUnarchive).ctx(format!(
                "Failed to unpack template tarball into desired location. Desired location was {:?}. Error was: {}",
                &args.location,
                e
            ))
        })?;

    Ok(())
}

fn main() {
    if let Err(e) = cli() {
        print!("{e}");
        std::process::exit(1);
    }
}
