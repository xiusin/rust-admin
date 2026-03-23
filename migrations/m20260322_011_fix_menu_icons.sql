-- m20260322_011_fix_menu_icons.sql
-- 描述：修复所有菜单图标，使用Arco Design图标格式
-- 作者：tuoke
-- 日期：2026-03-22

-- =====================================================
-- 一、一级菜单图标修复（使用icon-前缀）
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
UPDATE sys_menu SET icon = 'icon-shopping-bag' WHERE id = 99;
UPDATE sys_menu SET icon = 'icon-wechat' WHERE id = 200;
UPDATE sys_menu SET icon = 'icon-message' WHERE id = 210;
UPDATE sys_menu SET icon = 'icon-credit-card' WHERE id = 220;
UPDATE sys_menu SET icon = 'icon-shopping-cart' WHERE id = 230;
UPDATE sys_menu SET icon = 'icon-car' WHERE id = 240;
UPDATE sys_menu SET icon = 'icon-cloud' WHERE id = 250;

-- =====================================================
-- 二、二级菜单图标修复
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
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 1008;
UPDATE sys_menu SET icon = 'icon-desktop' WHERE id = 1009;
UPDATE sys_menu SET icon = 'icon-storage' WHERE id = 1010;

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

-- =====================================================
-- 三、电商业务模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-money' WHERE id = 9901;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 9902;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 9903;
UPDATE sys_menu SET icon = 'icon-truck' WHERE id = 9905;
UPDATE sys_menu SET icon = 'icon-rollback' WHERE id = 9906;
UPDATE sys_menu SET icon = 'icon-star' WHERE id = 9907;
UPDATE sys_menu SET icon = 'icon-tag' WHERE id = 9908;
UPDATE sys_menu SET icon = 'icon-stop' WHERE id = 9909;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 9910;

-- =====================================================
-- 四、微信管理模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-message' WHERE id = 2001;
UPDATE sys_menu SET icon = 'icon-apps' WHERE id = 2002;
UPDATE sys_menu SET icon = 'icon-user' WHERE id = 2003;
UPDATE sys_menu SET icon = 'icon-file' WHERE id = 2004;

-- =====================================================
-- 五、短信管理模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2101;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 2102;
UPDATE sys_menu SET icon = 'icon-file' WHERE id = 2103;

-- =====================================================
-- 六、支付管理模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2201;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2203;

-- =====================================================
-- 七、订单管理模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-list' WHERE id = 2301;
UPDATE sys_menu SET icon = 'icon-file-text' WHERE id = 2302;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2303;

-- =====================================================
-- 八、物流管理模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-home' WHERE id = 2401;
UPDATE sys_menu SET icon = 'icon-search' WHERE id = 2402;
UPDATE sys_menu SET icon = 'icon-location' WHERE id = 2403;

-- =====================================================
-- 九、OSS对象存储模块二级菜单图标
-- =====================================================
UPDATE sys_menu SET icon = 'icon-folder' WHERE id = 2501;
UPDATE sys_menu SET icon = 'icon-settings' WHERE id = 2502;
UPDATE sys_menu SET icon = 'icon-bar-chart' WHERE id = 2503;

SELECT '所有菜单图标已更新完成' as result;
