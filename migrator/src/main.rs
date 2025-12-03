use clap::{Parser, Subcommand};
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "migrate")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { service: String },
    Revert { service: String },
}

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let (action, service) = match cli.command {
        Commands::Run { service } => ("run", service),
        Commands::Revert { service } => ("revert", service),
    };

    // Construct env variable name dynamically
    let env_var = format!("{}_DATABASE_URL", service.to_uppercase());
    let database_url =
        env::var(&env_var).unwrap_or_else(|_| panic!("Environment variable {} not set", env_var));

    // Run the sqlx command
    let mut child = Command::new("sqlx")
        .args([
            "migrate",
            action,
            "--source",
            &format!("services/{}/migrations", service),
            "--database-url",
            &database_url,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    // Capture stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("{}", line?);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("Migration failed for service {}", service);
    }

    Ok(())
}
