#[cfg(feature = "cli")]
use mediaflow::cli;

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("No CLI, because `cli` feature is not installed");
    std::process::exit(1);
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), cli::CliError> {
    cli::run().await?;
    Ok(())
}
