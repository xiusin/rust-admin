-- m20260323_001_product_create_tables.sql
-- 描述：商品管理模块核心表结构
-- 作者：tuoke
-- 日期：2026-03-23

-- =====================================================
-- 1. 商品分类表（支持多级分类，最多4级）
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_category` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `parent_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '父分类ID，0为顶级分类',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '分类名称',
    `icon` varchar(500) DEFAULT NULL COMMENT '分类图标URL',
    `image` varchar(500) DEFAULT NULL COMMENT '分类图片URL',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序（升序）',
    `level` tinyint(4) NOT NULL DEFAULT 1 COMMENT '层级深度（1-4）',
    `path` varchar(255) NOT NULL DEFAULT '' COMMENT '层级路径（如：1,2,3）',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `show_in_nav` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否显示在导航栏（0否 1是）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_parent_id` (`parent_id`),
    KEY `idx_level` (`level`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分类表';

-- =====================================================
-- 2. 商品品牌表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_brand` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '品牌名称',
    `logo` varchar(500) DEFAULT NULL COMMENT '品牌Logo',
    `description` varchar(500) DEFAULT NULL COMMENT '品牌描述',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品品牌表';

-- =====================================================
-- 3. 商品主表（SPU）
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_product` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `category_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '分类ID',
    `brand_id` bigint(20) DEFAULT NULL COMMENT '品牌ID',
    `name` varchar(200) NOT NULL DEFAULT '' COMMENT '商品名称',
    `subtitle` varchar(500) DEFAULT NULL COMMENT '商品副标题',
    `cover_image` varchar(500) NOT NULL DEFAULT '' COMMENT '封面图片',
    `images` json DEFAULT NULL COMMENT '商品图片列表（JSON数组）',
    `video` varchar(500) DEFAULT NULL COMMENT '商品视频URL',
    `detail` longtext COMMENT '商品详情（富文本）',
    `product_type` tinyint(4) NOT NULL DEFAULT 1 COMMENT '商品类型（1实物 2虚拟 3核销）',
    `status` tinyint(4) NOT NULL DEFAULT 2 COMMENT '商品状态（1上架 2下架 3回收站）',
    `audit_status` tinyint(4) NOT NULL DEFAULT 1 COMMENT '审核状态（1待审核 2已审核 3审核不通过）',
    `audit_remark` varchar(500) DEFAULT NULL COMMENT '审核备注',
    `sale_status` tinyint(4) NOT NULL DEFAULT 2 COMMENT '开售状态（1开售 2未开售）',
    `sale_time` datetime DEFAULT NULL COMMENT '开售时间',
    `line_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '划线价',
    `sale_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '售价（最低SKU价）',
    `cost_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '成本价',
    `stock` int(11) NOT NULL DEFAULT 0 COMMENT '总库存',
    `sales` int(11) NOT NULL DEFAULT 0 COMMENT '实际销量',
    `virtual_sales` int(11) NOT NULL DEFAULT 0 COMMENT '虚拟销量',
    `limit_buy` int(11) NOT NULL DEFAULT 0 COMMENT '限购数量（0不限购）',
    `limit_type` tinyint(4) NOT NULL DEFAULT 1 COMMENT '限购维度（1按商品 2按SKU）',
    `shipping_method` tinyint(4) NOT NULL DEFAULT 1 COMMENT '配送方式（1快递 2自提 3无需配送）',
    `shipping_template_id` bigint(20) DEFAULT NULL COMMENT '运费模板ID',
    `weight` decimal(10,3) NOT NULL DEFAULT 0.000 COMMENT '商品重量（kg）',
    `volume` decimal(10,3) NOT NULL DEFAULT 0.000 COMMENT '商品体积（m³）',
    `unit` varchar(20) NOT NULL DEFAULT '件' COMMENT '商品单位',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `is_multi_spec` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否多规格（0单规格 1多规格）',
    `is_hot` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否热销（0否 1是）',
    `is_new` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否新品（0否 1是）',
    `is_recommend` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否推荐（0否 1是）',
    `keywords` varchar(200) DEFAULT NULL COMMENT 'SEO关键词',
    `description` varchar(500) DEFAULT NULL COMMENT 'SEO描述',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_category_id` (`category_id`),
    KEY `idx_brand_id` (`brand_id`),
    KEY `idx_status` (`status`),
    KEY `idx_audit_status` (`audit_status`),
    KEY `idx_sale_status` (`sale_status`),
    KEY `idx_is_hot` (`is_hot`),
    KEY `idx_is_new` (`is_new`),
    KEY `idx_is_recommend` (`is_recommend`),
    KEY `idx_sort` (`sort`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品主表';

-- =====================================================
-- 4. 商品分组表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_product_group` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '分组名称',
    `description` varchar(500) DEFAULT NULL COMMENT '分组描述',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分组表';

-- =====================================================
-- 5. 商品分组关联表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_product_group_relation` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `group_id` bigint(20) NOT NULL COMMENT '分组ID',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_product_group` (`product_id`, `group_id`),
    KEY `idx_group_id` (`group_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分组关联表';

-- =====================================================
-- 6. 商品规格名表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_spec` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '规格名（如：颜色、尺码）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_product_id` (`product_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品规格名表';

-- =====================================================
-- 7. 商品规格值表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_spec_value` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `spec_id` bigint(20) NOT NULL COMMENT '规格名ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `value` varchar(100) NOT NULL DEFAULT '' COMMENT '规格值（如：红色、XL）',
    `image` varchar(500) DEFAULT NULL COMMENT '规格值图片',
    `color_code` varchar(20) DEFAULT NULL COMMENT '颜色代码（如：#FF0000）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_spec_id` (`spec_id`),
    KEY `idx_product_id` (`product_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品规格值表';

-- =====================================================
-- 8. 商品SKU表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_sku` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `sku_code` varchar(100) NOT NULL DEFAULT '' COMMENT 'SKU编码',
    `spec_value_ids` varchar(255) NOT NULL DEFAULT '' COMMENT '规格值ID组合（逗号分隔，如：1,2,3）',
    `spec_text` varchar(500) NOT NULL DEFAULT '' COMMENT '规格描述文本（如：红色-XL）',
    `image` varchar(500) DEFAULT NULL COMMENT 'SKU图片',
    `sale_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '售价',
    `line_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '划线价',
    `cost_price` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '成本价',
    `stock` int(11) NOT NULL DEFAULT 0 COMMENT '库存',
    `sales` int(11) NOT NULL DEFAULT 0 COMMENT '销量',
    `weight` decimal(10,3) NOT NULL DEFAULT 0.000 COMMENT '重量（kg）',
    `volume` decimal(10,3) NOT NULL DEFAULT 0.000 COMMENT '体积（m³）',
    `status` tinyint(4) NOT NULL DEFAULT 1 COMMENT '状态（1启用 2禁用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_sku_code` (`sku_code`),
    KEY `idx_product_id` (`product_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品SKU表';

-- =====================================================
-- 9. 库存变动日志表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_stock_log` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `sku_id` bigint(20) DEFAULT NULL COMMENT 'SKU ID（单规格为NULL）',
    `change_type` tinyint(4) NOT NULL COMMENT '变动类型（1入库 2出库 3预占 4释放 5退货入库 6盘点调整）',
    `change_num` int(11) NOT NULL COMMENT '变动数量（正数为增加，负数为减少）',
    `before_stock` int(11) NOT NULL COMMENT '变动前库存',
    `after_stock` int(11) NOT NULL COMMENT '变动后库存',
    `order_no` varchar(64) DEFAULT NULL COMMENT '关联订单号',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `operator_id` bigint(20) DEFAULT NULL COMMENT '操作人ID',
    `operator_name` varchar(50) DEFAULT NULL COMMENT '操作人名称',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_product_id` (`product_id`),
    KEY `idx_sku_id` (`sku_id`),
    KEY `idx_order_no` (`order_no`),
    KEY `idx_change_type` (`change_type`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存变动日志表';

-- =====================================================
-- 10. 库存预警配置表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_stock_alert` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `sku_id` bigint(20) DEFAULT NULL COMMENT 'SKU ID（单规格为NULL）',
    `alert_stock` int(11) NOT NULL DEFAULT 10 COMMENT '预警库存阈值',
    `is_alert` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否已预警（0否 1是）',
    `last_alert_at` datetime DEFAULT NULL COMMENT '最后预警时间',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_product_sku` (`product_id`, `sku_id`),
    KEY `idx_is_alert` (`is_alert`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存预警配置表';

-- =====================================================
-- 11. 运费模板表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_shipping_template` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '模板名称',
    `charge_type` tinyint(4) NOT NULL DEFAULT 1 COMMENT '计费方式（1按件 2按重量 3按体积）',
    `is_free` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否包邮（0否 1是）',
    `free_condition_type` tinyint(4) NOT NULL DEFAULT 0 COMMENT '包邮条件类型（0无 1满金额 2满件数 3满重量）',
    `free_condition_value` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '包邮条件值',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='运费模板表';

-- =====================================================
-- 12. 运费模板地区规则表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_shipping_template_region` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `template_id` bigint(20) NOT NULL COMMENT '运费模板ID',
    `region_type` tinyint(4) NOT NULL DEFAULT 1 COMMENT '地区类型（1默认 2自定义）',
    `region_ids` varchar(2000) DEFAULT NULL COMMENT '地区ID列表（逗号分隔）',
    `region_names` varchar(2000) DEFAULT NULL COMMENT '地区名称列表（逗号分隔）',
    `first_unit` int(11) NOT NULL DEFAULT 1 COMMENT '首件/首重/首体积',
    `first_fee` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '首费',
    `continue_unit` int(11) NOT NULL DEFAULT 1 COMMENT '续件/续重/续体积',
    `continue_fee` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '续费',
    `is_free` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否包邮（0否 1是）',
    `free_condition_value` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '包邮条件值',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_template_id` (`template_id`),
    KEY `idx_region_type` (`region_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='运费模板地区规则表';

-- =====================================================
-- 13. 门店表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_store` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '门店名称',
    `logo` varchar(500) DEFAULT NULL COMMENT '门店Logo',
    `cover_image` varchar(500) DEFAULT NULL COMMENT '门店封面图',
    `contact_name` varchar(50) DEFAULT NULL COMMENT '联系人',
    `contact_phone` varchar(20) DEFAULT NULL COMMENT '联系电话',
    `province` varchar(30) DEFAULT NULL COMMENT '省',
    `city` varchar(30) DEFAULT NULL COMMENT '市',
    `district` varchar(30) DEFAULT NULL COMMENT '区',
    `address` varchar(200) DEFAULT NULL COMMENT '详细地址',
    `longitude` decimal(10,6) DEFAULT NULL COMMENT '经度',
    `latitude` decimal(10,6) DEFAULT NULL COMMENT '纬度',
    `business_hours` varchar(100) DEFAULT NULL COMMENT '营业时间（如：09:00-22:00）',
    `description` varchar(500) DEFAULT NULL COMMENT '门店描述',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_status` (`status`),
    KEY `idx_city` (`city`),
    KEY `idx_district` (`district`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='门店表';

-- =====================================================
-- 14. 门店商品库存表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_store_stock` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `store_id` bigint(20) NOT NULL COMMENT '门店ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `sku_id` bigint(20) DEFAULT NULL COMMENT 'SKU ID',
    `stock` int(11) NOT NULL DEFAULT 0 COMMENT '库存',
    `alert_stock` int(11) NOT NULL DEFAULT 10 COMMENT '预警库存',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_store_product_sku` (`store_id`, `product_id`, `sku_id`),
    KEY `idx_product_id` (`product_id`),
    KEY `idx_sku_id` (`sku_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='门店商品库存表';

-- =====================================================
-- 15. 核销码表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_verification_code` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `order_id` bigint(20) NOT NULL COMMENT '订单ID',
    `order_no` varchar(64) NOT NULL DEFAULT '' COMMENT '订单号',
    `order_item_id` bigint(20) NOT NULL COMMENT '订单明细ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `sku_id` bigint(20) DEFAULT NULL COMMENT 'SKU ID',
    `code` varchar(32) NOT NULL DEFAULT '' COMMENT '核销码',
    `qr_code` varchar(500) DEFAULT NULL COMMENT '核销二维码URL',
    `total_count` int(11) NOT NULL DEFAULT 1 COMMENT '可核销总次数',
    `used_count` int(11) NOT NULL DEFAULT 0 COMMENT '已核销次数',
    `status` tinyint(4) NOT NULL DEFAULT 1 COMMENT '状态（1待核销 2已核销 3已过期 4已退款）',
    `expire_at` datetime DEFAULT NULL COMMENT '有效期截止',
    `verified_at` datetime DEFAULT NULL COMMENT '核销时间',
    `verified_by` bigint(20) DEFAULT NULL COMMENT '核销操作人ID',
    `verified_by_name` varchar(50) DEFAULT NULL COMMENT '核销操作人名称',
    `store_id` bigint(20) DEFAULT NULL COMMENT '核销门店ID',
    `store_name` varchar(100) DEFAULT NULL COMMENT '核销门店名称',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_order_id` (`order_id`),
    KEY `idx_order_no` (`order_no`),
    KEY `idx_product_id` (`product_id`),
    KEY `idx_status` (`status`),
    KEY `idx_expire_at` (`expire_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='核销码表';

-- =====================================================
-- 16. 核销记录表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_verification_log` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `verification_code_id` bigint(20) NOT NULL COMMENT '核销码ID',
    `code` varchar(32) NOT NULL DEFAULT '' COMMENT '核销码',
    `order_no` varchar(64) NOT NULL DEFAULT '' COMMENT '订单号',
    `store_id` bigint(20) DEFAULT NULL COMMENT '核销门店ID',
    `store_name` varchar(100) DEFAULT NULL COMMENT '核销门店名称',
    `verified_by` bigint(20) DEFAULT NULL COMMENT '核销操作人ID',
    `verified_by_name` varchar(50) DEFAULT NULL COMMENT '核销操作人名称',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_verification_code_id` (`verification_code_id`),
    KEY `idx_code` (`code`),
    KEY `idx_order_no` (`order_no`),
    KEY `idx_store_id` (`store_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='核销记录表';

-- =====================================================
-- 17. 商品分类关联表（多对多）
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_product_category` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `category_id` bigint(20) NOT NULL COMMENT '分类ID',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_product_category` (`product_id`, `category_id`),
    KEY `idx_category_id` (`category_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分类关联表';

-- =====================================================
-- 18. 商品属性模板表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_attribute_template` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '属性模板名称',
    `category_id` bigint(20) DEFAULT NULL COMMENT '关联分类ID',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_category_id` (`category_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品属性模板表';

-- =====================================================
-- 19. 商品属性项表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_attribute` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `template_id` bigint(20) DEFAULT NULL COMMENT '属性模板ID',
    `name` varchar(100) NOT NULL DEFAULT '' COMMENT '属性名称',
    `attr_type` tinyint(4) NOT NULL DEFAULT 1 COMMENT '属性类型（1文本 2单选 3多选 4数值）',
    `is_required` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否必填（0否 1是）',
    `is_filter` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否可筛选（0否 1是）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态（0正常 1停用）',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_template_id` (`template_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品属性项表';

-- =====================================================
-- 20. 商品属性值表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_attribute_value` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `attribute_id` bigint(20) NOT NULL COMMENT '属性ID',
    `value` varchar(200) NOT NULL DEFAULT '' COMMENT '属性值',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_attribute_id` (`attribute_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品属性值表';

-- =====================================================
-- 21. 商品属性关联表
-- =====================================================
CREATE TABLE IF NOT EXISTS `p_product_attribute` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `attribute_id` bigint(20) NOT NULL COMMENT '属性ID',
    `attribute_name` varchar(100) NOT NULL DEFAULT '' COMMENT '属性名称',
    `attribute_value` varchar(500) NOT NULL DEFAULT '' COMMENT '属性值',
    `created_at` datetime DEFAULT NULL COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_product_id` (`product_id`),
    KEY `idx_attribute_id` (`attribute_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品属性关联表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `p_product_attribute`;
-- DROP TABLE IF EXISTS `p_attribute_value`;
-- DROP TABLE IF EXISTS `p_attribute`;
-- DROP TABLE IF EXISTS `p_attribute_template`;
-- DROP TABLE IF EXISTS `p_product_category`;
-- DROP TABLE IF EXISTS `p_verification_log`;
-- DROP TABLE IF EXISTS `p_verification_code`;
-- DROP TABLE IF EXISTS `p_store_stock`;
-- DROP TABLE IF EXISTS `p_store`;
-- DROP TABLE IF EXISTS `p_shipping_template_region`;
-- DROP TABLE IF EXISTS `p_shipping_template`;
-- DROP TABLE IF EXISTS `p_stock_alert`;
-- DROP TABLE IF EXISTS `p_stock_log`;
-- DROP TABLE IF EXISTS `p_sku`;
-- DROP TABLE IF EXISTS `p_spec_value`;
-- DROP TABLE IF EXISTS `p_spec`;
-- DROP TABLE IF EXISTS `p_product_group_relation`;
-- DROP TABLE IF EXISTS `p_product_group`;
-- DROP TABLE IF EXISTS `p_product`;
-- DROP TABLE IF EXISTS `p_brand`;
-- DROP TABLE IF EXISTS `p_category`;
