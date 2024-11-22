mod bitsink;

use anyhow::Result;
use axum::extract::Path;
use bitsink::cli::Cli;
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    tracing_subscriber::fmt::init();
    // Parse the CLI arguments and execute the corresponding commands
    // let cli = Cli::parse();
    // cli.run();

    let mut sink = bitsink::BitSink::new();
    let mut project_1 =
        bitsink::project::Project::new("Project 1".to_string(), "Description 1".to_string())?;
    // project_1.allow_defaults();
    project_1.add_route(
        "/test/:name",
        axum::routing::get(|Path(name): Path<String>| async move {
            format!("Project 1 test: {}", name)
        }),
    );
    project_1.allow_origin(Url::parse("http://google.com")?);
    
    sink.add_project(project_1);

    let mut project_2 =
        bitsink::project::Project::new("Project 2".to_string(), "Description 2".to_string())?;
    project_2.allow_defaults();
    project_2.add_route(
        "/p2_test",
        axum::routing::get(|| async { "Project 2 test" }),
    );
    sink.add_project(project_2);

    sink.start().await?;
    Ok(())
}
