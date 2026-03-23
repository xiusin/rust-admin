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
        let url = arg.url;
        let resp = reqwest::get(url.as_str()).await;
        let end_time = Local::now().naive_local();
        let elapsed = end_time.signed_duration_since(start_time).num_milliseconds() as i32;

        let runcount = SysJobModel::updata_run_count(arg.job_id).await.unwrap_or(0);
        let mut jog_add = JobLogAdd {
            job_id: arg.job_id,
            job_name: arg.job_name,
            job_group: arg.job_group,
            run_count: runcount,
            elapsed_time: elapsed,
            ..Default::default()
        };
        match resp {
            Ok(r) => {
                let txt = r.text().await.unwrap_or_default();
                jog_add.job_message = Some(if txt.len() > 2048 {
                    let truncated = &txt[..2000];
                    format!("{truncated} ...数据太长，不记录完整内容。")
                } else {
                    txt
                });
                jog_add.status = "0".to_string(); // 0-成功
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
