use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,Default,FromQueryResult)]
pub struct JobLogRes {
    #[serde(with = "i64_to_string")]
    pub log_id: i64,
    #[serde(with = "i64_to_string")]
    pub job_id: i64,
    pub job_name: String,
    pub job_group: String,
    pub run_count: i32,
    pub job_message: Option<String>,
    pub status: String,
    pub elapsed_time: i32,
    pub created_at: DateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct JobLogAdd {
    pub job_id: i64,
    pub job_name: String,
    pub job_group: String,
    pub run_count: i32,
    pub job_message: Option<String>,
    pub status: String,
    pub elapsed_time: i32,
}
 
#[derive(Debug, Clone, Serialize, Deserialize,Default,Validate)]
pub struct JobLogSearch {  
    pub job_id: String
}