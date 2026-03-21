use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::core::route::{RouteInfo, AuthConfig, LogConfig, RateLimitConfig};

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub info: RouteInfo,
    pub auth: Option<AuthConfig>,
    pub log: Option<LogConfig>,
    pub rate_limit: Option<RateLimitConfig>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedHandler {
    pub info: RouteInfo,
    pub auth: Option<AuthConfig>,
    pub log: Option<LogConfig>,
    pub rate_limit: Option<RateLimitConfig>,
}

impl AnnotatedHandler {
    pub fn new(info: RouteInfo) -> Self {
        Self {
            info,
            auth: None,
            log: None,
            rate_limit: None,
        }
    }

    pub fn with_auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn with_log(mut self, log: LogConfig) -> Self {
        self.log = Some(log);
        self
    }

    pub fn with_rate_limit(mut self, rate_limit: RateLimitConfig) -> Self {
        self.rate_limit = Some(rate_limit);
        self
    }
}

pub struct RouteRegistry {
    routes: Mutex<HashMap<String, AnnotatedHandler>>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, name: &str, handler: AnnotatedHandler) {
        let mut routes = self.routes.lock().unwrap();
        if routes.contains_key(name) {
            tracing::warn!("Route handler '{}' already registered, skipping", name);
            return;
        }
        routes.insert(name.to_string(), handler);
        let path = routes.get(name).map(|h| format!("{} {}", h.info.method.as_str(), h.info.path)).unwrap_or_default();
        tracing::info!("Registered: {} -> {}", name, path);
    }

    pub fn get_all_routes(&self) -> Vec<(String, AnnotatedHandler)> {
        let routes = self.routes.lock().unwrap();
        routes.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn get_routes_by_tag(&self, tag: &str) -> Vec<(String, AnnotatedHandler)> {
        let routes = self.routes.lock().unwrap();
        routes.iter()
            .filter(|(_, v)| v.info.tag == tag)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn print_routes(&self) {
        let routes = self.get_all_routes();
        tracing::info!("=== Registered Annotated Routes ===");
        for (name, entry) in &routes {
            let auth_info = entry.auth.as_ref()
                .map(|a| if a.required { "auth=required" } else { "auth=optional" })
                .unwrap_or("no auth");
            tracing::info!(
                "  {} {} -> {} ({}) [{}]",
                entry.info.method.as_str(),
                entry.info.path,
                name,
                entry.info.tag,
                auth_info
            );
        }
        tracing::info!("Total: {} annotated routes", routes.len());
    }
}

impl Default for RouteRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub static ROUTE_REGISTRY: Lazy<RouteRegistry> = Lazy::new(|| RouteRegistry::new());

#[macro_export]
macro_rules! collect_route {
    ($name:expr, $handler_fn:expr) => {
        $crate::core::route::ROUTE_REGISTRY.register(
            $name,
            $handler_fn,
        );
    };
}