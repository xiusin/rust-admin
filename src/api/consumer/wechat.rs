use axum::routing::{get, post};
use crate::api::web_path::{WebPath, WebPathType};
use crate::common::error::Error;
use crate::common::result::ApiResponse;
use axum::Json;

#[derive(Debug, serde::Deserialize)]
pub struct WechatAuthUrlArgs {
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct WechatCallbackArgs {
    pub code: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MiniLoginArgs {
    pub code: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AuthUrlResp {
    pub url: String,
}

#[derive(Debug, serde::Serialize)]
pub struct WechatCallbackResp {
    pub openid: String,
    pub unionid: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct MiniLoginResp {
    pub openid: String,
    pub session_key: String,
}

pub async fn get_auth_url(
    Json(args): Json<WechatAuthUrlArgs>,
) -> Result<Json<ApiResponse<AuthUrlResp>>, Error> {
    let auth_url = format!(
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid=YOUR_APPID&redirect_uri={}&response_type=code&scope={}#wechat_redirect",
        urlencoding::encode(&args.redirect_uri),
        &args.scope
    );

    Ok(Json(ApiResponse::success(AuthUrlResp { url: auth_url })))
}

pub async fn callback(
    Json(args): Json<WechatCallbackArgs>,
) -> Result<Json<ApiResponse<WechatCallbackResp>>, Error> {
    if args.code.is_none() {
        return Err(Error::bad_request("授权码不能为空"));
    }

    Ok(Json(ApiResponse::success(WechatCallbackResp {
        openid: "placeholder_openid".to_string(),
        unionid: None,
    })))
}

pub async fn mini_login(
    Json(_args): Json<MiniLoginArgs>,
) -> Result<Json<ApiResponse<MiniLoginResp>>, Error> {
    Ok(Json(ApiResponse::success(MiniLoginResp {
        openid: "placeholder_openid".to_string(),
        session_key: "placeholder_session_key".to_string(),
    })))
}

pub fn wechat_api() -> WebPath {
    WebPath::new()
        .route("/auth-url", WebPathType::Post, Some("获取微信授权URL"), post(get_auth_url))
        .route("/callback", WebPathType::Get, Some("微信授权回调"), get(callback))
        .route("/mini/login", WebPathType::Post, Some("小程序登录"), post(mini_login))
}