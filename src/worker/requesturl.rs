use crate::common::error::Result;
use crate::domain::model::{
    m_job::SysJobModel,
    m_job_log::{JobLogAdd, SysJobLogModel},
};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

lazy_static::lazy_static! {
    static ref CIRCUIT_BREAKERS: dashmap::DashMap<String, std::sync::Arc<parking_lot::Mutex<CircuitBreaker>>> = dashmap::DashMap::new();
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CbState {
    Closed,
    Open(std::time::Instant),
    HalfOpen,
}

struct CircuitBreaker {
    state: CbState,
    failure_count: u32,
    success_count: u32,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self {
            state: CbState::Closed,
            failure_count: 0,
            success_count: 0,
        }
    }
}

impl CircuitBreaker {
    fn allow_request(&mut self) -> bool {
        match self.state {
            CbState::Closed => true,
            CbState::Open(opened_at) => {
                if opened_at.elapsed() >= std::time::Duration::from_secs(60) {
                    self.state = CbState::HalfOpen;
                    self.success_count = 0;
                    true
                } else {
                    false
                }
            }
            CbState::HalfOpen => true,
        }
    }

    fn record_success(&mut self) {
        match self.state {
            CbState::Closed => {
                self.failure_count = 0;
            }
            CbState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= 2 {
                    self.state = CbState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CbState::Open(_) => {}
        }
    }

    fn record_failure(&mut self) {
        match self.state {
            CbState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= 5 {
                    self.state = CbState::Open(std::time::Instant::now());
                }
            }
            CbState::HalfOpen => {
                self.state = CbState::Open(std::time::Instant::now());
            }
            CbState::Open(_) => {}
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RequestUrlMsg {
    pub url: String,
    pub job_id: i64,
    pub task_type: String,
    pub job_name: String,
    pub job_group: String,
}

#[derive(Clone)]
pub struct RequestUrlWorker {}

impl AppWorker<RequestUrlMsg> for RequestUrlWorker {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Worker<RequestUrlMsg> for RequestUrlWorker {
    fn opts() -> WorkerOpts<RequestUrlMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: RequestUrlMsg) -> Result<()> {
        use chrono::Local;
        info!("request url: {}", arg.url);
        let start_time = Local::now().naive_local();
        
        let host = reqwest::Url::parse(&arg.url)
            .ok()
            .and_then(|u| u.host_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string());
            
        let cb_arc = CIRCUIT_BREAKERS
            .entry(host.clone())
            .or_insert_with(|| std::sync::Arc::new(parking_lot::Mutex::new(CircuitBreaker::default())))
            .clone();

        let allow_request = {
            let mut cb = cb_arc.lock();
            cb.allow_request()
        };

        if !allow_request {
            let end_time = Local::now().naive_local();
            let elapsed = end_time.signed_duration_since(start_time).num_milliseconds() as i32;
            let runcount = SysJobModel::updata_run_count(arg.job_id).await.unwrap_or(0);
            let jog_add = JobLogAdd {
                job_id: arg.job_id,
                job_name: arg.job_name,
                job_group: arg.job_group,
                run_count: runcount,
                elapsed_time: elapsed,
                job_message: Some("熔断器已开启，请求被拒绝".to_string()),
                status: "1".to_string(),
                ..Default::default()
            };
            let _ = SysJobLogModel::add(jog_add).await;
            return Ok(());
        }

        let mut retries = 0;
        let max_retries = 3;
        let mut delay = std::time::Duration::from_secs(1);
        
        let mut final_resp = None;

        while retries <= max_retries {
            match reqwest::get(arg.url.as_str()).await {
                Ok(r) => {
                    if r.status().is_server_error() {
                        final_resp = Some(Ok(r));
                    } else {
                        final_resp = Some(Ok(r));
                        break;
                    }
                }
                Err(e) => {
                    final_resp = Some(Err(e));
                }
            }
            
            if retries < max_retries {
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            retries += 1;
        }

        let end_time = Local::now().naive_local();
        let elapsed = end_time.signed_duration_since(start_time).num_milliseconds() as i32;

        let is_success = match &final_resp {
            Some(Ok(r)) => !r.status().is_server_error(),
            _ => false,
        };

        {
            let mut cb = cb_arc.lock();
            if is_success {
                cb.record_success();
            } else {
                cb.record_failure();
            }
        }

        let runcount = SysJobModel::updata_run_count(arg.job_id).await.unwrap_or(0);
        let mut jog_add = JobLogAdd {
            job_id: arg.job_id,
            job_name: arg.job_name,
            job_group: arg.job_group,
            run_count: runcount,
            elapsed_time: elapsed,
            ..Default::default()
        };

        match final_resp.unwrap() {
            Ok(r) => {
                let txt = r.text().await.unwrap_or_default();
                jog_add.job_message = Some(if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                });
                if is_success {
                    jog_add.status = "0".to_string(); // 0-成功
                } else {
                    jog_add.status = "1".to_string(); // 1-失败
                }
            }
            Err(e) => {
                let txt = e.to_string();
                jog_add.job_message = Some(if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                });
                jog_add.status = "1".to_string(); // 1-失败
            }
        };
        let _ = SysJobLogModel::add(jog_add).await;
        Ok(())
    }
}
