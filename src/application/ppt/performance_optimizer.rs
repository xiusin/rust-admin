use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock};

pub struct PerformanceOptimizer {
    concurrent_limiter: Arc<Semaphore>,
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

#[derive(Default)]
pub struct PerformanceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time_ms: f64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl PerformanceOptimizer {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            concurrent_limiter: Arc::new(Semaphore::new(max_concurrent)),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    pub async fn acquire_semaphore(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.concurrent_limiter.acquire().await.unwrap()
    }

    pub async fn record_request(&self, success: bool, response_time_ms: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }
        
        let total = metrics.total_requests;
        let current_avg = metrics.avg_response_time_ms;
        metrics.avg_response_time_ms = 
            (current_avg * (total - 1) as f64 + response_time_ms as f64) / total as f64;
    }

    pub async fn record_cache_hit(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_hits += 1;
    }

    pub async fn record_cache_miss(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_misses += 1;
    }

    pub async fn get_metrics(&self) -> PerformanceMetrics {
        let metrics = self.metrics.read().await;
        PerformanceMetrics {
            total_requests: metrics.total_requests,
            successful_requests: metrics.successful_requests,
            failed_requests: metrics.failed_requests,
            avg_response_time_ms: metrics.avg_response_time_ms,
            cache_hits: metrics.cache_hits,
            cache_misses: metrics.cache_misses,
        }
    }

    pub fn get_cache_hit_rate(&self, metrics: &PerformanceMetrics) -> f64 {
        let total = metrics.cache_hits + metrics.cache_misses;
        if total == 0 {
            return 0.0;
        }
        metrics.cache_hits as f64 / total as f64
    }

    pub fn get_success_rate(&self, metrics: &PerformanceMetrics) -> f64 {
        if metrics.total_requests == 0 {
            return 0.0;
        }
        metrics.successful_requests as f64 / metrics.total_requests as f64
    }
}

pub struct QueryOptimizer;

impl QueryOptimizer {
    pub fn get_recommended_indexes() -> Vec<String> {
        vec![
            "CREATE INDEX idx_ppt_project_user ON ppt_project(user_id)".to_string(),
            "CREATE INDEX idx_ppt_project_status ON ppt_project(status)".to_string(),
            "CREATE INDEX idx_ppt_page_project ON ppt_page(project_id)".to_string(),
            "CREATE INDEX idx_page_element_page ON page_element(page_id)".to_string(),
            "CREATE INDEX idx_style_template_industry ON ppt_style_template(industry)".to_string(),
            "CREATE INDEX idx_ai_log_project ON ppt_ai_generation_log(project_id)".to_string(),
            "CREATE INDEX idx_ai_log_created ON ppt_ai_generation_log(created_at)".to_string(),
        ]
    }

    pub fn optimize_select_query(query: &str) -> String {
        let mut optimized = query.to_string();
        
        if !optimized.to_lowercase().contains(" limit ") {
            optimized.push_str(" LIMIT 1000");
        }
        
        optimized
    }
}

pub struct AICallOptimizer {
    batch_size: usize,
    batch_timeout_ms: u64,
}

impl AICallOptimizer {
    pub fn new(batch_size: usize, batch_timeout_ms: u64) -> Self {
        Self {
            batch_size,
            batch_timeout_ms,
        }
    }

    pub fn should_batch(&self, pending_count: usize) -> bool {
        pending_count >= self.batch_size
    }

    pub fn get_batch_timeout_ms(&self) -> u64 {
        self.batch_timeout_ms
    }

    pub fn estimate_tokens(text: &str) -> usize {
        let char_count = text.chars().count();
        let chinese_chars = text.chars().filter(|c| c > &'\u{4e00}' && c < &'\u{9fff}').count();
        let other_chars = char_count - chinese_chars;
        
        (chinese_chars * 2 + other_chars / 4) as usize
    }

    pub fn truncate_for_token_limit(text: &str, max_tokens: usize) -> String {
        let estimated = Self::estimate_tokens(text);
        if estimated <= max_tokens {
            return text.to_string();
        }
        
        let ratio = max_tokens as f64 / estimated as f64;
        let target_chars = (text.chars().count() as f64 * ratio) as usize;
        
        text.chars().take(target_chars).collect()
    }
}

pub struct FileProcessingOptimizer;

impl FileProcessingOptimizer {
    pub fn get_chunk_size(file_size: u64) -> u64 {
        match file_size {
            0..=1_000_000 => 64 * 1024,
            1_000_001..=10_000_000 => 256 * 1024,
            10_000_001..=100_000_000 => 1024 * 1024,
            _ => 4 * 1024 * 1024,
        }
    }

    pub fn should_use_streaming(file_size: u64) -> bool {
        file_size > 10_000_000
    }

    pub fn get_optimal_thread_count() -> usize {
        let num_cpus = std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4);
        (num_cpus * 2).max(4).min(16)
    }
}
