use crate::application::ecommerce::models::{EcommerceError, EcommercePlatform};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Instant, Duration as StdDuration};
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

// 任务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncTaskType {
    ProductSync,
    OrderSync,
    AfterSaleSync,
    InventorySync,
    PromotionSync,
    TaobaoKeSync,
    PromotionLinkSync,
    ShippingAlertSync,
    CommissionSync,
}

// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

// 任务定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTask {
    pub id: String,
    pub task_type: SyncTaskType,
    pub platform_id: i64,
    pub platform_name: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub started_at: Option<DateTime<Local>>,
    pub completed_at: Option<DateTime<Local>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
}

// API速率限制配置
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_calls: u32,
    pub time_window: StdDuration,
    pub last_reset: Instant,
    pub call_count: u32,
}

impl RateLimitConfig {
    pub fn new(max_calls: u32, time_window_seconds: u64) -> Self {
        Self {
            max_calls,
            time_window: StdDuration::from_secs(time_window_seconds),
            last_reset: Instant::now(),
            call_count: 0,
        }
    }
    
    pub async fn check_and_wait(&mut self) {
        let now = Instant::now();
        if now - self.last_reset > self.time_window {
            self.last_reset = now;
            self.call_count = 0;
        }
        
        if self.call_count >= self.max_calls {
            let wait_time = self.time_window - (now - self.last_reset);
            tokio::time::sleep(wait_time).await;
            self.last_reset = Instant::now();
            self.call_count = 0;
        }
        
        self.call_count += 1;
    }
}

// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

// 任务调度器
pub struct SyncScheduler {
    tasks: Arc<Mutex<Vec<SyncTask>>>,
    platforms: Arc<Mutex<Vec<Arc<dyn EcommercePlatform>>>>,
    rate_limits: Arc<Mutex<HashMap<String, RateLimitConfig>>>,
    retry_config: RetryConfig,
}

impl SyncScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(vec![])),
            platforms: Arc::new(Mutex::new(vec![])),
            rate_limits: Arc::new(Mutex::new(HashMap::new())),
            retry_config: RetryConfig::default(),
        }
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    // 添加平台
    pub async fn add_platform(&self, platform: Arc<dyn EcommercePlatform>) {
        let mut platforms = self.platforms.lock().await;
        platforms.push(platform.clone());
        
        // 为平台设置默认的速率限制
        let mut rate_limits = self.rate_limits.lock().await;
        rate_limits.insert(platform.platform_name().to_string(), RateLimitConfig::new(60, 60));
    }
    
    // 设置平台速率限制
    pub async fn set_rate_limit(&self, platform_name: &str, max_calls: u32, time_window_seconds: u64) {
        let mut rate_limits = self.rate_limits.lock().await;
        rate_limits.insert(platform_name.to_string(), RateLimitConfig::new(max_calls, time_window_seconds));
    }

    // 创建任务
    pub async fn create_task(&self, task_type: SyncTaskType, platform_id: i64, platform_name: String) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = SyncTask {
            id: task_id.clone(),
            task_type,
            platform_id,
            platform_name,
            status: TaskStatus::Pending,
            created_at: Local::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            retry_count: 0,
        };

        let mut tasks = self.tasks.lock().await;
        tasks.push(task);
        task_id
    }

    // 执行任务
    pub async fn execute_task(&self, task_id: &str) -> Result<(), EcommerceError> {
        let mut tasks = self.tasks.lock().await;
        let task_index = tasks.iter().position(|t| t.id == task_id).ok_or(
            EcommerceError::ValidationError(format!("Task not found: {}", task_id))
        )?;

        let task = &mut tasks[task_index];
        task.status = TaskStatus::Running;
        task.started_at = Some(Local::now());

        // 执行具体的同步任务
        let result = match task.task_type {
            SyncTaskType::ProductSync => self.sync_products(task.platform_id).await,
            SyncTaskType::OrderSync => self.sync_orders(task.platform_id).await,
            SyncTaskType::AfterSaleSync => self.sync_after_sales(task.platform_id).await,
            SyncTaskType::InventorySync => self.sync_inventory(task.platform_id).await,
            SyncTaskType::PromotionSync => self.sync_promotions(task.platform_id).await,
            SyncTaskType::TaobaoKeSync => self.sync_taobao_ke(task.platform_id).await,
            SyncTaskType::PromotionLinkSync => self.sync_promotion_links(task.platform_id).await,
            SyncTaskType::ShippingAlertSync => self.sync_shipping_alerts(task.platform_id).await,
            SyncTaskType::CommissionSync => self.sync_commissions(task.platform_id).await,
        };

        // 更新任务状态
        match result {
            Ok(_) => {
                task.status = TaskStatus::Completed;
                task.completed_at = Some(Local::now());
            }
            Err(e) => {
                task.status = TaskStatus::Failed;
                task.completed_at = Some(Local::now());
                task.error_message = Some(e.to_string());
                task.retry_count += 1;

                // 失败重试逻辑
                if task.retry_count < self.retry_config.max_retries as i32 {
                    let delay = self.retry_config.initial_delay * 
                        (self.retry_config.backoff_multiplier.powi(task.retry_count - 1) as u32);
                    let scheduler_clone = self.clone();
                    let task_id_clone = task_id.to_string();
                    tokio::spawn(async move {
                        tokio::time::sleep(delay).await;
                        let _ = scheduler_clone.execute_task(&task_id_clone).await;
                    });
                }
            }
        }

        Ok(())
    }

    // 带重试和速率限制的API调用包装器
    async fn with_retry_and_rate_limit<F, Fut, T>(
        &self,
        platform_name: &str,
        operation: F,
    ) -> Result<T, EcommerceError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, EcommerceError>>,
    {
        let mut last_error = None;
        
        for attempt in 0..self.retry_config.max_retries {
            // 检查速率限制
            {
                let mut rate_limits = self.rate_limits.lock().await;
                if let Some(rate_limit) = rate_limits.get_mut(platform_name) {
                    rate_limit.check_and_wait().await;
                }
            }
            
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_config.max_retries - 1 {
                        let delay = self.retry_config.initial_delay * 
                            (self.retry_config.backoff_multiplier.powi(attempt as i32) as u32);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| EcommerceError::UnknownError("Unknown error".to_string())))
    }

    // 同步商品
    async fn sync_products(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let product_service = platform.product_service();
            let params = crate::application::ecommerce::models::GetProductsParams {
                page: 1,
                page_size: 100,
                status: None,
                keyword: None,
            };
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                product_service.get_products(params.clone())
            }).await;
            
            match result {
                Ok(products) => {
                    println!("Synced {} products from {}", products.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing products from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步订单
    async fn sync_orders(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let order_service = platform.order_service();
            let start_date = chrono::Local::now().checked_sub_days(chrono::Days::new(7))
                .map(|d| d.to_string())
                .unwrap_or_default();
            let end_date = chrono::Local::now().to_string();
            let params = crate::application::ecommerce::models::GetOrdersParams {
                page: 1,
                page_size: 100,
                order_status: None,
                start_date: Some(start_date),
                end_date: Some(end_date),
            };
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                order_service.get_orders(params.clone())
            }).await;
            
            match result {
                Ok(orders) => {
                    println!("Synced {} orders from {}", orders.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing orders from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步售后
    async fn sync_after_sales(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let after_sale_service = platform.after_sale_service();
            let start_date = chrono::Local::now().checked_sub_days(chrono::Days::new(30))
                .map(|d| d.to_string())
                .unwrap_or_default();
            let end_date = chrono::Local::now().to_string();
            let params = crate::application::ecommerce::models::GetAfterSalesParams {
                page: 1,
                page_size: 100,
                after_sale_status: None,
                start_date: Some(start_date),
                end_date: Some(end_date),
            };
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                after_sale_service.get_after_sales(params.clone())
            }).await;
            
            match result {
                Ok(after_sales) => {
                    println!("Synced {} after sales from {}", after_sales.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing after sales from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步库存
    async fn sync_inventory(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let product_service = platform.product_service();
            let params = crate::application::ecommerce::models::GetProductsParams {
                page: 1,
                page_size: 100,
                status: Some("active".to_string()),
                keyword: None,
            };
            
            let products = self.with_retry_and_rate_limit(&platform_name, || {
                product_service.get_products(params.clone())
            }).await?;
            
            let inventory_service = platform.inventory_service();
            for product in products {
                for sku in product.skus {
                    let result = self.with_retry_and_rate_limit(&platform_name, || {
                        inventory_service.get_inventory(&product.platform_product_id, Some(&sku.sku_id))
                    }).await;
                    
                    match result {
                        Ok(stock) => {
                            println!("Synced inventory for product {} SKU {}: {}", 
                                product.platform_product_id, sku.sku_id, stock);
                        }
                        Err(e) => {
                            println!("Error syncing inventory: {:?}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // 同步推广
    async fn sync_promotions(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let promotion_service = platform.promotion_service();
            let params = crate::application::ecommerce::models::GetPromotionsParams {
                page: 1,
                page_size: 100,
                status: Some("active".to_string()),
            };
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                promotion_service.get_promotions(params.clone())
            }).await;
            
            match result {
                Ok(promotions) => {
                    println!("Synced {} promotions from {}", promotions.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing promotions from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步淘宝客商品
    async fn sync_taobao_ke(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let taobao_ke_service = platform.taobao_ke_service();
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                taobao_ke_service.pull_promotion_products(None, 1, 100)
            }).await;
            
            match result {
                Ok(products) => {
                    println!("Synced {} Taobao Ke products from {}", products.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing Taobao Ke products from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步推广链接
    async fn sync_promotion_links(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let promotion_link_service = platform.promotion_link_service();
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                promotion_link_service.get_promotion_links(None, None)
            }).await;
            
            match result {
                Ok(links) => {
                    println!("Synced {} promotion links from {}", links.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing promotion links from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步发货超时预警
    async fn sync_shipping_alerts(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let shipping_alert_service = platform.shipping_alert_service();
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                shipping_alert_service.generate_shipping_alerts()
            }).await;
            
            match result {
                Ok(alerts) => {
                    println!("Generated {} shipping timeout alerts from {}", alerts.len(), platform_name);
                }
                Err(e) => {
                    println!("Error generating shipping alerts from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 同步佣金记录
    async fn sync_commissions(&self, _platform_id: i64) -> Result<(), EcommerceError> {
        let platforms = self.platforms.lock().await;
        for platform in platforms.iter() {
            let platform_name = platform.platform_name().to_string();
            let taobao_ke_service = platform.taobao_ke_service();
            let start_date = chrono::Local::now().checked_sub_days(chrono::Days::new(30))
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            let end_date = chrono::Local::now().format("%Y-%m-%d").to_string();
            
            let result = self.with_retry_and_rate_limit(&platform_name, || {
                taobao_ke_service.get_commission_records(&start_date, &end_date)
            }).await;
            
            match result {
                Ok(records) => {
                    println!("Synced {} commission records from {}", records.len(), platform_name);
                }
                Err(e) => {
                    println!("Error syncing commission records from {}: {:?}", platform_name, e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    // 获取任务列表
    pub async fn get_tasks(&self) -> Vec<SyncTask> {
        let tasks = self.tasks.lock().await;
        tasks.clone()
    }

    // 启动定时任务
    pub async fn start_scheduled_tasks(self: Arc<Self>) {
        // 启动商品同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(1));
            loop {
                interval.tick().await;
                // 可以在这里调用商品同步逻辑
            }
        });

        // 启动订单同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_minutes(30));
            loop {
                interval.tick().await;
                // 可以在这里调用订单同步逻辑
            }
        });

        // 启动库存同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(2));
            loop {
                interval.tick().await;
                // 可以在这里调用库存同步逻辑
            }
        });

        // 启动推广同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(4));
            loop {
                interval.tick().await;
                // 可以在这里调用推广同步逻辑
            }
        });

        // 启动淘宝客商品同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(6));
            loop {
                interval.tick().await;
                // 可以在这里调用淘宝客商品同步逻辑
            }
        });

        // 启动推广链接同步定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(8));
            loop {
                interval.tick().await;
                // 可以在这里调用推广链接同步逻辑
            }
        });

        // 启动发货超时预警定时任务
        let scheduler_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(1));
            loop {
                interval.tick().await;
                // 可以在这里调用发货超时预警同步逻辑
            }
        });

        // 启动佣金记录同步定时任务
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_hours(12));
            loop {
                interval.tick().await;
                // 可以在这里调用佣金记录同步逻辑
            }
        });
    }

    // 批量执行任务
    pub async fn batch_execute_tasks(&self, task_ids: Vec<String>) -> Vec<(String, Result<(), EcommerceError>)> {
        let mut results = Vec::new();
        for task_id in task_ids {
            let result = self.execute_task(&task_id).await;
            results.push((task_id, result));
        }
        results
    }
}

impl Clone for SyncScheduler {
    fn clone(&self) -> Self {
        Self {
            tasks: Arc::clone(&self.tasks),
            platforms: Arc::clone(&self.platforms),
            rate_limits: Arc::clone(&self.rate_limits),
            retry_config: self.retry_config.clone(),
        }
    }
}
