use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod templates;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = Args::parse();

    templates::puke_template();

    Ok(())
}
