use clap::{Parser, ValueEnum};
use flate2::read::GzDecoder;
use std::{fs::read_dir, path::PathBuf};
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
    let location_utf8 =
        args.location.to_str().unwrap_or("(location is not utf-8)");

    let template_tarball = args.base_template.tarball().map_err(|e| {
        e.push(ErrT::NotImplemented).ctx(format!(
            r#"Base template "{:?}" not supported yet."#,
            args.base_template
        ))
    })?;

    if !args.location.is_dir() {
        return Err(ErrStack::new(ErrT::ValidationError)
            .ctx(format!("Location {location_utf8} is not a directory"))
            .into());
    };

    let num_items = read_dir(&args.location)
        .map_err(|e| {
            ErrStack::io(e)
                .ctx(format!("while checking if {location_utf8} is empty"))
        })?
        .try_fold(Vec::new(), |mut acc, item| match item {
            Ok(item) => {
                acc.push(item.file_name());
                Ok(acc)
            }
            Err(e) => Err(ErrStack::io(e).ctx(format!(
                "on one item while checking if {location_utf8} is empty"
            ))),
        })?;
    if !num_items.is_empty() {
        return Err(ErrStack::new(ErrT::ValidationError)
            .ctx(format!(
                "{} is not an empty directory. Contains items: {}",
                location_utf8,
                num_items
                    .iter()
                    .map(|item| item.to_str().unwrap_or("(not utf-8)"))
                    .fold(String::new(), |mut acc, item| {
                        if !acc.is_empty() {
                            acc.push(',');
                            acc.push(' ');
                        }
                        acc.push_str(item);
                        acc
                    })
            ))
            .into());
    }

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
