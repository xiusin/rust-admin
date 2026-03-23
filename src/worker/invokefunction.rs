use crate::common::error::Result;
use crate::domain::model::{
    m_job::SysJobModel,
    m_job_log::{JobLogAdd, SysJobLogModel},
};
use crate::service::sys::{s_sys_api_permission, s_sys_white_jwt};
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct InvokeFunctionMsg {
    pub job_id: Option<i64>,
    pub callfun: String,
    pub parmets: String,
}

#[derive(Clone)]
pub struct InvokeFunctionWorker {}

impl AppWorker<InvokeFunctionMsg> for InvokeFunctionWorker {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Worker<InvokeFunctionMsg> for InvokeFunctionWorker {
    fn opts() -> WorkerOpts<InvokeFunctionMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: InvokeFunctionMsg) -> Result<()> {
        use chrono::Local;
        // info!("InvokeFunctionWorker perform: {:?}", arg);
        let start_time = Local::now().naive_local();
        let message = match arg.callfun.as_str() {
            "updateapi" => s_sys_api_permission::update_all_api(arg.parmets).await,
            "clearuserinfo" => s_sys_white_jwt::clear_user_info().await,
            _ => Ok("未找到对应的方法".to_string()),
        };
        let end_time = Local::now().naive_local();
        let elapsed = end_time.signed_duration_since(start_time).num_milliseconds() as i32;

        if let Some(job_id) = arg.job_id {
            let runcount = SysJobModel::updata_run_count(job_id).await.unwrap_or(0);
            let mut jog_add = JobLogAdd {
                job_id,
                job_name: arg.callfun.clone(),
                job_group: "invoke".to_string(),
                run_count: runcount,
                elapsed_time: elapsed,
                ..Default::default()
            };
            match message {
                Ok(r) => {
                    let txt = r;
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
        }

        Ok(())
    }
}
