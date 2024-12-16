use anyhow::Result;
use axum::{
    body::Body, extract::State, http::{Request, StatusCode, Uri}, response::{Html, Response}, Router
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub mod auth;
pub mod collection;
pub mod project;
pub use project::Project;

const DEFAULT_PORT: u16 = 3195;

pub struct BitSink {
    projects: Arc<RwLock<Vec<Project>>>,
    port: u16,
    server_task: Option<JoinHandle<Result<()>>>,
}

impl BitSink {
    pub async fn projects(&self) -> Vec<Project> {
        self.projects.read().await.clone()
    }

    pub async fn start(&mut self) -> Result<()> {
        let app = self.router();
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        println!("Server running at http://{}", addr);

        axum::serve(listener, app).await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(server_task) = self.server_task.take() {
            server_task.abort();
            // Handle abort error explicitly
            match server_task.await {
                Ok(result) => result,
                Err(e) if e.is_cancelled() => Ok(()),
                Err(e) => Err(e.into()),
            }?;
        }
        Ok(())
    }

    pub fn router(&self) -> Router {
        let projects = self.projects.clone();

        Router::new()
            .route("/", axum::routing::any(|| async { "Welcome to BitSink" }))
            .route("/ping", axum::routing::any(|| async { "pong" }))
            .route("/health", axum::routing::any(|| async { "OK" }))
            .route("/projects", axum::routing::get(Self::projects_handler))
            .fallback(Self::dynamic_route_handler)
            .with_state(projects)
    }

    async fn projects_handler(State(projects): State<Arc<RwLock<Vec<Project>>>>) -> Html<String> {
        let projects = projects.read().await;
        let project_list = projects
            .iter()
            .map(|p| format!("<li><a href=\"/{}\">{}</a></li>", p.short_id(), p.name()))
            .collect::<Vec<_>>()
            .join("\n");

        Html(format!(
            r#"<!DOCTYPE html>
        <html>
            <head><title>Projects</title></head>
            <body>
                <h1>Projects</h1>
                <ul>{}</ul>
            </body>
        </html>"#,
            project_list
        ))
    }

    async fn dynamic_route_handler(
        State(projects): State<Arc<RwLock<Vec<Project>>>>,
        uri: Uri,
        req: Request<Body>,
    ) -> Response<Body> {
        let path = uri.path();
        let projects = projects.read().await;

        // Extract project ID from path
        let project_id = path.split('/').nth(1).unwrap_or("");

        if let Some(project) = projects.iter().find(|p| p.short_id() == project_id) {
            // Forward request to project router
            let remaining_path = path
                .strip_prefix(&format!("/{}", project_id))
                .unwrap_or("/");

            let mut new_req = Request::builder()
                .uri(remaining_path)
                .method(req.method().clone());

            // Copy headers
            for (key, value) in req.headers() {
                new_req = new_req.header(key, value);
            }

            let new_req = new_req.body(req.into_body()).unwrap();
            project.handle_request(new_req).await
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Project not found"))
                .unwrap()
        }
    }
}

impl BitSink {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(Vec::new())),
            port: DEFAULT_PORT,
            server_task: None,
        }
    }

    pub async fn add_project(&self, project: Project) -> Result<()> {
        let mut projects = self.projects.write().await;
        projects.push(project);
        Ok(())
    }

    pub async fn remove_project(&self, project_id: &str) -> Result<()> {
        let mut projects = self.projects.write().await;
        projects.retain(|p| p.short_id() != project_id);
        Ok(())
    }
}
