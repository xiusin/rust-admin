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

pub struct RouteRegistry {
    routes: Mutex<HashMap<String, RouteEntry>>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, handler_name: &str, entry: RouteEntry) {
        let mut routes = self.routes.lock().unwrap();
        if routes.contains_key(handler_name) {
            tracing::warn!("Route handler '{}' already registered, skipping", handler_name);
            return;
        }
        let entry_clone = entry.clone();
        routes.insert(handler_name.to_string(), entry);
        tracing::info!("Registered route: {} -> {} {}", entry_clone.info.method.as_str(), entry_clone.info.path, handler_name);
    }

    pub fn get_all_routes(&self) -> Vec<(String, RouteEntry)> {
        let routes = self.routes.lock().unwrap();
        routes.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn get_routes_by_tag(&self, tag: &str) -> Vec<(String, RouteEntry)> {
        let routes = self.routes.lock().unwrap();
        routes.iter()
            .filter(|(_, v)| v.info.tag == tag)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn print_routes(&self) {
        let routes = self.get_all_routes();
        tracing::info!("=== Registered Routes ===");
        for (name, entry) in &routes {
            tracing::info!(
                "  {} {} -> {} ({})",
                entry.info.method.as_str(),
                entry.info.path,
                name,
                entry.info.tag
            );
        }
        tracing::info!("Total: {} routes", routes.len());
    }
}

pub static ROUTE_REGISTRY: Lazy<RouteRegistry> = Lazy::new(|| RouteRegistry::new());

#[macro_export]
macro_rules! register_route {
    ($name:expr, $info:expr) => {
        $crate::core::route::ROUTE_REGISTRY.register(
            $name,
            $crate::core::route::RouteEntry {
                info: $info,
                auth: None,
                log: None,
                rate_limit: None,
            },
        );
    };
    ($name:expr, $info:expr, auth: $auth:expr) => {
        $crate::core::route::ROUTE_REGISTRY.register(
            $name,
            $crate::core::route::RouteEntry {
                info: $info,
                auth: Some($auth),
                log: None,
                rate_limit: None,
            },
        );
    };
    ($name:expr, $info:expr, auth: $auth:expr, log: $log:expr) => {
        $crate::core::route::ROUTE_REGISTRY.register(
            $name,
            $crate::core::route::RouteEntry {
                info: $info,
                auth: Some($auth),
                log: Some($log),
                rate_limit: None,
            },
        );
    };
    ($name:expr, $info:expr, auth: $auth:expr, log: $log:expr, rate_limit: $rl:expr) => {
        $crate::core::route::ROUTE_REGISTRY.register(
            $name,
            $crate::core::route::RouteEntry {
                info: $info,
                auth: Some($auth),
                log: Some($log),
                rate_limit: Some($rl),
            },
        );
    };
}