use std::sync::Arc;

use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, Response},
    Json, Router,
};
use project::Project;
use tokio::task::{JoinHandle, JoinSet};

pub mod auth;
pub mod cli;
pub mod collection;
pub mod project;

const DEFAULT_PORT: u16 = 3195;

pub struct BitSink {
    projects: Vec<Project>,
    port: u16,
    server_task: Option<JoinHandle<Result<()>>>,
}

impl BitSink {
    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub async fn start(&mut self) -> Result<()> {
        let port = self.port.clone();
        let app = self.router();
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        println!("Server running at http://{}", addr);
        // self.server_task = Some(tokio::spawn(async move {
        let _r = axum::serve(listener, app).await;
        // Ok(())
        // }));
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(server_task) = self.server_task.take() {
            server_task.abort();
            server_task.await??;
        }
        Ok(())
    }

    pub fn router(&self) -> Router {
        let mut router = self.static_router();
        let projects = self.projects.clone();
        for project in projects {
            let project_path = format!("/{}", project.short_id());
            let project_router = project.router();
            router = router.nest(&project_path, project_router);
        }
        router
    }
}

impl BitSink {
    fn static_router(&self) -> Router {
        let static_router = Router::new()
            .route("/", axum::routing::any(|| async {
                Html(
                    r#"
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <title>Bitsink</title>
                        </head>
                        <body>
                            <h1>Welcome to Bitsink</h1>
                            <p>Bitsink is a simple HTTP server for managing projects.</p>
                        </body>
                    </html>
                    "#,
                )
             }))
            .route("/ping", axum::routing::any(|| async { "pong" }))
            .route("/health", axum::routing::any(|| async { "OK" }))
            .route(
                "/projects",
                axum::routing::get({
                    let projects = self
                        .projects
                        .iter()
                        .map(|p| p.short_id().to_string())
                        .collect::<Vec<_>>();
                    move || async move { Html(
                        format!(
                            r#"
                            <!DOCTYPE html>
                            <html>
                                <head>
                                    <title>Projects</title>
                                </head>
                                <body>
                                    <h1>Projects</h1>
                                    <ul>
                                        {}
                                    </ul>
                                </body>
                            </html>
                            "#,
                            projects
                                .iter()
                                .map(|p| format!("<li><a href=\"/{}\">{}</a></li>", p, p))
                                .collect::<Vec<_>>()
                                .join("\n")
                        )
                    ) }
                }),
            );
        static_router
    }
}

impl BitSink {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            port: DEFAULT_PORT,
            server_task: None,
        }
    }
}
