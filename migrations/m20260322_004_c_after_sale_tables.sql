-- =====================================================
-- 售后模块表结构
-- 创建日期: 2026-03-22
-- 描述: 售后申请、退款、物流关联、状态管理
-- =====================================================

-- 1. 售后申请表
CREATE TABLE IF NOT EXISTS `c_after_sale` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `after_sale_no` varchar(64) NOT NULL COMMENT '售后单号',
    `order_id` bigint(20) NOT NULL COMMENT '订单ID',
    `order_no` varchar(64) NOT NULL COMMENT '订单编号',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `type` varchar(20) NOT NULL COMMENT '售后类型: refund_only/return_refund/exchange',
    `reason` varchar(500) NOT NULL COMMENT '申请原因',
    `description` varchar(1000) DEFAULT NULL COMMENT '详细描述',
    `evidence_urls` json DEFAULT NULL COMMENT '凭证图片URLs',
    `refund_amount` decimal(15,2) NOT NULL COMMENT '退款金额',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态: pending/processing/agreed/rejected/closed/completed',
    `apply_at` datetime NOT NULL COMMENT '申请时间',
    `process_at` datetime DEFAULT NULL COMMENT '处理时间',
    `complete_at` datetime DEFAULT NULL COMMENT '完成时间',
    `close_at` datetime DEFAULT NULL COMMENT '关闭时间',
    `reject_reason` varchar(500) DEFAULT NULL COMMENT '拒绝原因',
    `processor_id` bigint(20) DEFAULT NULL COMMENT '处理人ID',
    `processor_name` varchar(50) DEFAULT NULL COMMENT '处理人姓名',
    `timeout_at` datetime DEFAULT NULL COMMENT '超时时间',
    `is_timeout` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否超时',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_after_sale_no` (`after_sale_no`),
    KEY `idx_order_id` (`order_id`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_status` (`status`),
    KEY `idx_timeout_at` (`timeout_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后申请表';

-- 2. 售后商品明细表
CREATE TABLE IF NOT EXISTS `c_after_sale_item` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `after_sale_id` bigint(20) NOT NULL COMMENT '售后单ID',
    `order_item_id` bigint(20) NOT NULL COMMENT '订单商品ID',
    `product_id` bigint(20) NOT NULL COMMENT '商品ID',
    `product_name` varchar(200) NOT NULL COMMENT '商品名称',
    `sku_id` bigint(20) DEFAULT NULL COMMENT 'SKU ID',
    `sku_name` varchar(200) DEFAULT NULL COMMENT 'SKU名称',
    `quantity` int(11) NOT NULL COMMENT '数量',
    `unit_price` decimal(15,2) NOT NULL COMMENT '单价',
    `refund_amount` decimal(15,2) NOT NULL COMMENT '退款金额',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_after_sale_id` (`after_sale_id`),
    KEY `idx_order_item_id` (`order_item_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后商品明细表';

-- 3. 售后退款记录表
CREATE TABLE IF NOT EXISTS `c_after_sale_refund` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `after_sale_id` bigint(20) NOT NULL COMMENT '售后单ID',
    `refund_no` varchar(64) NOT NULL COMMENT '退款单号',
    `transaction_id` varchar(64) DEFAULT NULL COMMENT '支付交易ID',
    `refund_channel` varchar(20) NOT NULL COMMENT '退款渠道: wechat/alipay/balance',
    `refund_amount` decimal(15,2) NOT NULL COMMENT '退款金额',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态: pending/processing/success/failed',
    `refund_at` datetime DEFAULT NULL COMMENT '退款成功时间',
    `fail_reason` varchar(500) DEFAULT NULL COMMENT '失败原因',
    `retry_count` int(11) NOT NULL DEFAULT 0 COMMENT '重试次数',
    `next_retry_at` datetime DEFAULT NULL COMMENT '下次重试时间',
    `callback_data` text DEFAULT NULL COMMENT '回调数据',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_refund_no` (`refund_no`),
    KEY `idx_after_sale_id` (`after_sale_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后退款记录表';

-- 4. 售后物流表
CREATE TABLE IF NOT EXISTS `c_after_sale_logistics` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `after_sale_id` bigint(20) NOT NULL COMMENT '售后单ID',
    `logistics_type` varchar(20) NOT NULL COMMENT '物流类型: return/exchange',
    `logistics_company` varchar(50) DEFAULT NULL COMMENT '物流公司',
    `tracking_no` varchar(100) DEFAULT NULL COMMENT '物流单号',
    `sender_name` varchar(50) DEFAULT NULL COMMENT '寄件人姓名',
    `sender_phone` varchar(20) DEFAULT NULL COMMENT '寄件人电话',
    `sender_address` varchar(500) DEFAULT NULL COMMENT '寄件人地址',
    `receiver_name` varchar(50) DEFAULT NULL COMMENT '收件人姓名',
    `receiver_phone` varchar(20) DEFAULT NULL COMMENT '收件人电话',
    `receiver_address` varchar(500) DEFAULT NULL COMMENT '收件人地址',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态: pending/shipped/received',
    `shipped_at` datetime DEFAULT NULL COMMENT '发货时间',
    `received_at` datetime DEFAULT NULL COMMENT '签收时间',
    `tracking_info` json DEFAULT NULL COMMENT '物流轨迹信息',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `idx_after_sale_id` (`after_sale_id`),
    KEY `idx_tracking_no` (`tracking_no`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后物流表';

-- 5. 售后状态变更记录表
CREATE TABLE IF NOT EXISTS `c_after_sale_status_log` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `after_sale_id` bigint(20) NOT NULL COMMENT '售后单ID',
    `old_status` varchar(20) DEFAULT NULL COMMENT '原状态',
    `new_status` varchar(20) NOT NULL COMMENT '新状态',
    `operator_type` varchar(20) NOT NULL COMMENT '操作人类型: consumer/admin/system',
    `operator_id` bigint(20) DEFAULT NULL COMMENT '操作人ID',
    `operator_name` varchar(50) DEFAULT NULL COMMENT '操作人姓名',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_after_sale_id` (`after_sale_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后状态变更记录表';

-- 6. 售后超时配置表
CREATE TABLE IF NOT EXISTS `c_after_sale_timeout_config` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `stage` varchar(50) NOT NULL COMMENT '阶段: apply_audit/return_ship/return_receive/refund_process',
    `timeout_hours` int(11) NOT NULL COMMENT '超时小时数',
    `auto_action` varchar(50) NOT NULL COMMENT '超时自动动作: auto_agree/auto_complete/auto_close',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `description` varchar(200) DEFAULT NULL COMMENT '描述',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_stage` (`stage`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='售后超时配置表';

-- 插入默认超时配置
INSERT INTO `c_after_sale_timeout_config` (`id`, `stage`, `timeout_hours`, `auto_action`, `description`) VALUES
(1, 'apply_audit', 48, 'auto_agree', '商家审核超时自动同意'),
(2, 'return_ship', 168, 'auto_close', '用户退货超时自动关闭'),
(3, 'return_receive', 72, 'auto_complete', '商家确认收货超时自动完成'),
(4, 'refund_process', 24, 'auto_retry', '退款处理超时自动重试');
