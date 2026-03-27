-- m20260326_002_cms_content_tables.sql
-- 描述：CMS内容相关表结构
-- 作者：tuoke
-- 日期：2026-03-26

-- =====================================================
-- 1. CMS内容主表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_content` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `model_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '模型ID',
    `category_id` bigint(20) DEFAULT NULL COMMENT '分类ID',
    `title` varchar(200) NOT NULL DEFAULT '' COMMENT '标题',
    `slug` varchar(200) DEFAULT NULL COMMENT 'URL别名',
    `keywords` varchar(500) DEFAULT NULL COMMENT '关键词',
    `description` varchar(1000) DEFAULT NULL COMMENT '描述',
    `author_id` bigint(20) DEFAULT NULL COMMENT '作者ID',
    `source` varchar(100) DEFAULT NULL COMMENT '来源',
    `thumbnail` varchar(500) DEFAULT NULL COMMENT '缩略图',
    `images` json DEFAULT NULL COMMENT '图片集',
    `attachments` json DEFAULT NULL COMMENT '附件',
    `content_type` varchar(20) NOT NULL DEFAULT 'article' COMMENT '内容类型',
    `status` tinyint(1) NOT NULL DEFAULT 0 COMMENT '状态（0草稿 1待审核 2已拒绝 3已发布 4已下线 5回收站）',
    `publish_time` datetime DEFAULT NULL COMMENT '发布时间',
    `expire_time` datetime DEFAULT NULL COMMENT '过期时间',
    `sort` int(11) NOT NULL DEFAULT 0 COMMENT '排序',
    `view_count` bigint(20) NOT NULL DEFAULT 0 COMMENT '浏览次数',
    `like_count` bigint(20) NOT NULL DEFAULT 0 COMMENT '点赞次数',
    `comment_count` bigint(20) NOT NULL DEFAULT 0 COMMENT '评论次数',
    `is_top` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否置顶（0否 1是）',
    `is_recommend` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否推荐（0否 1是）',
    `is_hot` tinyint(1) NOT NULL DEFAULT 0 COMMENT '是否热门（0否 1是）',
    `allow_comment` tinyint(1) NOT NULL DEFAULT 1 COMMENT '是否允许评论（0否 1是）',
    `password` varchar(100) DEFAULT NULL COMMENT '访问密码',
    `template` varchar(100) DEFAULT NULL COMMENT '模板文件',
    `extra_data` json DEFAULT NULL COMMENT '扩展数据',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_model` (`model_id`),
    KEY `idx_category` (`category_id`),
    KEY `idx_author` (`author_id`),
    KEY `idx_status` (`status`),
    KEY `idx_publish_time` (`publish_time`),
    KEY `idx_slug` (`slug`),
    KEY `idx_is_top` (`is_top`),
    KEY `idx_is_recommend` (`is_recommend`),
    KEY `idx_is_hot` (`is_hot`),
    KEY `idx_view_count` (`view_count`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容主表';

-- =====================================================
-- 2. CMS内容标签关联表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_content_tag` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `content_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '内容ID',
    `tag_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '标签ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_content_tag` (`content_id`, `tag_id`),
    KEY `idx_content` (`content_id`),
    KEY `idx_tag` (`tag_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容标签关联表';

-- =====================================================
-- 3. CMS内容版本表
-- =====================================================
CREATE TABLE IF NOT EXISTS `cms_content_version` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `content_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '内容ID',
    `version` int(11) NOT NULL DEFAULT 1 COMMENT '版本号',
    `data` json NOT NULL COMMENT '内容快照',
    `change_log` varchar(500) DEFAULT NULL COMMENT '变更日志',
    `operator_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '操作人ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    KEY `idx_content` (`content_id`),
    KEY `idx_version` (`content_id`, `version`),
    KEY `idx_operator` (`operator_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='CMS内容版本表';

-- ROLLBACK:
-- DROP TABLE IF EXISTS `cms_content_version`;
-- DROP TABLE IF EXISTS `cms_content_tag`;
-- DROP TABLE IF EXISTS `cms_content`;
