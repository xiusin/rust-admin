use crate::application::ecommerce::models::EcommerceError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use md5::Md5;
use std::io::Write;
use std::fmt::Write as FmtWrite;

// 淘宝API客户端
#[derive(Clone)]
pub struct TaobaoApiClient {
    app_key: String,
    app_secret: String,
    access_token: Option<String>,
    client: Client,
}

impl TaobaoApiClient {
    pub fn new(app_key: String, app_secret: String, access_token: Option<String>) -> Self {
        Self {
            app_key,
            app_secret,
            access_token,
            client: Client::new(),
        }
    }

    // 生成签名
    fn generate_sign(&self, params: &HashMap<String, String>) -> String {
        let mut sorted_params: Vec<(&String, &String)> = params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        let mut sign_str = self.app_secret.clone();
        for (key, value) in sorted_params {
            sign_str.push_str(key);
            sign_str.push_str(value);
        }
        sign_str.push_str(&self.app_secret);

        let mut md5 = Md5::new();
        md5.write_all(sign_str.as_bytes()).unwrap();
        format!("{:x}", md5.finalize())
    }

    // 调用API
    pub async fn call_api(&self, method: &str, params: HashMap<String, String>) -> Result<Value, EcommerceError> {
        let mut request_params = HashMap::new();
        request_params.insert("app_key".to_string(), self.app_key.clone());
        request_params.insert("method".to_string(), method.to_string());
        request_params.insert("timestamp".to_string(), chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        request_params.insert("format".to_string(), "json".to_string());
        request_params.insert("v".to_string(), "2.0".to_string());
        request_params.insert("sign_method".to_string(), "md5".to_string());

        if let Some(access_token) = &self.access_token {
            request_params.insert("session".to_string(), access_token.clone());
        }

        // 添加业务参数
        for (key, value) in params {
            request_params.insert(key, value);
        }

        // 生成签名
        let sign = self.generate_sign(&request_params);
        request_params.insert("sign".to_string(), sign);

        // 构建请求URL
        let mut url = "https://eco.taobao.com/router/rest".to_string();
        let mut first = true;
        for (key, value) in request_params {
            if first {
                url.push_str("?");
                first = false;
            } else {
                url.push_str("&");
            }
            url.push_str(&urlencoding::encode(&key));
            url.push_str("=");
            url.push_str(&urlencoding::encode(&value));
        }

        // 发送请求
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| EcommerceError::NetworkError(e.to_string()))?;

        let response_text = response.text().await
            .map_err(|e| EcommerceError::NetworkError(e.to_string()))?;

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| EcommerceError::ApiError(format!("Invalid response: {}", e)))?;

        // 检查响应是否有错误
        if response_json.get("error_response").is_some() {
            let error = response_json["error_response"].clone();
            let error_code = error["code"].as_i64().unwrap_or(0);
            let error_msg = error["msg"].as_str().unwrap_or("Unknown error");
            return Err(EcommerceError::ApiError(format!("Error {}: {}", error_code, error_msg)));
        }

        Ok(response_json)
    }

    // 获取商品列表
    pub async fn get_products(&self, page: i32, page_size: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("page_no".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.call_api("taobao.items.onsale.get", params).await
    }

    // 获取商品详情
    pub async fn get_product(&self, product_id: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("num_iid".to_string(), product_id.to_string());
        self.call_api("taobao.item.get", params).await
    }

    // 获取订单列表
    pub async fn get_orders(&self, page: i32, page_size: i32, start_date: Option<String>, end_date: Option<String>) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("page_no".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        
        if let Some(start_date) = start_date {
            params.insert("start_created".to_string(), start_date);
        }
        if let Some(end_date) = end_date {
            params.insert("end_created".to_string(), end_date);
        }
        
        self.call_api("taobao.trades.sold.get", params).await
    }

    // 发货
    pub async fn ship_order(&self, order_id: &str, logistics_company: &str, tracking_number: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("tid".to_string(), order_id.to_string());
        params.insert("company_code".to_string(), logistics_company.to_string());
        params.insert("out_sid".to_string(), tracking_number.to_string());
        self.call_api("taobao.logistics.offline.send", params).await
    }

    // 获取售后列表
    pub async fn get_after_sales(&self, page: i32, page_size: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("page_no".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.call_api("taobao.refunds.get", params).await
    }

    // 处理售后
    pub async fn handle_after_sale(&self, refund_id: &str, action: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("refund_id".to_string(), refund_id.to_string());
        params.insert("operation".to_string(), action.to_string());
        self.call_api("taobao.refund.agree", params).await
    }

    // 更新库存
    pub async fn update_inventory(&self, product_id: &str, sku_id: Option<&str>, quantity: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("num_iid".to_string(), product_id.to_string());
        params.insert("quantity".to_string(), quantity.to_string());
        
        if let Some(sku_id) = sku_id {
            params.insert("sku_id".to_string(), sku_id.to_string());
        }
        
        self.call_api("taobao.item.quantity.update", params).await
    }
}
