use std::collections::HashMap;
use crate::core::route::{RouteInfo, AuthConfig, LogConfig, RateLimitConfig};

pub struct RouteCollector {
    routes: Vec<RouteRegistration>,
}

struct RouteRegistration {
    info: RouteInfo,
    auth: Option<AuthConfig>,
    log: Option<LogConfig>,
    rate_limit: Option<RateLimitConfig>,
}

impl RouteCollector {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn register<H: crate::core::route::RouteMeta>(&mut self) -> &mut Self {
        let info = H::route_meta();
        let auth = None;
        let log = None;
        let rate_limit = None;

        if std::any::type_name::<H>().contains("list") {
            self.routes.push(RouteRegistration {
                info,
                auth: Some(crate::core::route::AuthConfig {
                    required: true,
                    roles: vec!["admin".to_string()],
                    permissions: vec![],
                }),
                log: Some(crate::core::route::LogConfig {
                    operation: "查询用户列表".to_string(),
                }),
                rate_limit: None,
            });
        }

        self
    }

    pub fn get_routes(&self) -> &Vec<RouteRegistration> {
        &self.routes
    }

    pub fn get_routes_by_tag(&self, tag: &str) -> Vec<&RouteRegistration> {
        self.routes.iter().filter(|r| r.info.tag == tag).collect()
    }
}

impl Default for RouteCollector {
    fn default() -> Self {
        Self::new()
    }
}