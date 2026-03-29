-- m20260327_011_ppt_payment_order.sql
-- 描述：创建PPT支付订单表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_payment_order` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `order_no` varchar(64) NOT NULL COMMENT '订单号',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `plan_id` bigint(20) NOT NULL COMMENT '套餐ID',
    `subscription_id` bigint(20) DEFAULT NULL COMMENT '订阅ID',
    `amount` decimal(10,2) NOT NULL COMMENT '支付金额',
    `original_amount` decimal(10,2) NOT NULL COMMENT '原价金额',
    `discount_amount` decimal(10,2) NOT NULL DEFAULT 0.00 COMMENT '优惠金额',
    `payment_method` varchar(20) NOT NULL COMMENT '支付方式(alipay/wechat/balance)',
    `payment_channel` varchar(50) DEFAULT NULL COMMENT '支付渠道',
    `transaction_id` varchar(128) DEFAULT NULL COMMENT '第三方交易号',
    `callback_data` text COMMENT '回调数据',
    `paid_at` datetime DEFAULT NULL COMMENT '支付时间',
    `refunded_at` datetime DEFAULT NULL COMMENT '退款时间',
    `refund_amount` decimal(10,2) DEFAULT NULL COMMENT '退款金额',
    `refund_reason` varchar(500) DEFAULT NULL COMMENT '退款原因',
    `status` varchar(20) NOT NULL DEFAULT 'pending' COMMENT '状态(pending/paid/cancelled/refunded)',
    `expires_at` datetime NOT NULL COMMENT '过期时间',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_no` (`order_no`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plan_id` (`plan_id`),
    KEY `idx_subscription_id` (`subscription_id`),
    KEY `idx_status` (`status`),
    KEY `idx_transaction_id` (`transaction_id`),
    KEY `idx_created_at` (`created_at`),
    CONSTRAINT `fk_payment_order_user` FOREIGN KEY (`user_id`) REFERENCES `sys_user` (`id`) ON DELETE CASCADE,
    CONSTRAINT `fk_payment_order_plan` FOREIGN KEY (`plan_id`) REFERENCES `ppt_subscription_plan` (`id`) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT支付订单表';

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_payment_order`;
