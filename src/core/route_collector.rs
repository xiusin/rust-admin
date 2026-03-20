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

    pub fn register_with_meta(
        &mut self,
        info: RouteInfo,
        auth: Option<AuthConfig>,
        log: Option<LogConfig>,
        rate_limit: Option<RateLimitConfig>,
    ) -> &mut Self {
        self.routes.push(RouteRegistration {
            info,
            auth,
            log,
            rate_limit,
        });
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