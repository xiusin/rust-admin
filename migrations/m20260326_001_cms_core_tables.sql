-- m20260326_001_cms_core_tables.sql
-- 描述：CMS内容管理系统核心表结构
-- 作者：tuoke
-- 日期：2026-03-26

-- =====================================================
-- 1. CMS模型表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_model` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '模型名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '模型编码',
    `table_name` varchar(100) NOT NULL DEFAULT '' COMMENT '数据表名',
    `description` varchar(500) DEFAULT NULL COMMENT '模型描述',
    `icon` varchar(100) DEFAULT NULL COMMENT '模型图标',
    `category_id` bigint(20) DEFAULT NULL COMMENT '所属分类',
    `is_system` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否系统模型（0否 1是）',
    `is_enabled` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否启用（0否 1是）',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `config` json DEFAULT NULL COMMENT '模型配置',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    UNIQUE KEY `uk_table_name` (`table_name`),
    KEY `idx_category` (`category_id`),
    KEY `idx_enabled` (`is_enabled`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS模型表';

-- =====================================================
-- 2. CMS字段表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_field` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `model_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '所属模型ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '字段名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '字段编码',
    `field_type` varchar(30) NOT NULL DEFAULT '' COMMENT '字段类型',
    `db_type` varchar(50) NOT NULL DEFAULT '' COMMENT '数据库类型',
    `default_value` varchar(500) DEFAULT NULL COMMENT '默认值',
    `is_required` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否必填（0否 1是）',
    `is_unique` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否唯一（0否 1是）',
    `is_searchable` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否可搜索（0否 1是）',
    `is_sortable` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否可排序（0否 1是）',
    `is_filterable` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否可筛选（0否 1是）',
    `is_list_show` tinyint(1) NOT NULL DEFAULT 1 COMMENT '列表是否显示（0否 1是）',
    `is_form_show` tinyint(1) NOT NULL DEFAULT 1 COMMENT '表单是否显示（0否 1是）',
    `is_detail_show` tinyint(1) NOT NULL DEFAULT 1 COMMENT '详情是否显示（0否 1是）',
    `form_config` json DEFAULT NULL COMMENT '表单配置',
    `table_config` json DEFAULT NULL COMMENT '表格配置',
    `validation` json DEFAULT NULL COMMENT '验证规则',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_model_code` (`model_id`, `code`),
    KEY `idx_model` (`model_id`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS字段表';

-- =====================================================
-- 3. CMS分类表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_category` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `parent_id` bigint(20) DEFAULT NULL COMMENT '父分类ID',
    `model_id` bigint(20) DEFAULT NULL COMMENT '关联模型ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '分类名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '分类编码',
    `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
    `icon` varchar(100) DEFAULT NULL COMMENT '图标',
    `image` varchar(500) DEFAULT NULL COMMENT '图片',
    `description` varchar(500) DEFAULT NULL COMMENT '描述',
    `keywords` varchar(500) DEFAULT NULL COMMENT '关键词',
    `template_list` varchar(100) DEFAULT NULL COMMENT '列表模板',
    `template_detail` varchar(100) DEFAULT NULL COMMENT '详情模板',
    `page_size` int(11) NOT NULL DEFAULT 20 COMMENT '每页数量',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `seo_title` varchar(200) DEFAULT NULL COMMENT 'SEO标题',
    `seo_keywords` varchar(500) DEFAULT NULL COMMENT 'SEO关键词',
    `seo_description` varchar(1000) DEFAULT NULL COMMENT 'SEO描述',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_parent` (`parent_id`),
    KEY `idx_model` (`model_id`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS分类表';

-- =====================================================
-- 4. CMS标签表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_tag` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '标签名称',
    `code` varchar(50) NOT NULL DEFAULT '' COMMENT '标签编码',
    `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
    `color` varchar(20) DEFAULT NULL COMMENT '颜色',
    `icon` varchar(100) DEFAULT NULL COMMENT '图标',
    `description` varchar(500) DEFAULT NULL COMMENT '描述',
    `seo_title` varchar(200) DEFAULT NULL COMMENT 'SEO标题',
    `seo_keywords` varchar(500) DEFAULT NULL COMMENT 'SEO关键词',
    `seo_description` varchar(1000) DEFAULT NULL COMMENT 'SEO描述',
    `content_count` bigint(20) NOT NULL DEFAULT 0 COMMENT '内容数量',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `status` tinyint(1) NOT NULL DEFAULT 1 COMMENT '状态（0禁用 1启用）',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_code` (`code`),
    KEY `idx_status` (`status`),
    KEY `idx_sort` (`sort`),
    KEY `idx_content_count` (`content_count`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS标签表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `cms_tag`;
-- DROP TABLE IF EXISTS `cms_category`;
-- DROP TABLE IF EXISTS `cms_field`;
-- DROP TABLE IF EXISTS `cms_model`;
