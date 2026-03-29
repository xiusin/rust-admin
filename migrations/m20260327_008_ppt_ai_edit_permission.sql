-- m20260327_008_ppt_ai_edit_permission.sql
-- 描述：AI PPT编辑功能权限配置
-- 作者：tuoke
-- 日期：2026-03-27

-- 权限类型说明：
-- F = 按钮/功能权限

START TRANSACTION;

-- =====================================================
-- AI编辑功能权限
-- =====================================================

-- 内容润色
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4201, 'ai-ppt-edit-polish', '内容润色', 4001, 101, 'F', '1', 'ppt:ai-edit:polish', NOW());

-- 内容续写
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4202, 'ai-ppt-edit-continue', '内容续写', 4001, 102, 'F', '1', 'ppt:ai-edit:continue', NOW());

-- 内容总结
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4203, 'ai-ppt-edit-summarize', '内容总结', 4001, 103, 'F', '1', 'ppt:ai-edit:summarize', NOW());

-- 内容翻译
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4204, 'ai-ppt-edit-translate', '内容翻译', 4001, 104, 'F', '1', 'ppt:ai-edit:translate', NOW());

-- 智能建议
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4205, 'ai-ppt-edit-suggest', '智能建议', 4001, 105, 'F', '1', 'ppt:ai-edit:suggest', NOW());

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 4201 AND id <= 4205;
