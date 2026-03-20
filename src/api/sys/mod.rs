pub mod auth;
pub mod dashboard;
pub mod dept;
pub mod dict;
pub mod job;
pub mod menu;
pub mod monitor;
pub mod permission;
pub mod role;
pub mod server;
pub mod setting;
pub mod user;
pub mod user_handler;

pub use auth::white_sys;
pub use dashboard::sys_dashboard;
pub use dept::sys_dept;
pub use dict::{sys_dict_data, sys_dict_type};
pub use job::{sys_job, sys_job_info};
pub use menu::menu;
pub use monitor::{sys_cache, sys_login_info, sys_operation_log, sys_user_online};
pub use permission::sys_api_permission;
pub use role::{sys_role, sys_role_api};
pub use server::sys_server_info;
pub use setting::sys_setting;
pub use user::sys_user;

use super::web_path::WebPath;

pub fn init_annotated_routes() {
    user_handler::register_routes();
}

pub fn router_sys() -> WebPath {
    WebPath::new().nest(
        "/sys",
        WebPath::new()
            .nest("/user", sys_user())
            .nest("/menu", menu())
            .nest("/dept", sys_dept())
            .nest("/role", sys_role())
            .nest("/roleapi", sys_role_api())
            .nest("/logininfo", sys_login_info())
            .nest("/useronline", sys_user_online())
            .nest("/dictdata", sys_dict_data())
            .nest("/dicttype", sys_dict_type())
            .nest("/serverinfo", sys_server_info())
            .nest("/job", sys_job())
            .nest("/jobinfo", sys_job_info())
            .nest("/apipermission", sys_api_permission())
            .nest("/dashboard", sys_dashboard())
            .nest("/setting", sys_setting())
            .nest("/operationlog", sys_operation_log())
            .nest("/cache", sys_cache()),
    )
}