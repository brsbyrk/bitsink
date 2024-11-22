use anyhow::Result;
use axum::body::Body;
use axum::http::header::{HeaderName, HeaderValue};
use axum::http::method::Method;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::routing::MethodRouter;
use axum::{Json, Router};
use base64::Engine;
use ring::digest;
use tower::util::BoxLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use url::Url;
use uuid::Uuid;

use super::auth;
use super::auth::jwt::JwtAuth;

/// A project in BitSink.
///
/// A project is a collection of routes that can be accessed by clients.
/// Each project has a unique identifier, a name, a description, a list of allowed origins, a list of allowed methods, a list of allowed headers, a router, and a token.
/// The router is a collection of routes that can be accessed by clients.
/// The token is used to authenticate clients.
#[derive(Clone)]
pub struct Project {
    id: Uuid,
    short_id: String,
    name: String,
    description: String,
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Vec<String>,
    router: Router,
    token: String,
}

impl Project {
    /// Creates a new [`Project`].
    pub fn new(name: String, description: String) -> Result<Self> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let short_id = Self::uuid_to_short_id(id);
        let mut p = Self {
            id,
            short_id,
            name,
            description,
            allowed_origins: Vec::new(),
            allowed_methods: Vec::new(),
            allowed_headers: Vec::new(),
            router: Self::static_router(),
            token: JwtAuth::new(Self::generate_secret_key(id)).generate_token(&id_as_string)?,
        };
        println!("token: {}", p.token);
        Ok(p)
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn short_id(&self) -> &str {
        &self.short_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn allow_origin(&mut self, origin: Url) {
        self.allowed_origins.push(origin.as_str().to_string());
    }

    pub fn allow_method(&mut self, method: Method) {
        self.allowed_methods.push(method.as_str().to_string());
    }

    pub fn allow_header(&mut self, header: HeaderName) {
        self.allowed_headers.push(header.as_str().to_string());
    }

    pub fn add_route(&mut self, path: &str, handler: MethodRouter) {
        self.router = self.router.clone().route(path, handler);
    }

    // Returns the [`Router`] with CORS for this [`Project`].
    pub fn router(&self) -> Router {
        let p = self.clone();
        let router =
            self.router
                .clone()
                .layer(self.cors())
                .clone()
                .route_layer(axum::middleware::from_fn(move |req, next| {
                    let project = p.clone();
                    Self::auth_middleware(req, next, project)
                }));
        router
    }

    pub fn allow_all_origins(&mut self) {
        self.allowed_origins.clear();
    }

    pub fn allow_all_methods(&mut self) {
        self.allowed_methods.clear();
        // self.allowed_methods.push("GET".to_string());
        // self.allowed_methods.push("POST".to_string());
        // self.allowed_methods.push("PUT".to_string());
        // self.allowed_methods.push("DELETE".to_string());
    }

    pub fn allow_all_headers(&mut self) {
        self.allowed_headers.clear();
        self.allowed_headers.push("*".to_string());
    }

    pub fn allow_defaults(&mut self) {
        self.allow_all_origins();
        self.allow_all_methods();
        self.allow_all_headers();
    }

    // TODO: This is temporary solution. We need to check claims.
    pub fn validate_token(&self, token: &str) -> bool {
        Self::validate_project_token(self.id, token)
    }
}

// Private functions
impl Project {
    fn cors(&self) -> CorsLayer {
        let mut cors = CorsLayer::new();

        if self.allowed_origins.is_empty() {
            cors = cors.allow_origin(Any);
        } else {
            let origins: Vec<HeaderValue> = self
                .allowed_origins
                .iter()
                .map(|origin| HeaderValue::from_str(origin).unwrap())
                .collect();
            cors = cors.allow_origin(origins);
        }

        let methods: Vec<Method> = {
            if self.allowed_methods.is_empty() {
                vec![Method::GET, Method::POST]
            } else {
                self.allowed_methods
                    .iter()
                    .map(|method| Method::from_bytes(method.as_bytes()).unwrap())
                    .collect()
            }
        };

        let headers: Vec<HeaderName> = {
            if self.allowed_headers.is_empty() {
                vec![HeaderName::from_static("*")]
            } else {
                self.allowed_headers
                    .iter()
                    .map(|header| HeaderName::from_bytes(header.as_bytes()).unwrap())
                    .collect()
            }
        };
        // println!("methods: {:?}", methods);
        // println!("headers: {:?}", headers);
        // println!("cors: {:?}", cors);
        cors.allow_methods(methods).allow_headers(headers)
    }

    fn uuid_to_short_id(uuid: Uuid) -> String {
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes());
        // strip '==', "-", "_" characters
        encoded.replace("=", "").replace("-", "").replace("_", "").to_lowercase()
    }

    // TODO: This is a temporary solution. We need to generate a secure token.
    fn secret_key(&self) -> String {
        Self::generate_secret_key(self.id)
    }
}

impl Project {
    // Middleware to authenticate requests
    // This function is called for every request, and it checks for a valid JWT token
    async fn auth_middleware(
        req: Request<Body>,
        next: Next,
        project: Project,
    ) -> Result<Response<Body>, StatusCode> {
        // Check for the Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            // Extract the token from the header (e.g., "Bearer <token>")
            if let Ok(auth_str) = auth_header.to_str() {
                let parts: Vec<&str> = auth_str.split_whitespace().collect();
                if parts.len() == 2 && parts[0] == "Bearer" {
                    let token = parts[1];
                    if project.validate_token(token) {
                        // Token is valid, pass the request to the next handler
                        return Ok(next.run(req).await);
                    }
                }
            }
        }
        // If the header is missing or invalid, return 401 Unauthorized
        Err(StatusCode::UNAUTHORIZED)
    }

    fn static_router() -> Router {
        let hello_message = format!("Hello! this is inside project");
        let static_router = Router::new()
            .route("/", axum::routing::any(|| async { hello_message }))
            .route("/ping", axum::routing::any(|| async { "pong" }))
            .route("/health", axum::routing::any(|| async { "OK" }));
        static_router
    }

    fn validate_project_token(project_id: Uuid, token: &str) -> bool {
        let jwt_auth = auth::jwt::JwtAuth::new(Self::generate_secret_key(project_id));
        jwt_auth.validate_token(token).is_ok()
    }

    fn generate_secret_key(project_id: Uuid) -> String {
        let salt = "BitSink";
        let s = format!("{}{}", project_id.to_string(), salt);
        digest::digest(&digest::SHA256, s.as_bytes())
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}
