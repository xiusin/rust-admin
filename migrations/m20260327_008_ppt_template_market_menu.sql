-- m20260327_008_ppt_template_market_menu.sql
-- 描述：PPT模板市场菜单配置
-- 作者：tuoke
-- 日期：2026-03-27

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

START TRANSACTION;

-- =====================================================
-- 1. 模板市场菜单
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (4030, 'ai-ppt-template-market', '模板市场', 4000, 4, '/ai-ppt/template-market', 'ai-ppt/template-market/list', 'C', '1', 'ppt:template-market:list', 'icon-shop', NOW(), 'PPT模板市场');

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4031, 'ai-ppt-template-market-detail', '模板详情', 4030, 1, '/ai-ppt/template-market/detail', 'ai-ppt/template-market/detail', 'C', '1', 'ppt:template-market:detail', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4032, 'ai-ppt-template-market-upload', '上传模板', 4030, 2, '/ai-ppt/template-market/upload', 'ai-ppt/template-market/form', 'C', '1', 'ppt:template-market:upload', '', NOW());

-- =====================================================
-- 2. 模板市场按钮权限
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4033, 'ai-ppt-template-market-edit', '编辑模板', 4030, 3, '', 'F', '1', 'ppt:template-market:edit', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4034, 'ai-ppt-template-market-delete', '删除模板', 4030, 4, '', 'F', '1', 'ppt:template-market:delete', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4035, 'ai-ppt-template-market-apply', '应用模板', 4030, 5, '', 'F', '1', 'ppt:template-market:apply', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4036, 'ai-ppt-template-market-rate', '评价模板', 4030, 6, '', 'F', '1', 'ppt:template-market:rate', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4037, 'ai-ppt-template-market-collect', '收藏模板', 4030, 7, '', 'F', '1', 'ppt:template-market:collect', '', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (4038, 'ai-ppt-template-market-audit', '审核模板', 4030, 8, '', 'F', '1', 'ppt:template-market:audit', '', NOW());

-- =====================================================
-- 3. 我的收藏菜单
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (4040, 'ai-ppt-my-collection', '我的收藏', 4000, 5, '/ai-ppt/my-collection', 'ai-ppt/my-collection/list', 'C', '1', 'ppt:collection:list', 'icon-star', NOW(), '我收藏的PPT模板');

-- =====================================================
-- 4. 我的上传菜单
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (4050, 'ai-ppt-my-templates', '我的模板', 4000, 6, '/ai-ppt/my-templates', 'ai-ppt/my-templates/list', 'C', '1', 'ppt:my-templates:list', 'icon-folder', NOW(), '我上传的PPT模板');

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 4030 AND id < 4100;
