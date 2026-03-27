-- m20260326_004_cms_menu.sql
-- 描述：CMS内容管理系统菜单配置
-- 作者：tuoke
-- 日期：2026-03-26

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

START TRANSACTION;

-- =====================================================
-- 1. CMS主菜单
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (3000, 'cms', 'CMS管理', 0, 5, '/cms', 'M', '1', NULL, 'document', NOW(), 'CMS内容管理系统');

-- =====================================================
-- 2. 模型管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3001, 'cms-model', '模型管理', 3000, 1, '/cms/model', 'cms/model/list', 'C', '1', 'cms:model:list', 'grid', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3002, 'cms-model-design', '模型设计', 3001, 1, '/cms/model/design', 'cms/model/design', 'C', '1', 'cms:model:design', '', NOW());

-- =====================================================
-- 3. 内容管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3010, 'cms-content', '内容管理', 3000, 2, '/cms/content', 'cms/content/list', 'C', '1', 'cms:content:list', 'document', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3011, 'cms-content-edit', '内容编辑', 3010, 1, '/cms/content/edit', 'cms/content/form', 'C', '1', 'cms:content:edit', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3012, 'cms-content-detail', '内容详情', 3010, 2, '/cms/content/detail', 'cms/content/detail', 'C', '1', 'cms:content:detail', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3013, 'cms-content-recycle', '回收站', 3010, 3, '/cms/content/recycle', 'cms/content/recycle', 'C', '1', 'cms:content:recycle', '', NOW());

-- =====================================================
-- 4. 分类管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3020, 'cms-category', '分类管理', 3000, 3, '/cms/category', 'cms/category/list', 'C', '1', 'cms:category:list', 'folder', NOW());

-- =====================================================
-- 5. 标签管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3030, 'cms-tag', '标签管理', 3000, 4, '/cms/tag', 'cms/tag/list', 'C', '1', 'cms:tag:list', 'price-tag', NOW());

-- =====================================================
-- 6. 表单配置
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3040, 'cms-form', '表单配置', 3000, 5, '/cms/form', 'cms/form/list', 'C', '1', 'cms:form:list', 'form', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3041, 'cms-form-builder', '表单构建', 3040, 1, '/cms/form/builder', 'cms/form/builder', 'C', '1', 'cms:form:builder', '', NOW());

-- =====================================================
-- 7. 表格配置
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3050, 'cms-table', '表格配置', 3000, 6, '/cms/table', 'cms/table/list', 'C', '1', 'cms:table:list', 'table', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3051, 'cms-table-builder', '表格构建', 3050, 1, '/cms/table/builder', 'cms/table/builder', 'C', '1', 'cms:table:builder', '', NOW());

-- =====================================================
-- 8. 代码生成
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3060, 'cms-code-gen', '代码生成', 3000, 7, '/cms/code-gen', 'cms/code-gen/index', 'C', '1', 'cms:codegen:list', 'code', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (3061, 'cms-code-gen-preview', '代码预览', 3060, 1, '/cms/code-gen/preview', 'cms/code-gen/preview', 'C', '1', 'cms:codegen:preview', '', NOW());

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 3000 AND id < 3100;
