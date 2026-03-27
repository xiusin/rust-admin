-- m20260326_003_cms_config_tables.sql
-- 描述：CMS配置相关表结构
-- 作者：tuoke
-- 日期：2026-03-26

-- =====================================================
-- 1. CMS表单配置表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_form_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `model_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '所属模型ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '表单名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '表单编码',
    `form_type` varchar(20) NOT NULL DEFAULT '' COMMENT '表单类型',
    `layout` json DEFAULT NULL COMMENT '表单布局',
    `groups` json DEFAULT NULL COMMENT '表单分组',
    `actions` json DEFAULT NULL COMMENT '表单操作',
    `rules` json DEFAULT NULL COMMENT '表单验证规则',
    `hooks` json DEFAULT NULL COMMENT '表单钩子',
    `is_default` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否默认表单（0否 1是）',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_model_code` (`model_id`, `code`),
    KEY `idx_model` (`model_id`),
    KEY `idx_is_default` (`is_default`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS表单配置表';

-- =====================================================
-- 2. CMS表格配置表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_table_config` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `model_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '所属模型ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '表格名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '表格编码',
    `columns` json DEFAULT NULL COMMENT '列配置',
    `search` json DEFAULT NULL COMMENT '搜索配置',
    `filter` json DEFAULT NULL COMMENT '筛选配置',
    `actions` json DEFAULT NULL COMMENT '行操作',
    `batch_actions` json DEFAULT NULL COMMENT '批量操作',
    `toolbar` json DEFAULT NULL COMMENT '工具栏操作',
    `pagination` json DEFAULT NULL COMMENT '分页配置',
    `features` json DEFAULT NULL COMMENT '表格特性',
    `is_default` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否默认表格（0否 1是）',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_model_code` (`model_id`, `code`),
    KEY `idx_model` (`model_id`),
    KEY `idx_is_default` (`is_default`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS表格配置表';

-- =====================================================
-- 3. CMS代码生成配置表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_code_gen` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `model_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '模型ID',
    `gen_type` varchar(20) NOT NULL DEFAULT 'download' COMMENT '生成类型（download下载 preview预览 direct_apply直接应用）',
    `output_path` varchar(500) NOT NULL DEFAULT '' COMMENT '输出路径',
    `template_config` json DEFAULT NULL COMMENT '模板配置',
    `file_config` json DEFAULT NULL COMMENT '文件配置',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_model` (`model_id`),
    KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS代码生成配置表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `cms_code_gen`;
-- DROP TABLE IF EXISTS `cms_table_config`;
-- DROP TABLE IF EXISTS `cms_form_config`;
