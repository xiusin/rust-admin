use crate::application::ecommerce::models::{EcommerceError, OrderStatus, AfterSaleType, AfterSaleStatus, AfterSaleAction, Product, Order, AfterSale, Promotion};
use serde_json::Value;

// 归一化处理模块
pub struct NormalizationService;

impl NormalizationService {
    pub fn new() -> Self {
        Self
    }

    // 商品数据转换
    pub fn normalize_product(&self, platform: &str, data: Value) -> Result<Product, EcommerceError> {
        match platform {
            "taobao" => self.normalize_taobao_product(data),
            "pdd" => self.normalize_pdd_product(data),
            "douyin" => self.normalize_douyin_product(data),
            "amazon" => self.normalize_amazon_product(data),
            "wechat" => self.normalize_wechat_product(data),
            "xianyu" => self.normalize_xianyu_product(data),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 订单数据转换
    pub fn normalize_order(&self, platform: &str, data: Value) -> Result<Order, EcommerceError> {
        match platform {
            "taobao" => self.normalize_taobao_order(data),
            "pdd" => self.normalize_pdd_order(data),
            "douyin" => self.normalize_douyin_order(data),
            "amazon" => self.normalize_amazon_order(data),
            "wechat" => self.normalize_wechat_order(data),
            "xianyu" => self.normalize_xianyu_order(data),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 售后数据转换
    pub fn normalize_after_sale(&self, platform: &str, data: Value) -> Result<AfterSale, EcommerceError> {
        match platform {
            "taobao" => self.normalize_taobao_after_sale(data),
            "pdd" => self.normalize_pdd_after_sale(data),
            "douyin" => self.normalize_douyin_after_sale(data),
            "amazon" => self.normalize_amazon_after_sale(data),
            "wechat" => self.normalize_wechat_after_sale(data),
            "xianyu" => self.normalize_xianyu_after_sale(data),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 推广数据转换
    pub fn normalize_promotion(&self, platform: &str, data: Value) -> Result<Promotion, EcommerceError> {
        match platform {
            "taobao" => self.normalize_taobao_promotion(data),
            "pdd" => self.normalize_pdd_promotion(data),
            "douyin" => self.normalize_douyin_promotion(data),
            "amazon" => self.normalize_amazon_promotion(data),
            "wechat" => self.normalize_wechat_promotion(data),
            "xianyu" => self.normalize_xianyu_promotion(data),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 订单状态映射
    pub fn map_order_status(&self, platform: &str, platform_status: &str) -> Result<OrderStatus, EcommerceError> {
        match platform {
            "taobao" => self.map_taobao_order_status(platform_status),
            "pdd" => self.map_pdd_order_status(platform_status),
            "douyin" => self.map_douyin_order_status(platform_status),
            "amazon" => self.map_amazon_order_status(platform_status),
            "wechat" => self.map_wechat_order_status(platform_status),
            "xianyu" => self.map_xianyu_order_status(platform_status),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 售后状态映射
    pub fn map_after_sale_status(&self, platform: &str, platform_status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match platform {
            "taobao" => self.map_taobao_after_sale_status(platform_status),
            "pdd" => self.map_pdd_after_sale_status(platform_status),
            "douyin" => self.map_douyin_after_sale_status(platform_status),
            "amazon" => self.map_amazon_after_sale_status(platform_status),
            "wechat" => self.map_wechat_after_sale_status(platform_status),
            "xianyu" => self.map_xianyu_after_sale_status(platform_status),
            _ => Err(EcommerceError::ValidationError(format!("Unsupported platform: {}", platform))),
        }
    }

    // 错误映射
    pub fn map_error(&self, platform: &str, error_code: &str, error_msg: &str) -> EcommerceError {
        match platform {
            "taobao" => self.map_taobao_error(error_code, error_msg),
            "pdd" => self.map_pdd_error(error_code, error_msg),
            "douyin" => self.map_douyin_error(error_code, error_msg),
            "amazon" => self.map_amazon_error(error_code, error_msg),
            "wechat" => self.map_wechat_error(error_code, error_msg),
            "xianyu" => self.map_xianyu_error(error_code, error_msg),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 淘宝商品数据转换
    fn normalize_taobao_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["num_iid"].as_str().unwrap_or_default().to_string();
        let name = data["title"].as_str().unwrap_or_default().to_string();
        let description = data["desc"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0);
        let stock = data["num"].as_i64().unwrap_or(0) as i32;
        let status = if data["approve_status"].as_str().unwrap_or("") == "onsale" { "active" } else { "inactive" };
        
        let images: Vec<String> = data["pic_urls"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        let skus: Vec<Sku> = data["skus"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|sku| {
                let sku_id = sku["sku_id"].as_str().unwrap_or_default().to_string();
                let attributes: Vec<String> = sku["properties"].as_str().unwrap_or("")
                    .split(";")
                    .map(|s| s.to_string())
                    .collect();
                let sku_price = sku["price"].as_f64().unwrap_or(0.0);
                let sku_stock = sku["quantity"].as_i64().unwrap_or(0) as i32;
                
                Sku {
                    id: format!("{}_{}", product_id, sku_id),
                    sku_id,
                    attributes,
                    price: sku_price,
                    stock: sku_stock,
                }
            })
            .collect();
        
        Ok(Product {
            id: format!("taobao_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 拼多多商品数据转换
    fn normalize_pdd_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["goods_id"].as_i64().unwrap_or(0).to_string();
        let name = data["goods_name"].as_str().unwrap_or_default().to_string();
        let description = data["goods_desc"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0) / 100.0; // 拼多多价格单位是分
        let stock = data["stock"].as_i64().unwrap_or(0) as i32;
        let status = if data["is_onsale"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        let images: Vec<String> = data["image_urls"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        let skus: Vec<Sku> = data["skus"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|sku| {
                let sku_id = sku["sku_id"].as_i64().unwrap_or(0).to_string();
                let attributes: Vec<String> = sku["spec"].as_str().unwrap_or("")
                    .split("|")
                    .map(|s| s.to_string())
                    .collect();
                let sku_price = sku["price"].as_f64().unwrap_or(0.0) / 100.0;
                let sku_stock = sku["stock"].as_i64().unwrap_or(0) as i32;
                
                Sku {
                    id: format!("{}_{}", product_id, sku_id),
                    sku_id,
                    attributes,
                    price: sku_price,
                    stock: sku_stock,
                }
            })
            .collect();
        
        Ok(Product {
            id: format!("pdd_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["created_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["updated_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 抖音商品数据转换
    fn normalize_douyin_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["product_id"].as_str().unwrap_or_default().to_string();
        let name = data["product_name"].as_str().unwrap_or_default().to_string();
        let description = data["description"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0);
        let stock = data["stock"].as_i64().unwrap_or(0) as i32;
        let status = if data["status"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        let images: Vec<String> = data["image_list"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        let skus: Vec<Sku> = data["sku_list"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|sku| {
                let sku_id = sku["sku_id"].as_str().unwrap_or_default().to_string();
                let attributes: Vec<String> = sku["sku_attr"].as_str().unwrap_or("")
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                let sku_price = sku["price"].as_f64().unwrap_or(0.0);
                let sku_stock = sku["stock"].as_i64().unwrap_or(0) as i32;
                
                Sku {
                    id: format!("{}_{}", product_id, sku_id),
                    sku_id,
                    attributes,
                    price: sku_price,
                    stock: sku_stock,
                }
            })
            .collect();
        
        Ok(Product {
            id: format!("douyin_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 亚马逊商品数据转换
    fn normalize_amazon_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["ASIN"].as_str().unwrap_or_default().to_string();
        let name = data["title"].as_str().unwrap_or_default().to_string();
        let description = data["description"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0);
        let stock = data["quantity"].as_i64().unwrap_or(0) as i32;
        let status = if data["status"].as_str().unwrap_or("") == "ACTIVE" { "active" } else { "inactive" };
        
        let images: Vec<String> = data["images"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        let skus: Vec<Sku> = data["variations"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|sku| {
                let sku_id = sku["SKU"].as_str().unwrap_or_default().to_string();
                let attributes: Vec<String> = sku["attributes"].as_object().unwrap_or(&serde_json::Map::new())
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.as_str().unwrap_or("")))
                    .collect();
                let sku_price = sku["price"].as_f64().unwrap_or(0.0);
                let sku_stock = sku["quantity"].as_i64().unwrap_or(0) as i32;
                
                Sku {
                    id: format!("{}_{}", product_id, sku_id),
                    sku_id,
                    attributes,
                    price: sku_price,
                    stock: sku_stock,
                }
            })
            .collect();
        
        Ok(Product {
            id: format!("amazon_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["created_at"].as_str().unwrap_or_default().to_string(),
            updated_at: data["updated_at"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 微信商品数据转换
    fn normalize_wechat_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["product_id"].as_str().unwrap_or_default().to_string();
        let name = data["product_name"].as_str().unwrap_or_default().to_string();
        let description = data["product_desc"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0);
        let stock = data["stock"].as_i64().unwrap_or(0) as i32;
        let status = if data["status"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        let images: Vec<String> = data["image_list"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        let skus: Vec<Sku> = data["sku_list"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|sku| {
                let sku_id = sku["sku_id"].as_str().unwrap_or_default().to_string();
                let attributes: Vec<String> = sku["sku_info"].as_str().unwrap_or("")
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                let sku_price = sku["price"].as_f64().unwrap_or(0.0);
                let sku_stock = sku["stock"].as_i64().unwrap_or(0) as i32;
                
                Sku {
                    id: format!("{}_{}", product_id, sku_id),
                    sku_id,
                    attributes,
                    price: sku_price,
                    stock: sku_stock,
                }
            })
            .collect();
        
        Ok(Product {
            id: format!("wechat_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 闲鱼商品数据转换
    fn normalize_xianyu_product(&self, data: Value) -> Result<Product, EcommerceError> {
        let product_id = data["item_id"].as_str().unwrap_or_default().to_string();
        let name = data["title"].as_str().unwrap_or_default().to_string();
        let description = data["desc"].as_str().unwrap_or_default().to_string();
        let price = data["price"].as_f64().unwrap_or(0.0);
        let stock = data["num"].as_i64().unwrap_or(0) as i32;
        let status = if data["status"].as_str().unwrap_or("") == "onsale" { "active" } else { "inactive" };
        
        let images: Vec<String> = data["pic_urls"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        // 闲鱼通常没有SKU，所以创建一个默认SKU
        let skus = vec![Sku {
            id: format!("{}_{}", product_id, "default"),
            sku_id: "default".to_string(),
            attributes: vec!["默认".to_string()],
            price,
            stock,
        }];
        
        Ok(Product {
            id: format!("xianyu_{}", product_id),
            platform_product_id: product_id,
            name,
            description,
            price,
            stock,
            status: status.to_string(),
            images,
            skus,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 淘宝订单数据转换
    fn normalize_taobao_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["trade_id"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_taobao_order_status(data["status"].as_str().unwrap_or(""))?;
        let total_amount = data["total_fee"].as_f64().unwrap_or(0.0);
        
        let buyer_info = BuyerInfo {
            buyer_id: data["buyer_id"].as_str().unwrap_or_default().to_string(),
            buyer_name: data["buyer_nick"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["buyer_phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["receiver_name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["receiver_state"].as_str().unwrap_or(""),
                data["receiver_city"].as_str().unwrap_or(""),
                data["receiver_district"].as_str().unwrap_or(""),
                data["receiver_address"].as_str().unwrap_or("")
            ),
            logistics_company: data["logistics_company"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["tracking_number"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["orders"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["order_id"].as_str().unwrap_or_default().to_string();
                let product_id = item["num_iid"].as_str().unwrap_or_default().to_string();
                let sku_id = item["sku_id"].as_str().unwrap_or_default().to_string();
                let product_name = item["title"].as_str().unwrap_or_default().to_string();
                let sku_attributes = item["sku_properties"].as_str().unwrap_or("")
                    .split(";")
                    .map(|s| s.to_string())
                    .collect();
                let quantity = item["num"].as_i64().unwrap_or(0) as i32;
                let price = item["price"].as_f64().unwrap_or(0.0);
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("taobao_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 拼多多订单数据转换
    fn normalize_pdd_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["order_sn"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_pdd_order_status(data["order_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let total_amount = data["order_amount"].as_f64().unwrap_or(0.0) / 100.0; // 拼多多金额单位是分
        
        let buyer_info = BuyerInfo {
            buyer_id: data["buyer_id"].as_i64().unwrap_or(0).to_string(),
            buyer_name: data["buyer_nick"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["receiver_name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["province"].as_str().unwrap_or(""),
                data["city"].as_str().unwrap_or(""),
                data["district"].as_str().unwrap_or(""),
                data["address"].as_str().unwrap_or("")
            ),
            logistics_company: data["logistics_name"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["tracking_number"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["order_detail_list"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["order_detail_id"].as_i64().unwrap_or(0).to_string();
                let product_id = item["goods_id"].as_i64().unwrap_or(0).to_string();
                let sku_id = item["sku_id"].as_i64().unwrap_or(0).to_string();
                let product_name = item["goods_name"].as_str().unwrap_or_default().to_string();
                let sku_attributes = item["sku_info"].as_str().unwrap_or("")
                    .split("|")
                    .map(|s| s.to_string())
                    .collect();
                let quantity = item["goods_count"].as_i64().unwrap_or(0) as i32;
                let price = item["price"].as_f64().unwrap_or(0.0) / 100.0;
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("pdd_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 抖音订单数据转换
    fn normalize_douyin_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["order_id"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_douyin_order_status(data["order_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let total_amount = data["total_amount"].as_f64().unwrap_or(0.0);
        
        let buyer_info = BuyerInfo {
            buyer_id: data["buyer_id"].as_str().unwrap_or_default().to_string(),
            buyer_name: data["buyer_nickname"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["receiver_name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["province"].as_str().unwrap_or(""),
                data["city"].as_str().unwrap_or(""),
                data["district"].as_str().unwrap_or(""),
                data["detail_address"].as_str().unwrap_or("")
            ),
            logistics_company: data["logistics_company"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["tracking_number"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["order_items"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["item_id"].as_str().unwrap_or_default().to_string();
                let product_id = item["product_id"].as_str().unwrap_or_default().to_string();
                let sku_id = item["sku_id"].as_str().unwrap_or_default().to_string();
                let product_name = item["product_name"].as_str().unwrap_or_default().to_string();
                let sku_attributes = item["sku_attr"].as_str().unwrap_or("")
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                let quantity = item["quantity"].as_i64().unwrap_or(0) as i32;
                let price = item["price"].as_f64().unwrap_or(0.0);
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("douyin_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 亚马逊订单数据转换
    fn normalize_amazon_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["OrderId"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_amazon_order_status(data["OrderStatus"].as_str().unwrap_or(""))?;
        let total_amount = data["TotalAmount"].as_f64().unwrap_or(0.0);
        
        let buyer_info = BuyerInfo {
            buyer_id: data["BuyerId"].as_str().unwrap_or_default().to_string(),
            buyer_name: data["BuyerName"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["ShippingAddress"]["Phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["ShippingAddress"]["Name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["ShippingAddress"]["Phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["ShippingAddress"]["AddressLine1"].as_str().unwrap_or(""),
                data["ShippingAddress"]["AddressLine2"].as_str().unwrap_or(""),
                data["ShippingAddress"]["City"].as_str().unwrap_or(""),
                data["ShippingAddress"]["PostalCode"].as_str().unwrap_or("")
            ),
            logistics_company: data["Carrier"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["TrackingNumber"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["OrderItems"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["OrderItemId"].as_str().unwrap_or_default().to_string();
                let product_id = item["ASIN"].as_str().unwrap_or_default().to_string();
                let sku_id = item["SellerSKU"].as_str().unwrap_or_default().to_string();
                let product_name = item["Title"].as_str().unwrap_or_default().to_string();
                let sku_attributes = item["ItemAttributes"].as_object().unwrap_or(&serde_json::Map::new())
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.as_str().unwrap_or("")))
                    .collect();
                let quantity = item["QuantityOrdered"].as_i64().unwrap_or(0) as i32;
                let price = item["ItemPrice"]["Amount"].as_f64().unwrap_or(0.0);
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("amazon_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["PurchaseDate"].as_str().unwrap_or_default().to_string(),
            updated_at: data["LastUpdateDate"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 微信订单数据转换
    fn normalize_wechat_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["order_id"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_wechat_order_status(data["order_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let total_amount = data["total_amount"].as_f64().unwrap_or(0.0);
        
        let buyer_info = BuyerInfo {
            buyer_id: data["openid"].as_str().unwrap_or_default().to_string(),
            buyer_name: data["buyer_nickname"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["receiver_name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["province"].as_str().unwrap_or(""),
                data["city"].as_str().unwrap_or(""),
                data["district"].as_str().unwrap_or(""),
                data["detail_address"].as_str().unwrap_or("")
            ),
            logistics_company: data["logistics_company"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["tracking_number"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["order_items"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["item_id"].as_str().unwrap_or_default().to_string();
                let product_id = item["product_id"].as_str().unwrap_or_default().to_string();
                let sku_id = item["sku_id"].as_str().unwrap_or_default().to_string();
                let product_name = item["product_name"].as_str().unwrap_or_default().to_string();
                let sku_attributes = item["sku_info"].as_str().unwrap_or("")
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                let quantity = item["quantity"].as_i64().unwrap_or(0) as i32;
                let price = item["price"].as_f64().unwrap_or(0.0);
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("wechat_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 闲鱼订单数据转换
    fn normalize_xianyu_order(&self, data: Value) -> Result<Order, EcommerceError> {
        let order_id = data["trade_id"].as_str().unwrap_or_default().to_string();
        let platform_order_id = order_id.clone();
        let order_status = self.map_xianyu_order_status(data["status"].as_str().unwrap_or(""))?;
        let total_amount = data["total_fee"].as_f64().unwrap_or(0.0);
        
        let buyer_info = BuyerInfo {
            buyer_id: data["buyer_id"].as_str().unwrap_or_default().to_string(),
            buyer_name: data["buyer_nick"].as_str().unwrap_or_default().to_string(),
            buyer_phone: data["buyer_phone"].as_str().unwrap_or_default().to_string(),
        };
        
        let shipping_info = ShippingInfo {
            receiver_name: data["receiver_name"].as_str().unwrap_or_default().to_string(),
            receiver_phone: data["receiver_phone"].as_str().unwrap_or_default().to_string(),
            receiver_address: format!("{}{}{}{}", 
                data["receiver_state"].as_str().unwrap_or(""),
                data["receiver_city"].as_str().unwrap_or(""),
                data["receiver_district"].as_str().unwrap_or(""),
                data["receiver_address"].as_str().unwrap_or("")
            ),
            logistics_company: data["logistics_company"].as_str().unwrap_or_default().to_string(),
            tracking_number: data["tracking_number"].as_str().unwrap_or_default().to_string(),
        };
        
        let items: Vec<OrderItem> = data["orders"].as_array().unwrap_or(&vec![])
            .iter()
            .map(|item| {
                let item_id = item["order_id"].as_str().unwrap_or_default().to_string();
                let product_id = item["item_id"].as_str().unwrap_or_default().to_string();
                let sku_id = "default".to_string(); // 闲鱼通常没有SKU
                let product_name = item["title"].as_str().unwrap_or_default().to_string();
                let sku_attributes = vec!["默认".to_string()];
                let quantity = item["num"].as_i64().unwrap_or(0) as i32;
                let price = item["price"].as_f64().unwrap_or(0.0);
                
                OrderItem {
                    id: item_id,
                    product_id,
                    sku_id,
                    product_name,
                    sku_attributes,
                    quantity,
                    price,
                }
            })
            .collect();
        
        Ok(Order {
            id: format!("xianyu_{}", order_id),
            platform_order_id,
            order_status,
            total_amount,
            buyer_info,
            shipping_info,
            items,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 淘宝售后数据转换
    fn normalize_taobao_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["after_sale_id"].as_str().unwrap_or_default().to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["trade_id"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["after_sale_type"].as_str().unwrap_or("") {
            "REFUND" => AfterSaleType::Refund,
            "RETURN" => AfterSaleType::Return,
            "EXCHANGE" => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_taobao_after_sale_status(data["status"].as_str().unwrap_or(""))?;
        let amount = data["amount"].as_f64().unwrap_or(0.0);
        let reason = data["reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["images"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("taobao_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 拼多多售后数据转换
    fn normalize_pdd_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["after_sale_id"].as_i64().unwrap_or(0).to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["order_sn"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["after_sale_type"].as_i64().unwrap_or(0) {
            1 => AfterSaleType::Refund,
            2 => AfterSaleType::Return,
            3 => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_pdd_after_sale_status(data["after_sale_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let amount = data["refund_amount"].as_f64().unwrap_or(0.0) / 100.0; // 拼多多金额单位是分
        let reason = data["reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["evidence_pics"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("pdd_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["created_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["updated_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 抖音售后数据转换
    fn normalize_douyin_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["after_sale_id"].as_str().unwrap_or_default().to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["order_id"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["after_sale_type"].as_i64().unwrap_or(0) {
            1 => AfterSaleType::Refund,
            2 => AfterSaleType::Return,
            3 => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_douyin_after_sale_status(data["after_sale_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let amount = data["refund_amount"].as_f64().unwrap_or(0.0);
        let reason = data["reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["images"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("douyin_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 亚马逊售后数据转换
    fn normalize_amazon_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["RefundId"].as_str().unwrap_or_default().to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["OrderId"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["RefundType"].as_str().unwrap_or("") {
            "FullRefund" => AfterSaleType::Refund,
            "PartialRefund" => AfterSaleType::Refund,
            "Return" => AfterSaleType::Return,
            "Exchange" => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_amazon_after_sale_status(data["Status"].as_str().unwrap_or(""))?;
        let amount = data["RefundAmount"]["Amount"].as_f64().unwrap_or(0.0);
        let reason = data["Reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["Evidence"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("amazon_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["CreateDate"].as_str().unwrap_or_default().to_string(),
            updated_at: data["LastUpdateDate"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 微信售后数据转换
    fn normalize_wechat_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["after_sale_id"].as_str().unwrap_or_default().to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["order_id"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["after_sale_type"].as_i64().unwrap_or(0) {
            1 => AfterSaleType::Refund,
            2 => AfterSaleType::Return,
            3 => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_wechat_after_sale_status(data["after_sale_status"].as_i64().unwrap_or(0).to_string().as_str())?;
        let amount = data["refund_amount"].as_f64().unwrap_or(0.0);
        let reason = data["reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["images"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("wechat_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 闲鱼售后数据转换
    fn normalize_xianyu_after_sale(&self, data: Value) -> Result<AfterSale, EcommerceError> {
        let after_sale_id = data["after_sale_id"].as_str().unwrap_or_default().to_string();
        let platform_after_sale_id = after_sale_id.clone();
        let order_id = data["trade_id"].as_str().unwrap_or_default().to_string();
        let after_sale_type = match data["after_sale_type"].as_str().unwrap_or("") {
            "REFUND" => AfterSaleType::Refund,
            "RETURN" => AfterSaleType::Return,
            "EXCHANGE" => AfterSaleType::Exchange,
            _ => AfterSaleType::Refund,
        };
        let after_sale_status = self.map_xianyu_after_sale_status(data["status"].as_str().unwrap_or(""))?;
        let amount = data["amount"].as_f64().unwrap_or(0.0);
        let reason = data["reason"].as_str().unwrap_or_default().to_string();
        
        let images: Vec<String> = data["images"].as_array().unwrap_or(&vec![])
            .iter()
            .filter_map(|img| img.as_str().map(|s| s.to_string()))
            .collect();
        
        Ok(AfterSale {
            id: format!("xianyu_{}", after_sale_id),
            platform_after_sale_id,
            order_id,
            after_sale_type,
            after_sale_status,
            amount,
            reason,
            images,
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 淘宝推广数据转换
    fn normalize_taobao_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["activity_id"].as_str().unwrap_or_default().to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["activity_name"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["activity_type"].as_str().unwrap_or_default().to_string();
        let start_time = data["start_time"].as_str().unwrap_or_default().to_string();
        let end_time = data["end_time"].as_str().unwrap_or_default().to_string();
        let status = if data["status"].as_str().unwrap_or("") == "ACTIVE" { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("taobao_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 拼多多推广数据转换
    fn normalize_pdd_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["activity_id"].as_i64().unwrap_or(0).to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["activity_name"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["activity_type"].as_i64().unwrap_or(0).to_string();
        let start_time = data["start_time"].as_str().unwrap_or_default().to_string();
        let end_time = data["end_time"].as_str().unwrap_or_default().to_string();
        let status = if data["status"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("pdd_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["created_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["updated_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 抖音推广数据转换
    fn normalize_douyin_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["promotion_id"].as_str().unwrap_or_default().to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["promotion_name"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["promotion_type"].as_str().unwrap_or_default().to_string();
        let start_time = data["start_time"].as_str().unwrap_or_default().to_string();
        let end_time = data["end_time"].as_str().unwrap_or_default().to_string();
        let status = if data["status"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("douyin_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 亚马逊推广数据转换
    fn normalize_amazon_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["PromotionId"].as_str().unwrap_or_default().to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["PromotionName"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["PromotionType"].as_str().unwrap_or_default().to_string();
        let start_time = data["StartDate"].as_str().unwrap_or_default().to_string();
        let end_time = data["EndDate"].as_str().unwrap_or_default().to_string();
        let status = if data["Status"].as_str().unwrap_or("") == "ACTIVE" { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("amazon_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["CreationDate"].as_str().unwrap_or_default().to_string(),
            updated_at: data["LastUpdateDate"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 微信推广数据转换
    fn normalize_wechat_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["promotion_id"].as_str().unwrap_or_default().to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["promotion_name"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["promotion_type"].as_str().unwrap_or_default().to_string();
        let start_time = data["start_time"].as_str().unwrap_or_default().to_string();
        let end_time = data["end_time"].as_str().unwrap_or_default().to_string();
        let status = if data["status"].as_i64().unwrap_or(0) == 1 { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("wechat_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["create_time"].as_str().unwrap_or_default().to_string(),
            updated_at: data["update_time"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 闲鱼推广数据转换
    fn normalize_xianyu_promotion(&self, data: Value) -> Result<Promotion, EcommerceError> {
        let promotion_id = data["activity_id"].as_str().unwrap_or_default().to_string();
        let platform_promotion_id = promotion_id.clone();
        let name = data["activity_name"].as_str().unwrap_or_default().to_string();
        let promotion_type = data["activity_type"].as_str().unwrap_or_default().to_string();
        let start_time = data["start_time"].as_str().unwrap_or_default().to_string();
        let end_time = data["end_time"].as_str().unwrap_or_default().to_string();
        let status = if data["status"].as_str().unwrap_or("") == "ACTIVE" { "active" } else { "inactive" };
        
        Ok(Promotion {
            id: format!("xianyu_{}", promotion_id),
            platform_promotion_id,
            name,
            promotion_type,
            start_time,
            end_time,
            status: status.to_string(),
            created_at: data["created"].as_str().unwrap_or_default().to_string(),
            updated_at: data["modified"].as_str().unwrap_or_default().to_string(),
        })
    }

    // 淘宝订单状态映射
    fn map_taobao_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "WAIT_BUYER_PAY" => Ok(OrderStatus::PendingPayment),
            "WAIT_SELLER_SEND_GOODS" => Ok(OrderStatus::Paid),
            "WAIT_BUYER_CONFIRM_GOODS" => Ok(OrderStatus::Shipped),
            "TRADE_FINISHED" => Ok(OrderStatus::Completed),
            "TRADE_CLOSED" => Ok(OrderStatus::Canceled),
            "TRADE_REFUND" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown taobao order status: {}", status))),
        }
    }

    // 拼多多订单状态映射
    fn map_pdd_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "1" => Ok(OrderStatus::PendingPayment),
            "2" => Ok(OrderStatus::Paid),
            "3" => Ok(OrderStatus::Shipped),
            "4" => Ok(OrderStatus::Delivered),
            "5" => Ok(OrderStatus::Completed),
            "6" => Ok(OrderStatus::Canceled),
            "7" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown pdd order status: {}", status))),
        }
    }

    // 抖音订单状态映射
    fn map_douyin_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "1" => Ok(OrderStatus::PendingPayment),
            "2" => Ok(OrderStatus::Paid),
            "3" => Ok(OrderStatus::Shipped),
            "4" => Ok(OrderStatus::Delivered),
            "5" => Ok(OrderStatus::Completed),
            "6" => Ok(OrderStatus::Canceled),
            "7" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown douyin order status: {}", status))),
        }
    }

    // 亚马逊订单状态映射
    fn map_amazon_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "Pending" => Ok(OrderStatus::PendingPayment),
            "Unshipped" => Ok(OrderStatus::Paid),
            "Shipped" => Ok(OrderStatus::Shipped),
            "Delivered" => Ok(OrderStatus::Delivered),
            "Canceled" => Ok(OrderStatus::Canceled),
            "Refunded" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown amazon order status: {}", status))),
        }
    }

    // 微信订单状态映射
    fn map_wechat_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "1" => Ok(OrderStatus::PendingPayment),
            "2" => Ok(OrderStatus::Paid),
            "3" => Ok(OrderStatus::Shipped),
            "4" => Ok(OrderStatus::Delivered),
            "5" => Ok(OrderStatus::Completed),
            "6" => Ok(OrderStatus::Canceled),
            "7" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown wechat order status: {}", status))),
        }
    }

    // 闲鱼订单状态映射
    fn map_xianyu_order_status(&self, status: &str) -> Result<OrderStatus, EcommerceError> {
        match status {
            "WAIT_BUYER_PAY" => Ok(OrderStatus::PendingPayment),
            "WAIT_SELLER_SEND_GOODS" => Ok(OrderStatus::Paid),
            "WAIT_BUYER_CONFIRM_GOODS" => Ok(OrderStatus::Shipped),
            "TRADE_FINISHED" => Ok(OrderStatus::Completed),
            "TRADE_CLOSED" => Ok(OrderStatus::Canceled),
            "TRADE_REFUND" => Ok(OrderStatus::Refunded),
            _ => Err(EcommerceError::ValidationError(format!("Unknown xianyu order status: {}", status))),
        }
    }

    // 淘宝售后状态映射
    fn map_taobao_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "WAIT_SELLER_AGREE" => Ok(AfterSaleStatus::Pending),
            "SELLER_AGREE" => Ok(AfterSaleStatus::Processing),
            "WAIT_BUYER_SEND_GOODS" => Ok(AfterSaleStatus::Processing),
            "WAIT_SELLER_CONFIRM_GOODS" => Ok(AfterSaleStatus::Processing),
            "SUCCESS" => Ok(AfterSaleStatus::Completed),
            "CLOSED" => Ok(AfterSaleStatus::Rejected),
            _ => Err(EcommerceError::ValidationError(format!("Unknown taobao after sale status: {}", status))),
        }
    }

    // 拼多多售后状态映射
    fn map_pdd_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "1" => Ok(AfterSaleStatus::Pending),
            "2" => Ok(AfterSaleStatus::Processing),
            "3" => Ok(AfterSaleStatus::Approved),
            "4" => Ok(AfterSaleStatus::Rejected),
            "5" => Ok(AfterSaleStatus::Completed),
            _ => Err(EcommerceError::ValidationError(format!("Unknown pdd after sale status: {}", status))),
        }
    }

    // 抖音售后状态映射
    fn map_douyin_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "1" => Ok(AfterSaleStatus::Pending),
            "2" => Ok(AfterSaleStatus::Processing),
            "3" => Ok(AfterSaleStatus::Approved),
            "4" => Ok(AfterSaleStatus::Rejected),
            "5" => Ok(AfterSaleStatus::Completed),
            _ => Err(EcommerceError::ValidationError(format!("Unknown douyin after sale status: {}", status))),
        }
    }

    // 亚马逊售后状态映射
    fn map_amazon_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "Pending" => Ok(AfterSaleStatus::Pending),
            "Processing" => Ok(AfterSaleStatus::Processing),
            "Approved" => Ok(AfterSaleStatus::Approved),
            "Rejected" => Ok(AfterSaleStatus::Rejected),
            "Completed" => Ok(AfterSaleStatus::Completed),
            _ => Err(EcommerceError::ValidationError(format!("Unknown amazon after sale status: {}", status))),
        }
    }

    // 微信售后状态映射
    fn map_wechat_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "1" => Ok(AfterSaleStatus::Pending),
            "2" => Ok(AfterSaleStatus::Processing),
            "3" => Ok(AfterSaleStatus::Approved),
            "4" => Ok(AfterSaleStatus::Rejected),
            "5" => Ok(AfterSaleStatus::Completed),
            _ => Err(EcommerceError::ValidationError(format!("Unknown wechat after sale status: {}", status))),
        }
    }

    // 闲鱼售后状态映射
    fn map_xianyu_after_sale_status(&self, status: &str) -> Result<AfterSaleStatus, EcommerceError> {
        match status {
            "WAIT_SELLER_AGREE" => Ok(AfterSaleStatus::Pending),
            "SELLER_AGREE" => Ok(AfterSaleStatus::Processing),
            "WAIT_BUYER_SEND_GOODS" => Ok(AfterSaleStatus::Processing),
            "WAIT_SELLER_CONFIRM_GOODS" => Ok(AfterSaleStatus::Processing),
            "SUCCESS" => Ok(AfterSaleStatus::Completed),
            "CLOSED" => Ok(AfterSaleStatus::Rejected),
            _ => Err(EcommerceError::ValidationError(format!("Unknown xianyu after sale status: {}", status))),
        }
    }

    // 淘宝错误映射
    fn map_taobao_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "10000" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "1101" => EcommerceError::ValidationError(error_msg.to_string()),
            "2500" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 拼多多错误映射
    fn map_pdd_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "10001" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "10002" => EcommerceError::ValidationError(error_msg.to_string()),
            "10003" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 抖音错误映射
    fn map_douyin_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "40001" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "40002" => EcommerceError::ValidationError(error_msg.to_string()),
            "40003" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 亚马逊错误映射
    fn map_amazon_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "Unauthorized" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "InvalidParameter" => EcommerceError::ValidationError(error_msg.to_string()),
            "InternalError" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 微信错误映射
    fn map_wechat_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "40001" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "40002" => EcommerceError::ValidationError(error_msg.to_string()),
            "40003" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }

    // 闲鱼错误映射
    fn map_xianyu_error(&self, error_code: &str, error_msg: &str) -> EcommerceError {
        match error_code {
            "10000" => EcommerceError::AuthenticationError(error_msg.to_string()),
            "1101" => EcommerceError::ValidationError(error_msg.to_string()),
            "2500" => EcommerceError::ApiError(error_msg.to_string()),
            _ => EcommerceError::ApiError(format!("{}: {}", error_code, error_msg)),
        }
    }
}
