use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(
    name = "bitsink",
    about = "Bitsink is a lightweight, single-file CLI tool designed to store and manage data efficiently.",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the configuration file (default: config.toml)
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    /// Enable verbose logging
    #[arg(short, long, help = "Enable verbose logging.")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the Bitsink server
    #[command(alias = "s")]
    Server,

    /// Manage projects
    #[command(subcommand)]
    Project(ProjectCommand),

    /// Enter interactive mode
    #[command(alias = "i")]
    Interactive,
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Create a new project
    #[command(alias = "n")]
    New { name: String },

    /// List all projects
    #[command(alias = "l")]
    List,

    /// Delete a project
    #[command(alias = "d")]
    Delete { name: String },

    /// Rename a project
    Rename { old_name: String, new_name: String },

    /// Export a project to a file
    Export { name: String, file: String },

    /// Show detailed information about a project
    Info { name: String },
}

impl Cli {
    pub fn run(&self) {
        if self.verbose {
            println!("Verbose mode enabled.");
        }

        match &self.command {
            Commands::Server => {
                println!("Starting server...");
                // Implement server start logic
            }
            Commands::Project(action) => match action {
                ProjectCommand::New { name } => {
                    println!("Creating new project: {}", name);
                    // Implement project creation logic
                }
                ProjectCommand::List => {
                    println!("Listing all projects...");
                    // Implement listing logic
                }
                ProjectCommand::Delete { name } => {
                    println!("Deleting project: {}", name);
                    // Implement deletion logic
                }
                ProjectCommand::Rename { old_name, new_name } => {
                    println!("Renaming project from '{}' to '{}'", old_name, new_name);
                    // Implement renaming logic
                }
                ProjectCommand::Export { name, file } => {
                    println!("Exporting project '{}' to '{}'", name, file);
                    // Implement export logic
                }
                ProjectCommand::Info { name } => {
                    println!("Showing information for project: {}", name);
                    // Implement info display logic
                }
            },
            Commands::Interactive => {
                println!("Entering interactive mode...");
                // Implement interactive mode
            }
        }
    }
}