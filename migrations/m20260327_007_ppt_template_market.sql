-- m20260327_007_ppt_template_market.sql
-- 描述：PPT模板市场相关表
-- 作者：tuoke
-- 日期：2026-03-27

START TRANSACTION;

-- =====================================================
-- 1. 模板市场主表
-- =====================================================
CREATE TABLE `ppt_template_market` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(100) NOT NULL COMMENT '模板名称',
    `description` varchar(500) DEFAULT NULL COMMENT '模板描述',
    `industry` varchar(50) DEFAULT NULL COMMENT '所属行业',
    `style` varchar(50) DEFAULT NULL COMMENT '模板风格',
    `style_params` json DEFAULT NULL COMMENT '样式参数(JSON格式)',
    `thumbnail_url` varchar(500) DEFAULT NULL COMMENT '缩略图URL',
    `preview_urls` json DEFAULT NULL COMMENT '预览图URL列表(JSON格式)',
    `template_file` varchar(500) DEFAULT NULL COMMENT '模板文件路径',
    `uploader_id` bigint(20) NOT NULL COMMENT '上传者ID',
    `uploader_name` varchar(50) NOT NULL COMMENT '上传者名称',
    `downloads` int(11) NOT NULL DEFAULT '0' COMMENT '下载次数',
    `rating` float NOT NULL DEFAULT '0' COMMENT '平均评分',
    `rating_count` int(11) NOT NULL DEFAULT '0' COMMENT '评价数量',
    `collection_count` int(11) NOT NULL DEFAULT '0' COMMENT '收藏数量',
    `is_free` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否免费(0:付费 1:免费)',
    `price` decimal(10,2) DEFAULT NULL COMMENT '价格(付费时)',
    `status` char(1) NOT NULL DEFAULT '0' COMMENT '状态(0:待审核 1:已发布 2:已下架)',
    `remark` varchar(500) DEFAULT NULL COMMENT '备注',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted_at` datetime DEFAULT NULL COMMENT '删除时间',
    PRIMARY KEY (`id`),
    KEY `idx_uploader_id` (`uploader_id`),
    KEY `idx_industry` (`industry`),
    KEY `idx_style` (`style`),
    KEY `idx_status` (`status`),
    KEY `idx_downloads` (`downloads`),
    KEY `idx_rating` (`rating`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT模板市场';

-- =====================================================
-- 2. 模板评价表
-- =====================================================
CREATE TABLE `ppt_template_rating` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `template_id` bigint(20) NOT NULL COMMENT '模板ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `user_name` varchar(50) NOT NULL COMMENT '用户名称',
    `rating` int(11) NOT NULL COMMENT '评分(1-5)',
    `comment` varchar(500) DEFAULT NULL COMMENT '评论内容',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_template_user` (`template_id`, `user_id`),
    KEY `idx_template_id` (`template_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT模板评价';

-- =====================================================
-- 3. 模板收藏表
-- =====================================================
CREATE TABLE `ppt_template_collection` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `template_id` bigint(20) NOT NULL COMMENT '模板ID',
    `user_id` bigint(20) NOT NULL COMMENT '用户ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_template_user` (`template_id`, `user_id`),
    KEY `idx_template_id` (`template_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT模板收藏';

-- =====================================================
-- 4. 模板标签表
-- =====================================================
CREATE TABLE `ppt_template_tag` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `name` varchar(50) NOT NULL COMMENT '标签名称',
    `description` varchar(200) DEFAULT NULL COMMENT '标签描述',
    `usage_count` int(11) NOT NULL DEFAULT '0' COMMENT '使用次数',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_name` (`name`),
    KEY `idx_usage_count` (`usage_count`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT模板标签';

-- =====================================================
-- 5. 模板标签关联表
-- =====================================================
CREATE TABLE `ppt_template_tag_relation` (
    `id` bigint(20) NOT NULL COMMENT '主键ID',
    `template_id` bigint(20) NOT NULL COMMENT '模板ID',
    `tag_id` bigint(20) NOT NULL COMMENT '标签ID',
    `created_at` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_template_tag` (`template_id`, `tag_id`),
    KEY `idx_template_id` (`template_id`),
    KEY `idx_tag_id` (`tag_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='PPT模板标签关联';

-- =====================================================
-- 6. 插入默认标签数据
-- =====================================================
INSERT INTO `ppt_template_tag` (`id`, `name`, `description`, `usage_count`, `created_at`) VALUES
(1, '商务', '商务风格模板', 0, NOW()),
(2, '简约', '简约风格模板', 0, NOW()),
(3, '科技', '科技风格模板', 0, NOW()),
(4, '教育', '教育风格模板', 0, NOW()),
(5, '医疗', '医疗健康风格模板', 0, NOW()),
(6, '金融', '金融行业模板', 0, NOW()),
(7, '互联网', '互联网行业模板', 0, NOW()),
(8, '创意', '创意设计风格模板', 0, NOW()),
(9, '正式', '正式严肃风格模板', 0, NOW()),
(10, '活泼', '活泼轻松风格模板', 0, NOW());

COMMIT;

-- ROLLBACK:
-- DROP TABLE IF EXISTS `ppt_template_tag_relation`;
-- DROP TABLE IF EXISTS `ppt_template_tag`;
-- DROP TABLE IF EXISTS `ppt_template_collection`;
-- DROP TABLE IF EXISTS `ppt_template_rating`;
-- DROP TABLE IF EXISTS `ppt_template_market`;
