#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PUT" => Some(HttpMethod::Put),
            "DELETE" => Some(HttpMethod::Delete),
            "PATCH" => Some(HttpMethod::Patch),
            "HEAD" => Some(HttpMethod::Head),
            "OPTIONS" => Some(HttpMethod::Options),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub path: String,
    pub method: HttpMethod,
    pub handler_name: String,
    pub tag: String,
    pub summary: String,
}

impl RouteInfo {
    pub fn new(path: impl Into<String>, method: HttpMethod, handler_name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            method,
            handler_name: handler_name.into(),
            tag: String::new(),
            summary: String::new(),
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
        self
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthConfig {
    pub required: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub operation: String,
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests: u32,
    pub period: String,
}

pub trait RouteMeta: Send + Sync {
    fn route_meta() -> RouteInfo;
}

pub trait AuthMeta: Send + Sync {
    fn auth_meta() -> AuthConfig;
}

pub trait LogMeta: Send + Sync {
    fn log_meta() -> LogConfig;
}

pub trait RateLimitMeta: Send + Sync {
    fn rate_limit_meta() -> RateLimitConfig;
}