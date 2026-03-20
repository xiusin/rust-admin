pub mod api_permission_service;
pub mod cache_service;
pub mod captcha_service;
pub mod dashboard_service;
pub mod dept_service;
pub mod menu_service;
pub mod role_api_service;
pub mod role_service;
pub mod test_service;
pub mod upload_service;
pub mod user_dept_service;
pub mod user_role_service;
pub mod user_service;
pub mod white_jwt_service;

pub use api_permission_service as s_sys_api_permission;
pub use cache_service as s_sys_cache;
pub use captcha_service as s_sys_captcha;
pub use dashboard_service as s_sys_dashboard;
pub use dept_service as s_sys_dept;
pub use menu_service as s_sys_menu;
pub use role_api_service as s_sys_role_api;
pub use role_service as s_sys_role;
pub use test_service as s_sys_test;
pub use upload_service as s_sys_upload;
pub use user_dept_service as s_sys_user_dept;
pub use user_role_service as s_sys_user_role;
pub use user_service as s_sys_user;
pub use white_jwt_service as s_sys_white_jwt;

pub use crate::application::monitor::job_service as s_sys_job;
pub use crate::application::monitor::job_log_service as s_sys_job_log;
pub use crate::application::monitor::login_info_service as s_sys_login_info;
pub use crate::application::monitor::operation_log_service as s_sys_operation_log;
pub use crate::application::monitor::server_info_service as s_sys_server_info;

pub use crate::application::system::dict_data_service as s_sys_dict_data;
pub use crate::application::system::dict_type_service as s_sys_dict_type;
