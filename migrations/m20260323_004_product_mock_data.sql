-- m20260323_004_product_mock_data.sql
-- 描述：商品管理模块测试数据
-- 作者：tuoke
-- 日期：2026-03-23

-- =====================================================
-- 1. 商品分类测试数据
-- =====================================================
INSERT INTO `p_category` (`id`, `parent_id`, `name`, `icon`, `image`, `sort`, `level`, `path`, `status`, `show_in_nav`, `created_at`, `updated_at`) VALUES
(1, 0, '服装鞋帽', 'clothes', NULL, 1, 1, '1', '0', 1, NOW(), NOW()),
(2, 0, '数码电器', 'phone', NULL, 2, 1, '2', '0', 1, NOW(), NOW()),
(3, 0, '食品生鲜', 'food', NULL, 3, 1, '3', '0', 1, NOW(), NOW()),
(4, 0, '美妆个护', 'beauty', NULL, 4, 1, '4', '0', 1, NOW(), NOW()),
(5, 0, '家居家装', 'home', NULL, 5, 1, '5', '0', 0, NOW(), NOW()),
(101, 1, '男装', NULL, NULL, 1, 2, '1,101', '0', 0, NOW(), NOW()),
(102, 1, '女装', NULL, NULL, 2, 2, '1,102', '0', 0, NOW(), NOW()),
(103, 1, '男鞋', NULL, NULL, 3, 2, '1,103', '0', 0, NOW(), NOW()),
(104, 1, '女鞋', NULL, NULL, 4, 2, '1,104', '0', 0, NOW(), NOW()),
(201, 2, '手机通讯', NULL, NULL, 1, 2, '2,201', '0', 0, NOW(), NOW()),
(202, 2, '电脑办公', NULL, NULL, 2, 2, '2,202', '0', 0, NOW(), NOW()),
(203, 2, '家用电器', NULL, NULL, 3, 2, '2,203', '0', 0, NOW(), NOW()),
(204, 2, '数码配件', NULL, NULL, 4, 2, '2,204', '0', 0, NOW(), NOW()),
(301, 3, '生鲜水果', NULL, NULL, 1, 2, '3,301', '0', 0, NOW(), NOW()),
(302, 3, '休闲零食', NULL, NULL, 2, 2, '3,302', '0', 0, NOW(), NOW()),
(401, 4, '面部护肤', NULL, NULL, 1, 2, '4,401', '0', 0, NOW(), NOW()),
(402, 4, '彩妆香水', NULL, NULL, 2, 2, '4,402', '0', 0, NOW(), NOW()),
(10101, 101, 'T恤', NULL, NULL, 1, 3, '1,101,10101', '0', 0, NOW(), NOW()),
(10102, 101, '衬衫', NULL, NULL, 2, 3, '1,101,10102', '0', 0, NOW(), NOW()),
(10103, 101, '夹克', NULL, NULL, 3, 3, '1,101,10103', '0', 0, NOW(), NOW()),
(10201, 102, '连衣裙', NULL, NULL, 1, 3, '1,102,10201', '0', 0, NOW(), NOW()),
(10202, 102, '半身裙', NULL, NULL, 2, 3, '1,102,10202', '0', 0, NOW(), NOW()),
(20101, 201, '智能手机', NULL, NULL, 1, 3, '2,201,20101', '0', 0, NOW(), NOW()),
(20102, 201, '老人机', NULL, NULL, 2, 3, '2,201,20102', '0', 0, NOW(), NOW()),
(20201, 202, '笔记本', NULL, NULL, 1, 3, '2,202,20201', '0', 0, NOW(), NOW()),
(20202, 202, '台式机', NULL, NULL, 2, 3, '2,202,20202', '0', 0, NOW(), NOW());

-- =====================================================
-- 2. 商品品牌测试数据
-- =====================================================
INSERT INTO `p_brand` (`id`, `name`, `logo`, `description`, `sort`, `status`, `created_at`, `updated_at`) VALUES
(1, 'Apple', 'https://example.com/brand/apple.png', '苹果公司', 1, '0', NOW(), NOW()),
(2, '华为', 'https://example.com/brand/huawei.png', '华为技术有限公司', 2, '0', NOW(), NOW()),
(3, '小米', 'https://example.com/brand/xiaomi.png', '小米科技有限责任公司', 3, '0', NOW(), NOW()),
(4, 'OPPO', 'https://example.com/brand/oppo.png', 'OPPO广东移动通信有限公司', 4, '0', NOW(), NOW()),
(5, 'vivo', 'https://example.com/brand/vivo.png', 'vivo移动通信有限公司', 5, '0', NOW(), NOW()),
(6, 'Nike', 'https://example.com/brand/nike.png', '耐克公司', 6, '0', NOW(), NOW()),
(7, 'Adidas', 'https://example.com/brand/adidas.png', '阿迪达斯', 7, '0', NOW(), NOW()),
(8, 'UNIQLO', 'https://example.com/brand/uniqlo.png', '优衣库', 8, '0', NOW(), NOW()),
(9, 'ZARA', 'https://example.com/brand/zara.png', 'ZARA', 9, '0', NOW(), NOW()),
(10, 'H&M', 'https://example.com/brand/hm.png', 'H&M', 10, '0', NOW(), NOW());

-- =====================================================
-- 3. 商品分组测试数据
-- =====================================================
INSERT INTO `p_product_group` (`id`, `name`, `description`, `sort`, `status`, `created_at`, `updated_at`) VALUES
(1, '热销推荐', '热销商品推荐', 1, '0', NOW(), NOW()),
(2, '新品上市', '最新上架商品', 2, '0', NOW(), NOW()),
(3, '限时特惠', '限时优惠商品', 3, '0', NOW(), NOW()),
(4, '精选好物', '精选优质商品', 4, '0', NOW(), NOW()),
(5, '品牌专区', '品牌官方商品', 5, '0', NOW(), NOW());

-- =====================================================
-- 4. 运费模板测试数据
-- =====================================================
INSERT INTO `p_shipping_template` (`id`, `name`, `charge_type`, `is_free`, `free_condition_type`, `free_condition_value`, `status`, `created_at`, `updated_at`) VALUES
(1, '全国包邮', 1, 1, 0, 0.00, '0', NOW(), NOW()),
(2, '按件计费', 1, 0, 1, 99.00, '0', NOW(), NOW()),
(3, '按重量计费', 2, 0, 3, 5.00, '0', NOW(), NOW()),
(4, '按体积计费', 3, 0, 0, 0.00, '0', NOW(), NOW());

INSERT INTO `p_shipping_template_region` (`id`, `template_id`, `region_type`, `region_ids`, `region_names`, `first_unit`, `first_fee`, `continue_unit`, `continue_fee`, `is_free`, `free_condition_value`, `created_at`, `updated_at`) VALUES
(1, 1, 1, NULL, '全国', 1, 0.00, 1, 0.00, 1, 0.00, NOW(), NOW()),
(2, 2, 1, NULL, '默认', 1, 10.00, 1, 5.00, 0, 99.00, NOW(), NOW()),
(3, 2, 2, '110000,120000,310000', '北京,天津,上海', 1, 8.00, 1, 4.00, 0, 79.00, NOW(), NOW()),
(4, 2, 2, '540000,650000', '西藏,新疆', 1, 20.00, 1, 10.00, 0, 199.00, NOW(), NOW()),
(5, 3, 1, NULL, '默认', 1, 12.00, 1, 6.00, 0, 0.00, NOW(), NOW()),
(6, 4, 1, NULL, '默认', 1, 15.00, 1, 8.00, 0, 0.00, NOW(), NOW());

-- =====================================================
-- 5. 门店测试数据
-- =====================================================
INSERT INTO `p_store` (`id`, `name`, `logo`, `cover_image`, `contact_name`, `contact_phone`, `province`, `city`, `district`, `address`, `longitude`, `latitude`, `business_hours`, `description`, `sort`, `status`, `created_at`, `updated_at`) VALUES
(1, '北京朝阳店', 'https://example.com/store/logo1.png', 'https://example.com/store/cover1.png', '张经理', '13800138001', '北京市', '北京市', '朝阳区', '建国路88号SOHO现代城A座1层', 116.461879, 39.908722, '09:00-22:00', '北京朝阳区旗舰店', 1, '0', NOW(), NOW()),
(2, '上海浦东店', 'https://example.com/store/logo2.png', 'https://example.com/store/cover2.png', '李经理', '13800138002', '上海市', '上海市', '浦东新区', '陆家嘴环路1000号恒生银行大厦B1层', 121.506377, 31.239683, '10:00-22:00', '上海浦东新区门店', 2, '0', NOW(), NOW()),
(3, '深圳南山店', 'https://example.com/store/logo3.png', 'https://example.com/store/cover3.png', '王经理', '13800138003', '广东省', '深圳市', '南山区', '科技园南区深南大道9966号', 113.942025, 22.533716, '09:30-21:30', '深圳南山区门店', 3, '0', NOW(), NOW()),
(4, '广州天河店', 'https://example.com/store/logo4.png', 'https://example.com/store/cover4.png', '赵经理', '13800138004', '广东省', '广州市', '天河区', '天河路385号太古汇M层', 113.332951, 23.134521, '10:00-22:00', '广州天河区门店', 4, '0', NOW(), NOW()),
(5, '杭州西湖店', 'https://example.com/store/logo5.png', 'https://example.com/store/cover5.png', '钱经理', '13800138005', '浙江省', '杭州市', '西湖区', '文三路398号东信大厦1层', 120.130203, 30.287476, '09:00-21:00', '杭州西湖区门店', 5, '0', NOW(), NOW());

-- =====================================================
-- 6. 属性模板测试数据
-- =====================================================
INSERT INTO `p_attribute_template` (`id`, `name`, `category_id`, `sort`, `status`, `created_at`, `updated_at`) VALUES
(1, '手机属性', 20101, 1, '0', NOW(), NOW()),
(2, '电脑属性', 20201, 2, '0', NOW(), NOW()),
(3, '服装属性', 101, 3, '0', NOW(), NOW()),
(4, '鞋类属性', 103, 4, '0', NOW(), NOW());

INSERT INTO `p_attribute` (`id`, `template_id`, `name`, `attr_type`, `is_required`, `is_filter`, `sort`, `status`, `created_at`, `updated_at`) VALUES
(1, 1, '屏幕尺寸', 1, 1, 1, 1, '0', NOW(), NOW()),
(2, 1, '存储容量', 2, 1, 1, 2, '0', NOW(), NOW()),
(3, 1, '颜色', 2, 1, 1, 3, '0', NOW(), NOW()),
(4, 1, '处理器', 1, 0, 0, 4, '0', NOW(), NOW()),
(5, 1, '电池容量', 1, 0, 0, 5, '0', NOW(), NOW()),
(6, 2, '屏幕尺寸', 1, 1, 1, 1, '0', NOW(), NOW()),
(7, 2, '处理器', 1, 1, 1, 2, '0', NOW(), NOW()),
(8, 2, '内存容量', 2, 1, 1, 3, '0', NOW(), NOW()),
(9, 2, '硬盘容量', 2, 1, 1, 4, '0', NOW(), NOW()),
(10, 3, '材质', 2, 0, 1, 1, '0', NOW(), NOW()),
(11, 3, '季节', 2, 0, 1, 2, '0', NOW(), NOW()),
(12, 3, '风格', 2, 0, 1, 3, '0', NOW(), NOW()),
(13, 4, '鞋面材质', 2, 0, 1, 1, '0', NOW(), NOW()),
(14, 4, '鞋底材质', 2, 0, 0, 2, '0', NOW(), NOW()),
(15, 4, '闭合方式', 2, 0, 0, 3, '0', NOW(), NOW());

INSERT INTO `p_attribute_value` (`id`, `attribute_id`, `value`, `sort`, `created_at`) VALUES
(1, 2, '64GB', 1, NOW()),
(2, 2, '128GB', 2, NOW()),
(3, 2, '256GB', 3, NOW()),
(4, 2, '512GB', 4, NOW()),
(5, 2, '1TB', 5, NOW()),
(6, 3, '黑色', 1, NOW()),
(7, 3, '白色', 2, NOW()),
(8, 3, '蓝色', 3, NOW()),
(9, 3, '红色', 4, NOW()),
(10, 3, '绿色', 5, NOW()),
(11, 8, '8GB', 1, NOW()),
(12, 8, '16GB', 2, NOW()),
(13, 8, '32GB', 3, NOW()),
(14, 9, '256GB SSD', 1, NOW()),
(15, 9, '512GB SSD', 2, NOW()),
(16, 9, '1TB SSD', 3, NOW()),
(17, 10, '棉', 1, NOW()),
(18, 10, '涤纶', 2, NOW()),
(19, 10, '丝绸', 3, NOW()),
(20, 11, '春季', 1, NOW()),
(21, 11, '夏季', 2, NOW()),
(22, 11, '秋季', 3, NOW()),
(23, 11, '冬季', 4, NOW());

-- =====================================================
-- 7. 商品测试数据（SPU）
-- =====================================================
INSERT INTO `p_product` (`id`, `category_id`, `brand_id`, `name`, `subtitle`, `cover_image`, `images`, `video`, `detail`, `product_type`, `status`, `audit_status`, `sale_status`, `line_price`, `sale_price`, `cost_price`, `stock`, `sales`, `virtual_sales`, `limit_buy`, `limit_type`, `shipping_method`, `shipping_template_id`, `weight`, `volume`, `unit`, `sort`, `is_multi_spec`, `is_hot`, `is_new`, `is_recommend`, `keywords`, `description`, `created_at`, `updated_at`) VALUES
(1, 20101, 1, 'iPhone 15 Pro Max 256GB', '苹果最新旗舰手机', 'https://example.com/product/iphone15promax.jpg', '["https://example.com/product/iphone15promax_1.jpg", "https://example.com/product/iphone15promax_2.jpg", "https://example.com/product/iphone15promax_3.jpg"]', NULL, '<p>iPhone 15 Pro Max 采用钛金属设计，配备 A17 Pro 芯片，拥有 4800 万像素主摄，支持 5 倍光学变焦。</p>', 1, 1, 2, 1, 10999.00, 9999.00, 8000.00, 1000, 520, 1000, 0, 1, 1, 1, 0.221, 0.0005, '台', 1, 1, 1, 1, 1, 'iPhone,苹果,手机,iPhone15', 'iPhone 15 Pro Max 苹果旗舰手机', NOW(), NOW()),
(2, 20101, 2, '华为 Mate 60 Pro', '华为旗舰手机 麒麟芯片', 'https://example.com/product/mate60pro.jpg', '["https://example.com/product/mate60pro_1.jpg", "https://example.com/product/mate60pro_2.jpg"]', NULL, '<p>华为 Mate 60 Pro 搭载麒麟9000S芯片，支持卫星通话，拥有超可靠的玄武架构。</p>', 1, 1, 2, 1, 7999.00, 6999.00, 5500.00, 800, 380, 800, 2, 1, 1, 2, 0.225, 0.0005, '台', 2, 1, 1, 1, 1, '华为,Mate60,手机', '华为 Mate 60 Pro 旗舰手机', NOW(), NOW()),
(3, 20201, 2, '华为 MateBook X Pro', '华为轻薄笔记本', 'https://example.com/product/matebookxpro.jpg', '["https://example.com/product/matebookxpro_1.jpg"]', NULL, '<p>华为 MateBook X Pro 采用14.2英寸OLED屏幕，搭载第13代英特尔酷睿处理器。</p>', 1, 1, 2, 1, 12999.00, 11999.00, 9500.00, 200, 85, 200, 0, 1, 1, 2, 1.33, 0.003, '台', 3, 1, 0, 1, 1, '华为,笔记本,MateBook', '华为 MateBook X Pro 轻薄笔记本', NOW(), NOW()),
(4, 10101, 6, 'Nike Air Max 270', '耐克经典气垫跑鞋', 'https://example.com/product/nikeairmax270.jpg', '["https://example.com/product/nikeairmax270_1.jpg", "https://example.com/product/nikeairmax270_2.jpg"]', NULL, '<p>Nike Air Max 270 采用大容量气垫单元，提供全天舒适缓震。</p>', 1, 1, 2, 1, 1299.00, 899.00, 500.00, 500, 1200, 2000, 0, 1, 1, 2, 0.45, 0.002, '双', 4, 1, 1, 0, 1, 'Nike,耐克,跑鞋,AirMax', 'Nike Air Max 270 经典气垫跑鞋', NOW(), NOW()),
(5, 10201, 8, 'UNIQLO 女士纯棉T恤', '优衣库基础款T恤', 'https://example.com/product/uniqlotshirt.jpg', '["https://example.com/product/uniqlotshirt_1.jpg"]', NULL, '<p>优衣库女士纯棉T恤，舒适透气，百搭款式。</p>', 1, 1, 2, 1, 199.00, 79.00, 35.00, 2000, 5600, 10000, 10, 2, 1, 1, 0.15, 0.0003, '件', 5, 0, 0, 1, 0, '优衣库,T恤,女装', 'UNIQLO 女士纯棉T恤', NOW(), NOW()),
(6, 20101, 3, '小米14 Ultra', '小米影像旗舰', 'https://example.com/product/xiaomi14ultra.jpg', '["https://example.com/product/xiaomi14ultra_1.jpg"]', NULL, '<p>小米14 Ultra 搭载徕卡光学镜头，一英寸大底主摄，支持可变光圈。</p>', 1, 1, 2, 1, 6999.00, 5999.00, 4500.00, 600, 280, 500, 0, 1, 1, 3, 0.22, 0.0005, '台', 6, 1, 1, 1, 1, '小米,手机,小米14', '小米14 Ultra 影像旗舰', NOW(), NOW()),
(7, 301, NULL, '新疆阿克苏冰糖心苹果', '正宗新疆阿克苏苹果 5斤装', 'https://example.com/product/apple.jpg', '["https://example.com/product/apple_1.jpg"]', NULL, '<p>正宗新疆阿克苏冰糖心苹果，甜脆多汁，产地直发。</p>', 1, 1, 2, 1, 89.00, 59.00, 30.00, 5000, 12000, 15000, 5, 1, 1, 3, 2.5, 0.01, '份', 7, 1, 0, 1, 1, '苹果,水果,阿克苏', '新疆阿克苏冰糖心苹果', NOW(), NOW()),
(8, 302, NULL, '三只松鼠坚果大礼包', '网红零食坚果礼盒', 'https://example.com/product/nuts.jpg', '["https://example.com/product/nuts_1.jpg"]', NULL, '<p>三只松鼠坚果大礼包，包含夏威夷果、巴旦木、腰果等多种坚果。</p>', 1, 1, 2, 1, 199.00, 139.00, 80.00, 3000, 8500, 10000, 0, 1, 1, 3, 1.2, 0.008, '份', 8, 1, 1, 0, 1, '坚果,零食,三只松鼠', '三只松鼠坚果大礼包', NOW(), NOW()),
(9, 401, NULL, 'SK-II神仙水', '护肤精华露 230ml', 'https://example.com/product/skii.jpg', '["https://example.com/product/skii_1.jpg"]', NULL, '<p>SK-II护肤精华露，蕴含超过90%的PITERA精华，改善肌肤纹理。</p>', 1, 1, 2, 1, 1590.00, 1290.00, 900.00, 300, 580, 800, 0, 1, 1, 2, 0.3, 0.0005, '瓶', 9, 1, 1, 1, 1, 'SK-II,神仙水,护肤', 'SK-II神仙水护肤精华露', NOW(), NOW()),
(10, 20101, 4, 'OPPO Find X7 Ultra', 'OPPO影像旗舰', 'https://example.com/product/opfindx7ultra.jpg', '["https://example.com/product/opfindx7ultra_1.jpg"]', NULL, '<p>OPPO Find X7 Ultra 搭载双潜望镜头，哈苏影像系统。</p>', 1, 2, 2, 2, 6999.00, 5999.00, 4500.00, 400, 120, 200, 0, 1, 1, 3, 0.22, 0.0005, '台', 10, 0, 1, 0, 0, 'OPPO,手机,Find', 'OPPO Find X7 Ultra 影像旗舰', NOW(), NOW());

-- =====================================================
-- 8. 商品规格测试数据
-- =====================================================
INSERT INTO `p_spec` (`id`, `product_id`, `name`, `sort`, `created_at`) VALUES
(1, 1, '颜色', 1, NOW()),
(2, 1, '存储', 2, NOW()),
(3, 2, '颜色', 1, NOW()),
(4, 2, '存储', 2, NOW()),
(5, 4, '颜色', 1, NOW()),
(6, 4, '尺码', 2, NOW()),
(7, 5, '颜色', 1, NOW()),
(8, 5, '尺码', 2, NOW()),
(9, 6, '颜色', 1, NOW()),
(10, 6, '存储', 2, NOW());

INSERT INTO `p_spec_value` (`id`, `spec_id`, `product_id`, `value`, `image`, `color_code`, `sort`, `created_at`) VALUES
(1, 1, 1, '原色钛金属', 'https://example.com/sku/iphone15_natural.jpg', '#8B8680', 1, NOW()),
(2, 1, 1, '白色钛金属', 'https://example.com/sku/iphone15_white.jpg', '#F5F5F5', 2, NOW()),
(3, 1, 1, '蓝色钛金属', 'https://example.com/sku/iphone15_blue.jpg', '#4169E1', 3, NOW()),
(4, 1, 1, '黑色钛金属', 'https://example.com/sku/iphone15_black.jpg', '#2F2F2F', 4, NOW()),
(5, 2, 1, '256GB', NULL, NULL, 1, NOW()),
(6, 2, 1, '512GB', NULL, NULL, 2, NOW()),
(7, 2, 1, '1TB', NULL, NULL, 3, NOW()),
(8, 3, 2, '雅丹黑', 'https://example.com/sku/mate60_black.jpg', '#1C1C1C', 1, NOW()),
(9, 3, 2, '南糯紫', 'https://example.com/sku/mate60_purple.jpg', '#9370DB', 2, NOW()),
(10, 3, 2, '雅川青', 'https://example.com/sku/mate60_green.jpg', '#2E8B57', 3, NOW()),
(11, 4, 2, '256GB', NULL, NULL, 1, NOW()),
(12, 4, 2, '512GB', NULL, NULL, 2, NOW()),
(13, 4, 2, '1TB', NULL, NULL, 3, NOW()),
(14, 5, 4, '黑色', 'https://example.com/sku/nike_black.jpg', '#000000', 1, NOW()),
(15, 5, 4, '白色', 'https://example.com/sku/nike_white.jpg', '#FFFFFF', 2, NOW()),
(16, 5, 4, '蓝色', 'https://example.com/sku/nike_blue.jpg', '#0000FF', 3, NOW()),
(17, 6, 4, '39', NULL, NULL, 1, NOW()),
(18, 6, 4, '40', NULL, NULL, 2, NOW()),
(19, 6, 4, '41', NULL, NULL, 3, NOW()),
(20, 6, 4, '42', NULL, NULL, 4, NOW()),
(21, 6, 4, '43', NULL, NULL, 5, NOW()),
(22, 6, 4, '44', NULL, NULL, 6, NOW()),
(23, 7, 5, '白色', NULL, '#FFFFFF', 1, NOW()),
(24, 7, 5, '黑色', NULL, '#000000', 2, NOW()),
(25, 7, 5, '灰色', NULL, '#808080', 3, NOW()),
(26, 8, 5, 'S', NULL, NULL, 1, NOW()),
(27, 8, 5, 'M', NULL, NULL, 2, NOW()),
(28, 8, 5, 'L', NULL, NULL, 3, NOW()),
(29, 8, 5, 'XL', NULL, NULL, 4, NOW()),
(30, 8, 5, 'XXL', NULL, NULL, 5, NOW()),
(31, 9, 6, '黑色', 'https://example.com/sku/xiaomi14_black.jpg', '#000000', 1, NOW()),
(32, 9, 6, '白色', 'https://example.com/sku/xiaomi14_white.jpg', '#FFFFFF', 2, NOW()),
(33, 10, 6, '256GB', NULL, NULL, 1, NOW()),
(34, 10, 6, '512GB', NULL, NULL, 2, NOW()),
(35, 10, 6, '1TB', NULL, NULL, 3, NOW());

-- =====================================================
-- 9. 商品SKU测试数据
-- =====================================================
INSERT INTO `p_sku` (`id`, `product_id`, `sku_code`, `spec_value_ids`, `spec_text`, `image`, `sale_price`, `line_price`, `cost_price`, `stock`, `sales`, `weight`, `volume`, `status`, `created_at`, `updated_at`) VALUES
(1, 1, 'SKU-IPHONE15PM-256-NATURAL', '1,5', '原色钛金属 256GB', 'https://example.com/sku/iphone15_natural.jpg', 9999.00, 10999.00, 8000.00, 200, 120, 0.221, 0.0005, 1, NOW(), NOW()),
(2, 1, 'SKU-IPHONE15PM-256-WHITE', '2,5', '白色钛金属 256GB', 'https://example.com/sku/iphone15_white.jpg', 9999.00, 10999.00, 8000.00, 180, 100, 0.221, 0.0005, 1, NOW(), NOW()),
(3, 1, 'SKU-IPHONE15PM-256-BLUE', '3,5', '蓝色钛金属 256GB', 'https://example.com/sku/iphone15_blue.jpg', 9999.00, 10999.00, 8000.00, 150, 80, 0.221, 0.0005, 1, NOW(), NOW()),
(4, 1, 'SKU-IPHONE15PM-256-BLACK', '4,5', '黑色钛金属 256GB', 'https://example.com/sku/iphone15_black.jpg', 9999.00, 10999.00, 8000.00, 170, 90, 0.221, 0.0005, 1, NOW(), NOW()),
(5, 1, 'SKU-IPHONE15PM-512-NATURAL', '1,6', '原色钛金属 512GB', 'https://example.com/sku/iphone15_natural.jpg', 11999.00, 12999.00, 9500.00, 100, 60, 0.221, 0.0005, 1, NOW(), NOW()),
(6, 1, 'SKU-IPHONE15PM-512-WHITE', '2,6', '白色钛金属 512GB', 'https://example.com/sku/iphone15_white.jpg', 11999.00, 12999.00, 9500.00, 80, 40, 0.221, 0.0005, 1, NOW(), NOW()),
(7, 1, 'SKU-IPHONE15PM-1TB-NATURAL', '1,7', '原色钛金属 1TB', 'https://example.com/sku/iphone15_natural.jpg', 13999.00, 14999.00, 11000.00, 50, 20, 0.221, 0.0005, 1, NOW(), NOW()),
(8, 1, 'SKU-IPHONE15PM-1TB-BLACK', '4,7', '黑色钛金属 1TB', 'https://example.com/sku/iphone15_black.jpg', 13999.00, 14999.00, 11000.00, 70, 10, 0.221, 0.0005, 1, NOW(), NOW()),
(9, 2, 'SKU-MATE60P-256-BLACK', '8,11', '雅丹黑 256GB', 'https://example.com/sku/mate60_black.jpg', 6999.00, 7999.00, 5500.00, 200, 100, 0.225, 0.0005, 1, NOW(), NOW()),
(10, 2, 'SKU-MATE60P-256-PURPLE', '9,11', '南糯紫 256GB', 'https://example.com/sku/mate60_purple.jpg', 6999.00, 7999.00, 5500.00, 180, 90, 0.225, 0.0005, 1, NOW(), NOW()),
(11, 2, 'SKU-MATE60P-256-GREEN', '10,11', '雅川青 256GB', 'https://example.com/sku/mate60_green.jpg', 6999.00, 7999.00, 5500.00, 150, 80, 0.225, 0.0005, 1, NOW(), NOW()),
(12, 2, 'SKU-MATE60P-512-BLACK', '8,12', '雅丹黑 512GB', 'https://example.com/sku/mate60_black.jpg', 7999.00, 8999.00, 6200.00, 120, 60, 0.225, 0.0005, 1, NOW(), NOW()),
(13, 2, 'SKU-MATE60P-512-PURPLE', '9,12', '南糯紫 512GB', 'https://example.com/sku/mate60_purple.jpg', 7999.00, 8999.00, 6200.00, 100, 40, 0.225, 0.0005, 1, NOW(), NOW()),
(14, 2, 'SKU-MATE60P-1TB-BLACK', '8,13', '雅丹黑 1TB', 'https://example.com/sku/mate60_black.jpg', 8999.00, 9999.00, 7000.00, 50, 10, 0.225, 0.0005, 1, NOW(), NOW()),
(15, 4, 'SKU-NIKE270-BLACK-39', '14,17', '黑色 39', 'https://example.com/sku/nike_black.jpg', 899.00, 1299.00, 500.00, 50, 120, 0.45, 0.002, 1, NOW(), NOW()),
(16, 4, 'SKU-NIKE270-BLACK-40', '14,18', '黑色 40', 'https://example.com/sku/nike_black.jpg', 899.00, 1299.00, 500.00, 80, 200, 0.45, 0.002, 1, NOW(), NOW()),
(17, 4, 'SKU-NIKE270-BLACK-41', '14,19', '黑色 41', 'https://example.com/sku/nike_black.jpg', 899.00, 1299.00, 500.00, 100, 250, 0.45, 0.002, 1, NOW(), NOW()),
(18, 4, 'SKU-NIKE270-BLACK-42', '14,20', '黑色 42', 'https://example.com/sku/nike_black.jpg', 899.00, 1299.00, 500.00, 120, 300, 0.45, 0.002, 1, NOW(), NOW()),
(19, 4, 'SKU-NIKE270-BLACK-43', '14,21', '黑色 43', 'https://example.com/sku/nike_black.jpg', 899.00, 1299.00, 500.00, 80, 180, 0.45, 0.002, 1, NOW(), NOW()),
(20, 4, 'SKU-NIKE270-WHITE-39', '15,17', '白色 39', 'https://example.com/sku/nike_white.jpg', 899.00, 1299.00, 500.00, 40, 80, 0.45, 0.002, 1, NOW(), NOW()),
(21, 4, 'SKU-NIKE270-WHITE-40', '15,18', '白色 40', 'https://example.com/sku/nike_white.jpg', 899.00, 1299.00, 500.00, 60, 100, 0.45, 0.002, 1, NOW(), NOW()),
(22, 4, 'SKU-NIKE270-BLUE-41', '16,19', '蓝色 41', 'https://example.com/sku/nike_blue.jpg', 899.00, 1299.00, 500.00, 30, 50, 0.45, 0.002, 1, NOW(), NOW()),
(23, 5, 'SKU-UNIQLO-WHITE-S', '23,26', '白色 S', NULL, 79.00, 199.00, 35.00, 500, 1500, 0.15, 0.0003, 1, NOW(), NOW()),
(24, 5, 'SKU-UNIQLO-WHITE-M', '23,27', '白色 M', NULL, 79.00, 199.00, 35.00, 600, 1800, 0.15, 0.0003, 1, NOW(), NOW()),
(25, 5, 'SKU-UNIQLO-WHITE-L', '23,28', '白色 L', NULL, 79.00, 199.00, 35.00, 400, 1200, 0.15, 0.0003, 1, NOW(), NOW()),
(26, 5, 'SKU-UNIQLO-BLACK-S', '24,26', '黑色 S', NULL, 79.00, 199.00, 35.00, 300, 600, 0.15, 0.0003, 1, NOW(), NOW()),
(27, 5, 'SKU-UNIQLO-BLACK-M', '24,27', '黑色 M', NULL, 79.00, 199.00, 35.00, 350, 700, 0.15, 0.0003, 1, NOW(), NOW()),
(28, 5, 'SKU-UNIQLO-GRAY-L', '25,28', '灰色 L', NULL, 79.00, 199.00, 35.00, 200, 400, 0.15, 0.0003, 1, NOW(), NOW()),
(29, 6, 'SKU-XIAOMI14-BLACK-256', '31,33', '黑色 256GB', 'https://example.com/sku/xiaomi14_black.jpg', 5999.00, 6999.00, 4500.00, 150, 80, 0.22, 0.0005, 1, NOW(), NOW()),
(30, 6, 'SKU-XIAOMI14-WHITE-256', '32,33', '白色 256GB', 'https://example.com/sku/xiaomi14_white.jpg', 5999.00, 6999.00, 4500.00, 120, 70, 0.22, 0.0005, 1, NOW(), NOW()),
(31, 6, 'SKU-XIAOMI14-BLACK-512', '31,34', '黑色 512GB', 'https://example.com/sku/xiaomi14_black.jpg', 6499.00, 7499.00, 5000.00, 100, 50, 0.22, 0.0005, 1, NOW(), NOW()),
(32, 6, 'SKU-XIAOMI14-WHITE-512', '32,34', '白色 512GB', 'https://example.com/sku/xiaomi14_white.jpg', 6499.00, 7499.00, 5000.00, 80, 40, 0.22, 0.0005, 1, NOW(), NOW()),
(33, 6, 'SKU-XIAOMI14-BLACK-1TB', '31,35', '黑色 1TB', 'https://example.com/sku/xiaomi14_black.jpg', 6999.00, 7999.00, 5500.00, 50, 20, 0.22, 0.0005, 1, NOW(), NOW()),
(34, 6, 'SKU-XIAOMI14-WHITE-1TB', '32,35', '白色 1TB', 'https://example.com/sku/xiaomi14_white.jpg', 6999.00, 7999.00, 5500.00, 50, 20, 0.22, 0.0005, 1, NOW(), NOW());

-- =====================================================
-- 10. 商品分组关联测试数据
-- =====================================================
INSERT INTO `p_product_group_relation` (`id`, `product_id`, `group_id`, `created_at`) VALUES
(1, 1, 1, NOW()),
(2, 2, 1, NOW()),
(3, 6, 1, NOW()),
(4, 1, 2, NOW()),
(5, 2, 2, NOW()),
(6, 6, 2, NOW()),
(7, 7, 3, NOW()),
(8, 8, 3, NOW()),
(9, 1, 4, NOW()),
(10, 2, 4, NOW()),
(11, 3, 4, NOW()),
(12, 6, 4, NOW()),
(13, 1, 5, NOW()),
(14, 2, 5, NOW());

-- =====================================================
-- 11. 商品分类关联测试数据
-- =====================================================
INSERT INTO `p_product_category` (`id`, `product_id`, `category_id`, `created_at`) VALUES
(1, 1, 20101, NOW()),
(2, 2, 20101, NOW()),
(3, 3, 20201, NOW()),
(4, 4, 10101, NOW()),
(5, 5, 10201, NOW()),
(6, 6, 20101, NOW()),
(7, 7, 301, NOW()),
(8, 8, 302, NOW()),
(9, 9, 401, NOW()),
(10, 10, 20101, NOW());

-- =====================================================
-- 12. 库存预警配置测试数据
-- =====================================================
INSERT INTO `p_stock_alert` (`id`, `product_id`, `sku_id`, `alert_stock`, `is_alert`, `last_alert_at`, `created_at`, `updated_at`) VALUES
(1, 1, 1, 50, 0, NULL, NOW(), NOW()),
(2, 1, 5, 30, 0, NULL, NOW(), NOW()),
(3, 1, 7, 20, 1, NOW(), NOW(), NOW()),
(4, 2, 9, 50, 0, NULL, NOW(), NOW()),
(5, 2, 14, 20, 1, NOW(), NOW(), NOW()),
(6, 4, 15, 20, 1, NOW(), NOW(), NOW()),
(7, 6, 29, 30, 0, NULL, NOW(), NOW());

-- =====================================================
-- 13. 库存变动日志测试数据
-- =====================================================
INSERT INTO `p_stock_log` (`id`, `product_id`, `sku_id`, `change_type`, `change_num`, `before_stock`, `after_stock`, `order_no`, `remark`, `operator_id`, `operator_name`, `created_at`) VALUES
(1, 1, 1, 1, 200, 0, 200, NULL, '初始入库', 1, 'admin', NOW()),
(2, 1, 1, 2, -1, 200, 199, 'ORD202603230001', '订单出库', NULL, '系统', NOW()),
(3, 1, 5, 1, 100, 0, 100, NULL, '初始入库', 1, 'admin', NOW()),
(4, 2, 9, 1, 200, 0, 200, NULL, '初始入库', 1, 'admin', NOW()),
(5, 4, 17, 1, 100, 0, 100, NULL, '初始入库', 1, 'admin', NOW()),
(6, 4, 17, 2, -2, 100, 98, 'ORD202603230002', '订单出库', NULL, '系统', NOW()),
(7, 5, 23, 1, 500, 0, 500, NULL, '初始入库', 1, 'admin', NOW()),
(8, 5, 24, 1, 600, 0, 600, NULL, '初始入库', 1, 'admin', NOW()),
(9, 6, 29, 1, 150, 0, 150, NULL, '初始入库', 1, 'admin', NOW()),
(10, 6, 30, 1, 120, 0, 120, NULL, '初始入库', 1, 'admin', NOW());

-- =====================================================
-- 14. 商品属性关联测试数据
-- =====================================================
INSERT INTO `p_product_attribute` (`id`, `product_id`, `attribute_id`, `attribute_name`, `attribute_value`, `created_at`) VALUES
(1, 1, 1, '屏幕尺寸', '6.7英寸', NOW()),
(2, 1, 4, '处理器', 'A17 Pro', NOW()),
(3, 1, 5, '电池容量', '4422mAh', NOW()),
(4, 2, 1, '屏幕尺寸', '6.82英寸', NOW()),
(5, 2, 4, '处理器', '麒麟9000S', NOW()),
(6, 3, 6, '屏幕尺寸', '14.2英寸', NOW()),
(7, 3, 7, '处理器', '第13代英特尔酷睿i7', NOW()),
(8, 4, 13, '鞋面材质', '网布+合成材料', NOW()),
(9, 4, 14, '鞋底材质', '橡胶', NOW()),
(10, 5, 10, '材质', '纯棉', NOW()),
(11, 5, 11, '季节', '夏季', NOW()),
(12, 6, 1, '屏幕尺寸', '6.73英寸', NOW()),
(13, 6, 4, '处理器', '骁龙8 Gen 3', NOW());

-- ROLLBACK:
-- DELETE FROM p_product_attribute WHERE id >= 1;
-- DELETE FROM p_stock_log WHERE id >= 1;
-- DELETE FROM p_stock_alert WHERE id >= 1;
-- DELETE FROM p_product_category WHERE id >= 1;
-- DELETE FROM p_product_group_relation WHERE id >= 1;
-- DELETE FROM p_sku WHERE id >= 1;
-- DELETE FROM p_spec_value WHERE id >= 1;
-- DELETE FROM p_spec WHERE id >= 1;
-- DELETE FROM p_product WHERE id >= 1;
-- DELETE FROM p_attribute_value WHERE id >= 1;
-- DELETE FROM p_attribute WHERE id >= 1;
-- DELETE FROM p_attribute_template WHERE id >= 1;
-- DELETE FROM p_store WHERE id >= 1;
-- DELETE FROM p_shipping_template_region WHERE id >= 1;
-- DELETE FROM p_shipping_template WHERE id >= 1;
-- DELETE FROM p_product_group WHERE id >= 1;
-- DELETE FROM p_brand WHERE id >= 1;
-- DELETE FROM p_category WHERE id >= 1;
