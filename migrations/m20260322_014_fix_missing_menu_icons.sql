-- m20260322_014_fix_missing_menu_icons.sql
-- 描述：修复缺失的菜单图标，-- 作者：tuoke
-- 日期：2026-03-22

-- 一级菜单图标修复
UPDATE sys_menu SET icon = 'icon-lock' WHERE id = 13;
UPDATE sys_menu SET icon = 'icon-earth' WHERE id = 17;
UPDATE sys_menu SET icon = 'icon-credit-card' WHERE id = 220;
UPDATE sys_menu SET icon = 'icon-shopping-cart' WHERE id = 230;
UPDATE sys_menu SET icon = 'icon-car' WHERE id = 240;
UPDATE sys_menu SET icon = 'icon-table' WHERE id = 3;
UPDATE sys_menu SET icon = 'icon-edit' WHERE id = 4;

-- 二级菜单图标修复
UPDATE sys_menu SET icon = 'icon-route' WHERE id = 901;
UPDATE sys_menu SET icon = 'icon-tree' WHERE id = 903;
UPDATE sys_menu SET icon = 'icon-folder' WHERE id = 1004;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 1006;

-- 电商业务子菜单图标修复
UPDATE sys_menu SET icon = 'icon-money' WHERE id = 9901;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 9902;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 9903;
UPDATE sys_menu SET icon = 'icon-truck' WHERE id = 9905;
UPDATE sys_menu SET icon = 'icon-rollback' WHERE id = 9906;
UPDATE sys_menu SET icon = 'icon-star' WHERE id = 9907;
UPDATE sys_menu SET icon = 'icon-tag' WHERE id = 9908;
UPDATE sys_menu SET icon = 'icon-lock' WHERE id = 9909;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 9910;

-- 订单管理子菜单图标修复
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2301;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 2302;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2303;

-- 修复所有不以icon-开头的图标
UPDATE sys_menu SET icon = CONCAT('icon-', icon) WHERE icon IS NOT NULL AND icon != '' AND icon NOT LIKE 'icon-%' AND deleted_at IS NULL;

-- ROLLBACK:
-- UPDATE sys_menu SET icon = REPLACE(icon, 'icon-', '') WHERE icon LIKE 'icon-%' AND deleted_at IS NULL;
