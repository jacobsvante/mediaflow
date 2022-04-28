#[cfg(feature = "cli")]
use mediaflow::cli;

#[cfg(not(feature = "cli"))]
fn main() {
    println!("No CLI, because `cli` feature is not installed")
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), cli::CliError> {
    cli::run().await?;
    Ok(())
}
