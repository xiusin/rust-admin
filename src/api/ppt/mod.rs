pub mod ppt_project;
pub mod ppt_page;
pub mod page_element;
pub mod style_template;
pub mod ai_generation_log;
pub mod document_handler;
pub mod style_handler;
pub mod layout_handler;
pub mod generate_handler;
pub mod ai_edit_handler;
pub mod prompt_handler;
pub mod subscription_handler;
pub mod template_market_handler;
pub mod editor_handler;
pub mod analyze_handler;
pub mod content_handler;
pub mod ws_handler;

pub use ppt_project::ppt_project;
pub use ppt_page::ppt_page;
pub use page_element::page_element;
pub use style_template::style_template;
pub use ai_generation_log::ai_generation_log;
pub use template_market_handler::template_market;

use crate::api::web_path::WebPath;

pub fn router_ppt() -> WebPath {
    WebPath::new().nest(
        "/ppt",
        WebPath::new()
            .nest("/project", ppt_project())
            .nest("/page", ppt_page())
            .nest("/element", page_element())
            .nest("/template", template_market())
            .nest("/style-template", style_template())
            .nest("/ai-log", ai_generation_log())
            .nest("/document", document_handler::document_routes())
            .nest("/style", style_handler::style_routes())
            .nest("/layout", layout_handler::layout_routes())
            .nest("/generate", generate_handler::generate_routes())
            .nest("/ai-edit", ai_edit_handler::ai_edit_routes())
            .nest("/prompt", prompt_handler::prompt_routes())
            .nest("/subscription", subscription_handler::subscription_routes())
            .nest("/payment", subscription_handler::payment_routes())
            .nest("/credit", subscription_handler::credit_routes())
            .nest("/api-key", subscription_handler::api_key_routes())
            .nest("/editor", editor_handler::editor_routes())
            .nest("/analyze", analyze_handler::analyze_routes())
            .nest("/content", content_handler::content_routes())
            .nest("/ws", ws_handler::ws_routes()),
    )
}
