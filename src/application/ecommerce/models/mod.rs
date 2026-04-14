pub mod default_impl;

use std::error::Error;
use std::fmt;

// 电商平台错误定义
#[derive(Debug)]
pub enum EcommerceError {
    ApiError(String),
    NetworkError(String),
    AuthenticationError(String),
    ValidationError(String),
    DatabaseError(String),
    UnknownError(String),
}

impl fmt::Display for EcommerceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EcommerceError::ApiError(msg) => write!(f, "API Error: {}", msg),
            EcommerceError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            EcommerceError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            EcommerceError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            EcommerceError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            EcommerceError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl Error for EcommerceError {}

// 平台类型
pub enum PlatformType {
    Taobao,
    Pdd,
    Douyin,
    Xianyu,
    Amazon,
    Wechat,
}

impl PlatformType {
    pub fn as_str(&self) -> &str {
        match self {
            PlatformType::Taobao => "taobao",
            PlatformType::Pdd => "pdd",
            PlatformType::Douyin => "douyin",
            PlatformType::Xianyu => "xianyu",
            PlatformType::Amazon => "amazon",
            PlatformType::Wechat => "wechat",
        }
    }
}

// 订单状态
pub enum OrderStatus {
    PendingPayment,
    Paid,
    Shipped,
    Delivered,
    Completed,
    Canceled,
    Refunded,
}

// 售后类型
pub enum AfterSaleType {
    Refund,
    Return,
    Exchange,
}

// 售后状态
pub enum AfterSaleStatus {
    Pending,
    Processing,
    Approved,
    Rejected,
    Completed,
}

// 售后操作
pub enum AfterSaleAction {
    Approve,
    Reject,
    Refund,
    ShipBack,
}

// 商品结构体
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub platform_product_id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub status: String,
    pub images: Vec<String>,
    pub skus: Vec<Sku>,
    pub created_at: String,
    pub updated_at: String,
}

// SKU结构体
#[derive(Debug, Clone)]
pub struct Sku {
    pub id: String,
    pub sku_id: String,
    pub attributes: Vec<String>,
    pub price: f64,
    pub stock: i32,
}

// 订单结构体
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub platform_order_id: String,
    pub order_status: OrderStatus,
    pub total_amount: f64,
    pub buyer_info: BuyerInfo,
    pub shipping_info: ShippingInfo,
    pub items: Vec<OrderItem>,
    pub created_at: String,
    pub updated_at: String,
}

// 买家信息
#[derive(Debug, Clone)]
pub struct BuyerInfo {
    pub buyer_id: String,
    pub buyer_name: String,
    pub buyer_phone: String,
}

// 物流信息
#[derive(Debug, Clone)]
pub struct ShippingInfo {
    pub receiver_name: String,
    pub receiver_phone: String,
    pub receiver_address: String,
    pub logistics_company: String,
    pub tracking_number: String,
}

// 订单项
#[derive(Debug, Clone)]
pub struct OrderItem {
    pub id: String,
    pub product_id: String,
    pub sku_id: String,
    pub product_name: String,
    pub sku_attributes: Vec<String>,
    pub quantity: i32,
    pub price: f64,
}

// 售后结构体
#[derive(Debug, Clone)]
pub struct AfterSale {
    pub id: String,
    pub platform_after_sale_id: String,
    pub order_id: String,
    pub after_sale_type: AfterSaleType,
    pub after_sale_status: AfterSaleStatus,
    pub amount: f64,
    pub reason: String,
    pub images: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

// 推广活动结构体
#[derive(Debug, Clone)]
pub struct Promotion {
    pub id: String,
    pub platform_promotion_id: String,
    pub name: String,
    pub promotion_type: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

// 推广效果结构体
#[derive(Debug, Clone)]
pub struct PromotionEffect {
    pub promotion_id: String,
    pub click_count: i32,
    pub order_count: i32,
    pub sales_amount: f64,
    pub start_date: String,
    pub end_date: String,
}

// 获取商品参数
#[derive(Debug)]
pub struct GetProductsParams {
    pub page: i32,
    pub page_size: i32,
    pub status: Option<String>,
    pub keyword: Option<String>,
}

// 创建商品参数
#[derive(Debug)]
pub struct CreateProductParams {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub images: Vec<String>,
    pub skus: Vec<CreateSkuParams>,
    pub category_id: String,
}

// 创建SKU参数
#[derive(Debug)]
pub struct CreateSkuParams {
    pub attributes: Vec<String>,
    pub price: f64,
    pub stock: i32,
}

// 更新商品参数
#[derive(Debug)]
pub struct UpdateProductParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub images: Option<Vec<String>>,
    pub status: Option<String>,
}

// 获取订单参数
#[derive(Debug)]
pub struct GetOrdersParams {
    pub page: i32,
    pub page_size: i32,
    pub order_status: Option<OrderStatus>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// 获取售后参数
#[derive(Debug)]
pub struct GetAfterSalesParams {
    pub page: i32,
    pub page_size: i32,
    pub after_sale_status: Option<AfterSaleStatus>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// 获取推广活动参数
#[derive(Debug)]
pub struct GetPromotionsParams {
    pub page: i32,
    pub page_size: i32,
    pub status: Option<String>,
}

// 平台接口
pub trait EcommercePlatform {
    // 获取平台名称
    fn platform_name(&self) -> &str;
    
    // 测试连接
    fn test_connection(&self) -> Result<(), EcommerceError>;
    
    // 商品相关操作
    fn product_service(&self) -> Box<dyn ProductService>;
    
    // 订单相关操作
    fn order_service(&self) -> Box<dyn OrderService>;
    
    // 售后相关操作
    fn after_sale_service(&self) -> Box<dyn AfterSaleService>;
    
    // 推广相关操作
    fn promotion_service(&self) -> Box<dyn PromotionService>;
    
    // 库存相关操作
    fn inventory_service(&self) -> Box<dyn InventoryService>;
}

// 商品服务接口
pub trait ProductService {
    // 获取商品列表
    fn get_products(&self, params: GetProductsParams) -> Result<Vec<Product>, EcommerceError>;
    
    // 获取商品详情
    fn get_product(&self, product_id: &str) -> Result<Product, EcommerceError>;
    
    // 创建商品
    fn create_product(&self, product: CreateProductParams) -> Result<Product, EcommerceError>;
    
    // 更新商品
    fn update_product(&self, product_id: &str, product: UpdateProductParams) -> Result<Product, EcommerceError>;
    
    // 下架商品
    fn offline_product(&self, product_id: &str) -> Result<(), EcommerceError>;
}

// 订单服务接口
pub trait OrderService {
    // 获取订单列表
    fn get_orders(&self, params: GetOrdersParams) -> Result<Vec<Order>, EcommerceError>;
    
    // 获取订单详情
    fn get_order(&self, order_id: &str) -> Result<Order, EcommerceError>;
    
    // 更新订单状态
    fn update_order_status(&self, order_id: &str, status: OrderStatus) -> Result<Order, EcommerceError>;
    
    // 发货
    fn ship_order(&self, order_id: &str, shipping_info: ShippingInfo) -> Result<Order, EcommerceError>;
}

// 售后服务接口
pub trait AfterSaleService {
    // 获取售后列表
    fn get_after_sales(&self, params: GetAfterSalesParams) -> Result<Vec<AfterSale>, EcommerceError>;
    
    // 获取售后详情
    fn get_after_sale(&self, after_sale_id: &str) -> Result<AfterSale, EcommerceError>;
    
    // 处理售后申请
    fn handle_after_sale(&self, after_sale_id: &str, action: AfterSaleAction) -> Result<AfterSale, EcommerceError>;
}

// 推广服务接口
pub trait PromotionService {
    // 创建推广活动
    fn create_promotion(&self, promotion: CreatePromotionParams) -> Result<Promotion, EcommerceError>;
    
    // 获取推广活动列表
    fn get_promotions(&self, params: GetPromotionsParams) -> Result<Vec<Promotion>, EcommerceError>;
    
    // 获取推广效果
    fn get_promotion_effect(&self, promotion_id: &str) -> Result<PromotionEffect, EcommerceError>;
}

// 创建推广活动参数
#[derive(Debug)]
pub struct CreatePromotionParams {
    pub name: String,
    pub promotion_type: String,
    pub start_time: String,
    pub end_time: String,
    pub products: Vec<String>,
    pub discount: f64,
}

// 库存服务接口
pub trait InventoryService {
    // 更新库存
    fn update_inventory(&self, product_id: &str, sku_id: Option<&str>, quantity: i32) -> Result<(), EcommerceError>;
    
    // 获取库存
    fn get_inventory(&self, product_id: &str, sku_id: Option<&str>) -> Result<i32, EcommerceError>;
}
