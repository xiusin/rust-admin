-- m20260325_001_plugin_market_menu_permission.sql
-- 描述：插件市场模块菜单和权限配置
-- 作者：tuoke
-- 日期：2026-03-25

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

-- =====================================================
-- 1. 插件市场目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2800, 'plugin-market', '插件市场', 0, 25, '/plugin', 'M', '1', NULL, 'app', NOW(), '插件市场目录');

-- =====================================================
-- 2. 插件市场首页
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2801, 'plugin-home', '市场首页', 2800, 1, '/plugin/home', 'plugin-market/index', 'C', '1', 'plugin:home:list', 'home', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2802, 'plugin-list-page', '插件列表', 2800, 2, '/plugin/list', 'plugin-market/list', 'C', '1', 'plugin:list:list', 'list', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2803, 'plugin-search-page', '搜索插件', 2800, 3, '/plugin/search', 'plugin-market/search', 'C', '1', 'plugin:search:list', 'search', NOW());

-- =====================================================
-- 3. 我的插件目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2810, 'my-plugins', '我的插件', 2800, 4, '/plugin/my', 'M', '1', NULL, 'user', NOW(), '我的插件目录');

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2811, 'my-purchased', '已购插件', 2810, 1, '/plugin/my/purchased', 'plugin-my/purchased', 'C', '1', 'plugin:purchased:list', 'shopping-bag', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2812, 'my-subscription', '订阅管理', 2810, 2, '/plugin/my/subscription', 'plugin-my/subscription', 'C', '1', 'plugin:subscription:list', 'calendar', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2813, 'my-license', '许可证管理', 2810, 3, '/plugin/my/license', 'plugin-my/license', 'C', '1', 'plugin:license:list', 'key', NOW());

-- =====================================================
-- 4. 开发者中心目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2820, 'plugin-developer', '开发者中心', 2800, 5, '/plugin/developer', 'M', '1', NULL, 'code', NOW(), '开发者中心目录');

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2821, 'developer-dashboard', '开发者仪表板', 2820, 1, '/plugin/developer/dashboard', 'plugin-developer/dashboard', 'C', '1', 'plugin:developer:dashboard', 'dashboard', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2822, 'developer-plugins', '插件管理', 2820, 2, '/plugin/developer/plugins', 'plugin-developer/plugin-list', 'C', '1', 'plugin:developer:list', 'store', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2823, 'developer-edit', '插件编辑', 2820, 3, '/plugin/developer/edit', 'plugin-developer/plugin-edit', 'C', '1', 'plugin:developer:edit', 'edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2824, 'developer-versions', '版本管理', 2820, 4, '/plugin/developer/versions', 'plugin-developer/version', 'C', '1', 'plugin:developer:version', 'history', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2825, 'developer-sales', '销售统计', 2820, 5, '/plugin/developer/sales', 'plugin-developer/sales', 'C', '1', 'plugin:developer:sales', 'chart', NOW());

-- =====================================================
-- 5. 订单中心目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2830, 'order-center', '订单中心', 2800, 6, '/plugin/order', 'M', '1', NULL, 'order', NOW(), '订单中心目录');

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2831, 'order-cart', '购物车', 2830, 1, '/plugin/order/cart', 'order/cart', 'C', '1', 'plugin:cart:list', 'cart', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2832, 'order-checkout', '结算页', 2830, 2, '/plugin/order/checkout', 'order/checkout', 'C', '1', 'plugin:checkout:list', 'payment', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2833, 'order-list-page', '订单列表', 2830, 3, '/plugin/order/list', 'order/list', 'C', '1', 'plugin:order:list', 'list', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2834, 'order-detail-page', '订单详情', 2830, 4, '/plugin/order/detail', 'order/detail', 'C', '1', 'plugin:order:detail', 'file-text', NOW());

-- =====================================================
-- 6. 验证中心目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2840, 'verify-center', '验证中心', 2800, 7, '/plugin/verify', 'M', '1', NULL, 'shield', NOW(), '验证中心目录');

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2841, 'verify-activate', '插件激活', 2840, 1, '/plugin/verify/activate', 'verify/activate', 'C', '1', 'plugin:verify:activate', 'check-circle', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2842, 'verify-device', '设备管理', 2840, 2, '/plugin/verify/device', 'verify/device', 'C', '1', 'plugin:verify:device', 'mobile', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2843, 'verify-card', '卡密兑换', 2840, 3, '/plugin/verify/card', 'verify/card-redeem', 'C', '1', 'plugin:verify:card', 'gift', NOW());

-- =====================================================
-- 7. 插件管理权限（后台管理）
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2850, 'plugin-admin', '插件管理', 0, 26, '/admin/plugin', 'M', '1', NULL, 'plug', NOW());

-- 插件分类管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2851, 'plugin-category-admin', '插件分类', 2850, 1, '/admin/plugin/category', 'plugin-market/category', 'C', '1', 'plugin:category:list', 'folder', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285101, 'plugin-category-query', '分类查询', 2851, 1, 'F', '1', 'plugin:category:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285102, 'plugin-category-create', '分类新增', 2851, 2, 'F', '1', 'plugin:category:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285103, 'plugin-category-edit', '分类编辑', 2851, 3, 'F', '1', 'plugin:category:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285104, 'plugin-category-delete', '分类删除', 2851, 4, 'F', '1', 'plugin:category:delete', NOW());

-- 插件审核管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2852, 'plugin-audit-admin', '插件审核', 2850, 2, '/admin/plugin/audit', 'plugin-market/audit', 'C', '1', 'plugin:audit:list', 'check-circle', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285201, 'plugin-audit-query', '审核查询', 2852, 1, 'F', '1', 'plugin:audit:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285202, 'plugin-audit-pass', '审核通过', 2852, 2, 'F', '1', 'plugin:audit:pass', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285203, 'plugin-audit-reject', '审核拒绝', 2852, 3, 'F', '1', 'plugin:audit:reject', NOW());

-- 订单管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2853, 'plugin-order-admin', '订单管理', 2850, 3, '/admin/plugin/order', 'plugin-market/order-admin', 'C', '1', 'plugin:order:list', 'order', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285301, 'plugin-order-query', '订单查询', 2853, 1, 'F', '1', 'plugin:order:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285302, 'plugin-order-refund', '订单退款', 2853, 2, 'F', '1', 'plugin:order:refund', NOW());

-- 卡密管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2854, 'plugin-card-admin', '卡密管理', 2850, 4, '/admin/plugin/card', 'plugin-market/card-admin', 'C', '1', 'plugin:card:list', 'gift', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285401, 'plugin-card-generate', '生成卡密', 2854, 1, 'F', '1', 'plugin:card:generate', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285402, 'plugin-card-export', '导出卡密', 2854, 2, 'F', '1', 'plugin:card:export', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285403, 'plugin-card-freeze', '冻结卡密', 2854, 3, 'F', '1', 'plugin:card:freeze', NOW());

-- 开发者管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2855, 'plugin-developer-admin', '开发者管理', 2850, 5, '/admin/plugin/developer', 'plugin-market/developer-admin', 'C', '1', 'plugin:developer:list', 'user', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285501, 'plugin-developer-query', '开发者查询', 2855, 1, 'F', '1', 'plugin:developer:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285502, 'plugin-developer-audit', '开发者审核', 2855, 2, 'F', '1', 'plugin:developer:audit', NOW());

-- 许可证管理
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2856, 'plugin-license-admin', '许可证管理', 2850, 6, '/admin/plugin/license', 'plugin-market/license-admin', 'C', '1', 'plugin:license:list', 'key', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285601, 'plugin-license-query', '许可证查询', 2856, 1, 'F', '1', 'plugin:license:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285602, 'plugin-license-revoke', '许可证吊销', 2856, 2, 'F', '1', 'plugin:license:revoke', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (285603, 'plugin-license-unbind', '设备解绑', 2856, 3, 'F', '1', 'plugin:license:unbind', NOW());

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 2800 AND id < 2900;
