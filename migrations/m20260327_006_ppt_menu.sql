-- m20260327_006_ppt_menu.sql
-- 描述：AI PPT生成系统菜单配置
-- 作者：tuoke
-- 日期：2026-03-27

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

START TRANSACTION;

-- =====================================================
-- 1. AI PPT生成主菜单
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (4000, 'ppt', 'AI PPT生成', 0, 6, '/ppt', 'M', '1', NULL, 'icon-file', NOW(), 'AI PPT生成系统');

-- =====================================================
-- 2. PPT生成
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4001, 'ppt-generate', 'PPT生成', 4000, 1, '/ppt/generate', 'ppt/generate/index', 'C', '1', 'ppt:generate:list', 'icon-plus', NOW());

-- =====================================================
-- 3. PPT编辑器
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4002, 'ppt-editor', 'PPT编辑', 4000, 2, '/ppt/editor', 'ppt/editor/index', 'C', '1', 'ppt:editor:list', 'icon-edit', NOW());

-- =====================================================
-- 4. 模板市场
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4003, 'ppt-template', '模板市场', 4000, 3, '/ppt/template', 'ppt/template/index', 'C', '1', 'ppt:template:list', 'icon-apps', NOW());

-- =====================================================
-- 5. 生成历史
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4004, 'ppt-history', '生成历史', 4000, 4, '/ppt/history', 'ppt/history/index', 'C', '1', 'ppt:history:list', 'icon-history', NOW());

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 4000 AND id < 4100;
