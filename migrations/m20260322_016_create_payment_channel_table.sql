-- m20260322_016_create_payment_channel_table.sql
-- 描述：创建支付渠道管理表和退款记录表
-- 作者：tuoke
-- 日期：2026-03-22

-- 支付渠道配置表
CREATE TABLE IF NOT EXISTS `pay_channel` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '渠道名称',
    `code` varchar(50) NOT NULL COMMENT '渠道标识',
    `channel_type` varchar(20) NOT NULL COMMENT '渠道类型: wechat/alipay/unionpay/balance',
    `scenes` json DEFAULT NULL COMMENT '适用场景: ["h5","app","miniapp","pc"]',
    `config` json DEFAULT NULL COMMENT '渠道配置JSON',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用: 0停用 1启用',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_channel_type` (`channel_type`),
    KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='支付渠道配置表';

-- 退款记录表（关联c_payment_order）
CREATE TABLE IF NOT EXISTS `pay_refund` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `refund_no` varchar(64) NOT NULL COMMENT '退款单号',
    `refund_id` varchar(64) DEFAULT NULL COMMENT '第三方退款号',
    `order_no` varchar(64) NOT NULL COMMENT '原支付订单号(关联c_payment_order.order_no)',
    `consumer_id` bigint(20) NOT NULL COMMENT '用户ID',
    `amount` decimal(12,2) NOT NULL COMMENT '退款金额',
    `reason` varchar(500) DEFAULT NULL COMMENT '退款原因',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态: pending/processing/success/failed',
    `refund_time` datetime DEFAULT NULL COMMENT '退款完成时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_refund_no` (`refund_no`),
    KEY `idx_order_no` (`order_no`),
    KEY `idx_consumer_id` (`consumer_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='退款记录表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `pay_refund`;
-- DROP TABLE IF EXISTS `pay_channel`;
