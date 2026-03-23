-- m20260322_010_update_menu_icons.sql
-- 描述：更新菜单图标为合适的图标
-- 作者：tuoke
-- 日期：2026-03-22

-- =====================================================
-- 一、一级菜单图标更新（添加icon-前缀）
-- =====================================================
UPDATE sys_menu SET icon = 'icon-home' WHERE id = 1;
UPDATE sys_menu SET icon = 'icon-table' WHERE id = 3;
UPDATE sys_menu SET icon = 'icon-form' WHERE id = 4;
UPDATE sys_menu SET icon = 'icon-menu-fold' WHERE id = 5;
UPDATE sys_menu SET icon = 'icon-apps' WHERE id = 6;
UPDATE sys_menu SET icon = 'icon-code' WHERE id = 7;
UPDATE sys_menu SET icon = 'icon-tool' WHERE id = 9;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 10;
UPDATE sys_menu SET icon = 'icon-eye-invisible' WHERE id = 11;
UPDATE sys_menu SET icon = 'icon-eye' WHERE id = 12;
UPDATE sys_menu SET icon = 'icon-shield' WHERE id = 13;
UPDATE sys_menu SET icon = 'icon-link' WHERE id = 14;
UPDATE sys_menu SET icon = 'icon-dashboard' WHERE id = 15;
UPDATE sys_menu SET icon = 'icon-earth' WHERE id = 17;
UPDATE sys_menu SET icon = 'icon-info-circle' WHERE id = 18;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 99;

-- =====================================================
-- 二、二级菜单图标更新
-- =====================================================

-- 表格模块
UPDATE sys_menu SET icon = 'icon-table' WHERE id IN (301, 302);

-- 表单模块
UPDATE sys_menu SET icon = 'icon-form' WHERE id IN (401, 402);
-- 多级菜单
UPDATE sys_menu SET icon = 'icon-menu-fold' WHERE id IN (501, 502, 50201, 50202, 50203);
-- 组件模块
UPDATE sys_menu SET icon = 'icon-play-circle' WHERE id = 601;
UPDATE sys_menu SET icon = 'icon-printer' WHERE id = 602;
UPDATE sys_menu SET icon = 'icon-drag-dot-vertical' WHERE id = 603;
UPDATE sys_menu SET icon = 'icon-edit' WHERE id = 604;
UPDATE sys_menu SET icon = 'icon-question-circle' WHERE id = 605;
UPDATE sys_menu SET icon = 'icon-apps' WHERE id = 606;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 607;
UPDATE sys_menu SET icon = 'icon-fingerprint' WHERE id = 608;
UPDATE sys_menu SET icon = 'icon-barcode' WHERE id = 609;
UPDATE sys_menu SET icon = 'icon-qrcode' WHERE id = 610;
UPDATE sys_menu SET icon = 'icon-font-colors' WHERE id = 611;
UPDATE sys_menu SET icon = 'icon-mic' WHERE id = 612;
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 613;
UPDATE sys_menu SET icon = 'icon-layout' WHERE id = 614;
-- 指令模块
UPDATE sys_menu SET icon = 'icon-thunderbolt' WHERE id = 701;
UPDATE sys_menu SET icon = 'icon-pause-circle' WHERE id = 702;
UPDATE sys_menu SET icon = 'icon-bug' WHERE id = 703;
-- 功能模块
UPDATE sys_menu SET icon = 'icon-route' WHERE id = 901;
UPDATE sys_menu SET icon = 'icon-tool' WHERE id = 902;
UPDATE sys_menu SET icon = 'icon-tree' WHERE id = 903;
UPDATE sys_menu SET icon = 'icon-file' WHERE id = 904;
UPDATE sys_menu SET icon = 'icon-check-circle' WHERE id = 905;
-- 系统管理
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 1001;
UPDATE sys_menu SET icon = 'icon-user-group' WHERE id = 1002;
UPDATE sys_menu SET icon = 'icon-menu' WHERE id = 1003;
UPDATE sys_menu SET icon = 'icon-organization' WHERE id = 1004;
UPDATE sys_menu SET icon = 'icon-book' WHERE id = 1005;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 1006;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 1007;
-- 权限管理
UPDATE sys_menu SET icon = 'icon-check-square' WHERE id = 1301;
UPDATE sys_menu SET icon = 'icon-lock' WHERE id = 1302;
UPDATE sys_menu SET icon = 'icon-unlock' WHERE id = 1303;
-- 系统监控
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 1501;
UPDATE sys_menu SET icon = 'icon-clock-circle' WHERE id = 1502;
UPDATE sys_menu SET icon = 'icon-history' WHERE id = 1503;
-- 内链模块
UPDATE sys_menu SET icon = 'icon-palette' WHERE id = 140101;
UPDATE sys_menu SET icon = 'icon-color-picker' WHERE id = 140102;
UPDATE sys_menu SET icon = 'icon-grid' WHERE id = 140103;
UPDATE sys_menu SET icon = 'icon-map' WHERE id = 140104;
UPDATE sys_menu SET icon = 'icon-book' WHERE id = 140201;
UPDATE sys_menu SET icon = 'icon-code' WHERE id = 140202;
UPDATE sys_menu SET icon = 'icon-thunderbolt' WHERE id = 140203;
UPDATE sys_menu SET icon = 'icon-github' WHERE id = 140204;
UPDATE sys_menu SET icon = 'icon-fire' WHERE id = 140205;

SELECT '菜单图标更新完成' as result;
