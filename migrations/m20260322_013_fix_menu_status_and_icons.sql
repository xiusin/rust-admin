-- m20260322_013_fix_menu_status_and_icons.sql
-- 描述：修复菜单状态和图标
-- 作者：tuoke
-- 日期：2026-03-22

-- 1. 修复所有菜单状态为正常(0)
UPDATE sys_menu SET status = '0' WHERE status = '1' AND deleted_at IS NULL;

-- 2. 修复一级菜单图标
UPDATE sys_menu SET icon = 'icon-home' WHERE id = 1;
UPDATE sys_menu SET icon = 'icon-folder' WHERE id = 2;
UPDATE sys_menu SET icon = 'icon-table' WHERE id = 3;
UPDATE sys_menu SET icon = 'icon-edit' WHERE id = 4;
UPDATE sys_menu SET icon = 'icon-menu' WHERE id = 5;
UPDATE sys_menu SET icon = 'icon-apps' WHERE id = 6;
UPDATE sys_menu SET icon = 'icon-code' WHERE id = 7;
UPDATE sys_menu SET icon = 'icon-tool' WHERE id = 9;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 10;
UPDATE sys_menu SET icon = 'icon-lock' WHERE id = 13;
UPDATE sys_menu SET icon = 'icon-link' WHERE id = 14;
UPDATE sys_menu SET icon = 'icon-dashboard' WHERE id = 15;

-- C端用户管理
UPDATE sys_menu SET icon = 'icon-user-group' WHERE id = 99;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 991;
UPDATE sys_menu SET icon = 'icon-idcard' WHERE id = 992;
UPDATE sys_menu SET icon = 'icon-heart' WHERE id = 993;
UPDATE sys_menu SET icon = 'icon-star' WHERE id = 994;
UPDATE sys_menu SET icon = 'icon-location' WHERE id = 995;
UPDATE sys_menu SET icon = 'icon-calendar' WHERE id = 996;
UPDATE sys_menu SET icon = 'icon-heart' WHERE id = 997;

-- 微信管理
UPDATE sys_menu SET icon = 'icon-wechat' WHERE id = 200;
UPDATE sys_menu SET icon = 'icon-message' WHERE id = 2001;
UPDATE sys_menu SET icon = 'icon-apps' WHERE id = 2002;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 2003;
UPDATE sys_menu SET icon = 'icon-file' WHERE id = 2004;

-- 短信管理
UPDATE sys_menu SET icon = 'icon-message' WHERE id = 210;
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2101;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 2102;
UPDATE sys_menu SET icon = 'icon-file' WHERE id = 2103;

-- 支付管理
UPDATE sys_menu SET icon = 'icon-credit-card' WHERE id = 220;
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2201;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2203;

-- 订单管理
UPDATE sys_menu SET icon = 'icon-shopping-cart' WHERE id = 230;
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2301;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 2302;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2303;

-- 物流管理
UPDATE sys_menu SET icon = 'icon-car' WHERE id = 240;
UPDATE sys_menu SET icon = 'icon-home' WHERE id = 2401;
UPDATE sys_menu SET icon = 'icon-search' WHERE id = 2402;
UPDATE sys_menu SET icon = 'icon-location' WHERE id = 2403;

-- OSS对象存储
UPDATE sys_menu SET icon = 'icon-cloud' WHERE id = 250;
UPDATE sys_menu SET icon = 'icon-folder' WHERE id = 2501;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 2502;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2503;

-- ROLLBACK:
-- UPDATE sys_menu SET status = '1' WHERE deleted_at IS NULL;
