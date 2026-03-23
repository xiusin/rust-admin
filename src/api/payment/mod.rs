pub mod pay_channel;

use crate::api::web_path::WebPath;

pub fn router_payment() -> WebPath {
    WebPath::new()
        .nest("/pay_channel", pay_channel::pay_channel())
}

pub use pay_channel::pay_channel;
