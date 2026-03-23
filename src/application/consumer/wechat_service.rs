use crate::common::error::Error;
use serde::{Deserialize, Serialize};

pub struct WechatService {
    app_id: String,
    app_secret: String,
}

impl WechatService {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self { app_id, app_secret }
    }

    pub async fn get_auth_url(&self, redirect_uri: &str, scope: &str) -> Result<String, Error> {
        let auth_url = format!(
            "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope={}#wechat_redirect",
            self.app_id,
            urlencoding::encode(redirect_uri),
            scope
        );
        Ok(auth_url)
    }

    pub async fn callback(&self, code: &str) -> Result<WechatAccessTokenResp, Error> {
        let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
            self.app_id, self.app_secret, code
        );

        let resp = reqwest::get(&url)
            .await
            .map_err(|e| Error::internal_error(format!("Request error: {}", e)))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Parse error: {}", e)))?;

        if let Some(errcode) = body.get("errcode").and_then(|v| v.as_i64()) {
            if errcode != 0 {
                let errmsg = body.get("errmsg").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                return Err(Error::bad_request(format!("Wechat error: {}", errmsg)));
            }
        }

        Ok(WechatAccessTokenResp {
            access_token: body["access_token"].as_str().unwrap_or("").to_string(),
            expires_in: body["expires_in"].as_i64().unwrap_or(0),
            refresh_token: body["refresh_token"].as_str().unwrap_or("").to_string(),
            openid: body["openid"].as_str().unwrap_or("").to_string(),
            scope: body["scope"].as_str().unwrap_or("").to_string(),
        })
    }

    pub async fn get_user_info(&self, access_token: &str, openid: &str) -> Result<WechatUserInfo, Error> {
        let url = format!(
            "https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}",
            access_token, openid
        );

        let resp = reqwest::get(&url)
            .await
            .map_err(|e| Error::internal_error(format!("Request error: {}", e)))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Parse error: {}", e)))?;

        if let Some(errcode) = body.get("errcode").and_then(|v| v.as_i64()) {
            if errcode != 0 {
                let errmsg = body.get("errmsg").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                return Err(Error::bad_request(format!("Wechat error: {}", errmsg)));
            }
        }

        Ok(WechatUserInfo {
            openid: body["openid"].as_str().unwrap_or("").to_string(),
            nickname: body["nickname"].as_str().unwrap_or("").to_string(),
            sex: body["sex"].as_i64().unwrap_or(0),
            province: body["province"].as_str().unwrap_or("").to_string(),
            city: body["city"].as_str().unwrap_or("").to_string(),
            country: body["country"].as_str().unwrap_or("").to_string(),
            headimgurl: body["headimgurl"].as_str().unwrap_or("").to_string(),
            unionid: body["unionid"].as_str().map(|s| s.to_string()),
        })
    }

    pub async fn mini_program_login(&self, code: &str) -> Result<MiniProgramSessionResp, Error> {
        let url = format!(
            "https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
            self.app_id, self.app_secret, code
        );

        let resp = reqwest::get(&url)
            .await
            .map_err(|e| Error::internal_error(format!("Request error: {}", e)))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| Error::internal_error(format!("Parse error: {}", e)))?;

        if let Some(errcode) = body.get("errcode").and_then(|v| v.as_i64()) {
            if errcode != 0 {
                let errmsg = body.get("errmsg").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                return Err(Error::bad_request(format!("Wechat error: {}", errmsg)));
            }
        }

        Ok(MiniProgramSessionResp {
            openid: body["openid"].as_str().unwrap_or("").to_string(),
            session_key: body["session_key"].as_str().unwrap_or("").to_string(),
            unionid: body["unionid"].as_str().map(|s| s.to_string()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatAccessTokenResp {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatUserInfo {
    pub openid: String,
    pub nickname: String,
    pub sex: i64,
    pub province: String,
    pub city: String,
    pub country: String,
    pub headimgurl: String,
    pub unionid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniProgramSessionResp {
    pub openid: String,
    pub session_key: String,
    pub unionid: Option<String>,
}