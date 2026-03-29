use crate::common::error::{Error, Result};
use crate::infrastructure::ai::types::{GenerateOptions, ImageOptions, GeneratedImage, LLMService, ImageService};
use crate::infrastructure::ai::config::CircuitBreakerConfig;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;

const STATE_CLOSED: u8 = 0;
const STATE_OPEN: u8 = 1;
const STATE_HALF_OPEN: u8 = 2;

#[derive(Debug)]
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    failure_threshold: u32,
    last_failure_time: AtomicU64,
    recovery_timeout: Duration,
    state: AtomicU8,
    half_open_calls: AtomicU32,
    half_open_max_calls: u32,
    last_state_change: RwLock<Instant>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            failure_count: AtomicU32::new(0),
            failure_threshold: config.failure_threshold,
            last_failure_time: AtomicU64::new(0),
            recovery_timeout: Duration::from_millis(config.recovery_timeout_ms),
            state: AtomicU8::new(STATE_CLOSED),
            half_open_calls: AtomicU32::new(0),
            half_open_max_calls: config.half_open_max_calls,
            last_state_change: RwLock::new(Instant::now()),
        }
    }

    pub fn is_available(&self) -> bool {
        let state = self.state.load(Ordering::SeqCst);
        
        match state {
            STATE_CLOSED => true,
            STATE_OPEN => {
                let last_failure = self.last_failure_time.load(Ordering::SeqCst);
                let elapsed = Instant::now()
                    .duration_since(Instant::now() - Duration::from_secs(last_failure));
                
                if elapsed >= self.recovery_timeout {
                    self.transition_to_half_open();
                    true
                } else {
                    false
                }
            }
            STATE_HALF_OPEN => {
                let current_calls = self.half_open_calls.load(Ordering::SeqCst);
                current_calls < self.half_open_max_calls
            }
            _ => false,
        }
    }

    pub fn record_success(&self) {
        let state = self.state.load(Ordering::SeqCst);
        
        if state == STATE_HALF_OPEN {
            self.half_open_calls.fetch_sub(1, Ordering::SeqCst);
            let success_count = self.half_open_max_calls - self.half_open_calls.load(Ordering::SeqCst);
            
            if success_count >= self.half_open_max_calls {
                self.transition_to_closed();
            }
        } else if state == STATE_CLOSED {
            self.failure_count.store(0, Ordering::SeqCst);
        }
    }

    pub fn record_failure(&self) {
        let state = self.state.load(Ordering::SeqCst);
        
        match state {
            STATE_CLOSED => {
                let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                self.last_failure_time.store(
                    Instant::now().elapsed().as_secs(),
                    Ordering::SeqCst,
                );
                
                if failures >= self.failure_threshold {
                    self.transition_to_open();
                }
            }
            STATE_HALF_OPEN => {
                self.half_open_calls.fetch_sub(1, Ordering::SeqCst);
                self.transition_to_open();
            }
            STATE_OPEN => {
                self.last_failure_time.store(
                    Instant::now().elapsed().as_secs(),
                    Ordering::SeqCst,
                );
            }
            _ => {}
        }
    }

    fn transition_to_open(&self) {
        self.state.store(STATE_OPEN, Ordering::SeqCst);
        if let Ok(mut last_change) = self.last_state_change.try_write() {
            *last_change = Instant::now();
        }
    }

    fn transition_to_half_open(&self) {
        self.state.store(STATE_HALF_OPEN, Ordering::SeqCst);
        self.half_open_calls.store(0, Ordering::SeqCst);
        if let Ok(mut last_change) = self.last_state_change.try_write() {
            *last_change = Instant::now();
        }
    }

    fn transition_to_closed(&self) {
        self.state.store(STATE_CLOSED, Ordering::SeqCst);
        self.failure_count.store(0, Ordering::SeqCst);
        if let Ok(mut last_change) = self.last_state_change.try_write() {
            *last_change = Instant::now();
        }
    }

    pub fn state(&self) -> CircuitState {
        match self.state.load(Ordering::SeqCst) {
            STATE_CLOSED => CircuitState::Closed,
            STATE_OPEN => CircuitState::Open,
            STATE_HALF_OPEN => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        }
    }

    pub fn failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

pub struct AIServicePool {
    primary: Arc<(Box<dyn LLMService>, CircuitBreaker)>,
    fallback: Vec<Arc<(Box<dyn LLMService>, CircuitBreaker)>>,
}

impl AIServicePool {
    pub fn new(primary: Box<dyn LLMService>, fallback: Vec<Box<dyn LLMService>>, config: CircuitBreakerConfig) -> Self {
        let primary = Arc::new((primary, CircuitBreaker::new(config.clone())));
        let fallback = fallback
            .into_iter()
            .map(|s| Arc::new((s, CircuitBreaker::new(config.clone()))))
            .collect();
        
        Self { primary, fallback }
    }

    pub async fn generate(&self, prompt: &str, options: GenerateOptions) -> Result<String> {
        if self.primary.1.is_available() {
            match self.primary.0.generate(prompt, options.clone()).await {
                Ok(result) => {
                    self.primary.1.record_success();
                    return Ok(result);
                }
                Err(e) => {
                    self.primary.1.record_failure();
                    tracing::warn!("Primary LLM service failed: {}, trying fallback", e);
                }
            }
        }

        for fallback_service in &self.fallback {
            if fallback_service.1.is_available() {
                match fallback_service.0.generate(prompt, options.clone()).await {
                    Ok(result) => {
                        fallback_service.1.record_success();
                        return Ok(result);
                    }
                    Err(e) => {
                        fallback_service.1.record_failure();
                        tracing::warn!("Fallback LLM service failed: {}", e);
                    }
                }
            }
        }

        Err(Error::internal_error("All LLM services are unavailable"))
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        if self.primary.1.is_available() {
            match self.primary.0.embed(text).await {
                Ok(result) => {
                    self.primary.1.record_success();
                    return Ok(result);
                }
                Err(e) => {
                    self.primary.1.record_failure();
                    tracing::warn!("Primary embedding service failed: {}", e);
                }
            }
        }

        for fallback_service in &self.fallback {
            if fallback_service.1.is_available() {
                match fallback_service.0.embed(text).await {
                    Ok(result) => {
                        fallback_service.1.record_success();
                        return Ok(result);
                    }
                    Err(e) => {
                        fallback_service.1.record_failure();
                        tracing::warn!("Fallback embedding service failed: {}", e);
                    }
                }
            }
        }

        Err(Error::internal_error("All embedding services are unavailable"))
    }
}

pub struct ImageServicePool {
    primary: Arc<(Box<dyn ImageService>, CircuitBreaker)>,
    fallback: Vec<Arc<(Box<dyn ImageService>, CircuitBreaker)>>,
}

impl ImageServicePool {
    pub fn new(primary: Box<dyn ImageService>, fallback: Vec<Box<dyn ImageService>>, config: CircuitBreakerConfig) -> Self {
        let primary = Arc::new((primary, CircuitBreaker::new(config.clone())));
        let fallback = fallback
            .into_iter()
            .map(|s| Arc::new((s, CircuitBreaker::new(config.clone()))))
            .collect();
        
        Self { primary, fallback }
    }

    pub async fn generate_image(&self, prompt: &str, options: ImageOptions) -> Result<GeneratedImage> {
        if self.primary.1.is_available() {
            match self.primary.0.generate_image(prompt, options.clone()).await {
                Ok(result) => {
                    self.primary.1.record_success();
                    return Ok(result);
                }
                Err(e) => {
                    self.primary.1.record_failure();
                    tracing::warn!("Primary image service failed: {}, trying fallback", e);
                }
            }
        }

        for fallback_service in &self.fallback {
            if fallback_service.1.is_available() {
                match fallback_service.0.generate_image(prompt, options.clone()).await {
                    Ok(result) => {
                        fallback_service.1.record_success();
                        return Ok(result);
                    }
                    Err(e) => {
                        fallback_service.1.record_failure();
                        tracing::warn!("Fallback image service failed: {}", e);
                    }
                }
            }
        }

        Err(Error::internal_error("All image services are unavailable"))
    }
}
