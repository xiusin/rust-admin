use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use crate::common::error::Result;

pub struct CacheService {
    redis: Option<redis::aio::MultiplexedConnection>,
    enabled: bool,
    default_ttl: u64,
}

impl CacheService {
    pub fn new(redis: Option<redis::aio::MultiplexedConnection>, enabled: bool, default_ttl: u64) -> Self {
        Self {
            redis,
            enabled,
            default_ttl,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        if !self.enabled {
            return None;
        }

        if let Some(mut redis) = self.redis.clone() {
            if let Ok(data) = redis.get::<_, String>(key).await {
                if let Ok(value) = serde_json::from_str(&data) {
                    return Some(value);
                }
            }
        }
        None
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    pub async fn set_with_ttl<T: Serialize>(&self, key: &str, value: &T, ttl: u64) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(mut redis) = self.redis.clone() {
            let data = serde_json::to_string(value)?;
            let _: () = redis.set_ex(key, data, ttl).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(mut redis) = self.redis.clone() {
            let _: () = redis.del(key).await?;
        }
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(mut redis) = self.redis.clone() {
            if let Ok(exists) = redis.exists(key).await {
                return exists;
            }
        }
        false
    }

    pub async fn get_or_set<T, F, Fut>(
        &self,
        key: &str,
        f: F,
        ttl: u64,
    ) -> Result<T>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        if let Some(cached) = self.get(key).await {
            return Ok(cached);
        }

        let value = f().await?;
        self.set_with_ttl(key, &value, ttl).await?;
        Ok(value)
    }

    pub fn build_key(&self, prefix: &str, id: i64) -> String {
        format!("ppt:{}:{}", prefix, id)
    }

    pub fn build_style_key(&self, industry: &str) -> String {
        format!("ppt:style:{}", industry)
    }

    pub fn build_template_key(&self, template_id: i64) -> String {
        format!("ppt:template:{}", template_id)
    }

    pub fn build_analysis_key(&self, doc_hash: &str) -> String {
        format!("ppt:analysis:{}", doc_hash)
    }
}

pub struct CacheKeys;

impl CacheKeys {
    pub const STYLE_PREFIX: &'static str = "ppt:style";
    pub const TEMPLATE_PREFIX: &'static str = "ppt:template";
    pub const ANALYSIS_PREFIX: &'static str = "ppt:analysis";
    pub const PROJECT_PREFIX: &'static str = "ppt:project";
    pub const USER_CREDITS: &'static str = "ppt:credits";
}
