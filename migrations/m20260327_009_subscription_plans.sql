-- m20260327_009_subscription_plans.sql
-- 描述：创建PPT订阅套餐表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

CREATE TABLE IF NOT EXISTS `ppt_subscription_plan` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '套餐名称',
    `code` varchar(50) NOT NULL COMMENT '套餐编码',
    `description` text COMMENT '套餐描述',
    `price` decimal(10,2) NOT NULL COMMENT '价格',
    `original_price` decimal(10,2) DEFAULT NULL COMMENT '原价',
    `duration_days` int(11) NOT NULL COMMENT '有效期(天)',
    `max_projects` int(11) NOT NULL DEFAULT -1 COMMENT '最大项目数(-1无限)',
    `ai_credits` int(11) NOT NULL COMMENT 'AI调用额度',
    `features` json DEFAULT NULL COMMENT '功能特性',
    `is_active` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用',
    `sort_order` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_is_active` (`is_active`),
    KEY `idx_sort_order` (`sort_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT订阅套餐表';

INSERT INTO `ppt_subscription_plan` (`name`, `code`, `description`, `price`, `original_price`, `duration_days`, `max_projects`, `ai_credits`, `features`, `sort_order`) VALUES
('免费版', 'free', '适合个人轻度使用', 0.00, 0.00, 36500, 3, 10, '{"basic_templates": true, "export_formats": ["pptx"]}', 1),
('基础版', 'basic', '适合个人日常使用', 29.00, 39.00, 30, 10, 100, '{"basic_templates": true, "standard_templates": true, "export_formats": ["pptx", "pdf"]}', 2),
('专业版', 'professional', '适合专业人士和小团队', 99.00, 129.00, 30, 50, 500, '{"all_templates": true, "export_formats": ["pptx", "pdf", "images"], "api_access": true, "priority_support": true}', 3),
('企业版', 'enterprise', '适合企业和团队协作', 299.00, 399.00, 30, -1, 2000, '{"all_features": true, "api_access": true, "priority_support": true, "custom_branding": true}', 4);

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_subscription_plan`;
