use crate::application::ecommerce::models::EcommerceError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use md5::Md5;
use std::io::Write;

// 拼多多API客户端
#[derive(Clone)]
pub struct PddApiClient {
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    client: Client,
}

impl PddApiClient {
    pub fn new(client_id: String, client_secret: String, access_token: Option<String>) -> Self {
        Self {
            client_id,
            client_secret,
            access_token,
            client: Client::new(),
        }
    }

    // 生成签名
    fn generate_sign(&self, params: &HashMap<String, String>) -> String {
        let mut sorted_params: Vec<(&String, &String)> = params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        let mut sign_str = self.client_secret.clone();
        for (key, value) in sorted_params {
            sign_str.push_str(key);
            sign_str.push_str(value);
        }
        sign_str.push_str(&self.client_secret);

        let mut md5 = Md5::new();
        md5.write_all(sign_str.as_bytes()).unwrap();
        format!("{:x}", md5.finalize())
    }

    // 调用API
    pub async fn call_api(&self, method: &str, params: HashMap<String, String>) -> Result<Value, EcommerceError> {
        let mut request_params = HashMap::new();
        request_params.insert("client_id".to_string(), self.client_id.clone());
        request_params.insert("sign_method".to_string(), "md5".to_string());
        request_params.insert("timestamp".to_string(), chrono::Local::now().timestamp().to_string());

        if let Some(access_token) = &self.access_token {
            request_params.insert("access_token".to_string(), access_token.clone());
        }

        // 添加业务参数
        for (key, value) in params {
            request_params.insert(key, value);
        }

        // 生成签名
        let sign = self.generate_sign(&request_params);
        request_params.insert("sign".to_string(), sign);

        // 构建请求URL
        let url = format!("https://gw-api.pinduoduo.com/api/router");

        // 发送请求
        let response = self.client.post(&url)
            .form(&request_params)
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
            let error_code = error["error_code"].as_i64().unwrap_or(0);
            let error_msg = error["error_msg"].as_str().unwrap_or("Unknown error");
            return Err(EcommerceError::ApiError(format!("Error {}: {}", error_code, error_msg)));
        }

        Ok(response_json)
    }

    // 获取商品列表
    pub async fn get_products(&self, page: i32, page_size: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.goods.list.get".to_string());
        params.insert("page".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.call_api("pdd.goods.list.get", params).await
    }

    // 获取商品详情
    pub async fn get_product(&self, product_id: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.goods.info.get".to_string());
        params.insert("goods_id".to_string(), product_id.to_string());
        self.call_api("pdd.goods.info.get", params).await
    }

    // 获取订单列表
    pub async fn get_orders(&self, page: i32, page_size: i32, start_date: Option<String>, end_date: Option<String>) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.order.list.get".to_string());
        params.insert("page".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        
        if let Some(start_date) = start_date {
            params.insert("start_time".to_string(), start_date);
        }
        if let Some(end_date) = end_date {
            params.insert("end_time".to_string(), end_date);
        }
        
        self.call_api("pdd.order.list.get", params).await
    }

    // 发货
    pub async fn ship_order(&self, order_id: &str, logistics_company: &str, tracking_number: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.logistics.online.send".to_string());
        params.insert("order_sn".to_string(), order_id.to_string());
        params.insert("logistics_id".to_string(), logistics_company.to_string());
        params.insert("tracking_number".to_string(), tracking_number.to_string());
        self.call_api("pdd.logistics.online.send", params).await
    }

    // 获取售后列表
    pub async fn get_after_sales(&self, page: i32, page_size: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.refund.list.get".to_string());
        params.insert("page".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.call_api("pdd.refund.list.get", params).await
    }

    // 处理售后
    pub async fn handle_after_sale(&self, refund_id: &str, action: &str) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.refund.agree".to_string());
        params.insert("refund_id".to_string(), refund_id.to_string());
        self.call_api("pdd.refund.agree", params).await
    }

    // 更新库存
    pub async fn update_inventory(&self, product_id: &str, sku_id: Option<&str>, quantity: i32) -> Result<Value, EcommerceError> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), "pdd.goods.sku.stock.update".to_string());
        params.insert("goods_id".to_string(), product_id.to_string());
        
        if let Some(sku_id) = sku_id {
            params.insert("sku_id".to_string(), sku_id.to_string());
        }
        params.insert("stock_num".to_string(), quantity.to_string());
        
        self.call_api("pdd.goods.sku.stock.update", params).await
    }
}
