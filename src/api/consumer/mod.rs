pub mod consumer;
pub mod finance;
pub mod freight;
pub mod logistics;
pub mod media;
pub mod payment;
pub mod sms;
pub mod wechat;
pub mod after_sale;
pub mod user_extension;
pub mod config;

use crate::api::web_path::WebPath;

pub fn router_consumer() -> WebPath {
    WebPath::new()
        .nest("/consumer", consumer::consumer_api())
        .nest("/sms", sms::sms_api())
        .nest("/payment", payment::payment_api())
        .nest("/finance", finance::finance_api())
        .nest("/wechat", wechat::wechat_api())
        .nest("/media", media::media_api())
        .nest("/logistics", logistics::logistics_api())
        .nest("/freight", freight::freight_api())
        .nest("/after-sale", after_sale::after_sale_api())
        .nest("/user-ext", user_extension::user_extension_api())
        .merge(config::config_api())
}
