-- m20260322_009_c_missing_menu_permission.sql
-- 描述：补充缺失的菜单和权限配置（微信/短信/支付/订单/物流/OSS/系统设置）
-- 作者：tuoke
-- 日期：2026-03-22

-- =====================================================
-- 一、微信管理菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(200, 0, 'wechat', '微信管理', 'menu.wechat', '/wechat', NULL, '/wechat/mp', 'icon-wechat', 'M', '0', '0', '1', 200, NOW(), NOW()),
(2001, 200, 'wechat_mp', '公众号管理', 'menu.wechat.mp', '/wechat/mp', 'wechat/mp', NULL, 'icon-message', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2002, 200, 'wechat_mini', '小程序管理', 'menu.wechat.mini', '/wechat/mini', 'wechat/mini', NULL, 'icon-apps', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2003, 200, 'wechat_oauth', '授权记录', 'menu.wechat.oauth', '/wechat/oauth', 'wechat/oauth', NULL, 'icon-user', 'C', '0', '0', '1', 3, NOW(), NOW()),
(2004, 200, 'wechat_template', '消息模板', 'menu.wechat.template', '/wechat/template', 'wechat/template', NULL, 'icon-file', 'C', '0', '0', '1', 4, NOW(), NOW());

-- 微信管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(20011, 2001, 'wechat_mp_add', '新增公众号', '', 'F', '0', '1', 'wechat:mp:add', 1, NOW(), NOW()),
(20012, 2001, 'wechat_mp_edit', '编辑公众号', '', 'F', '0', '1', 'wechat:mp:edit', 2, NOW(), NOW()),
(20013, 2001, 'wechat_mp_del', '删除公众号', '', 'F', '0', '1', 'wechat:mp:del', 3, NOW(), NOW()),
(20021, 2002, 'wechat_mini_add', '新增小程序', '', 'F', '0', '1', 'wechat:mini:add', 1, NOW(), NOW()),
(20022, 2002, 'wechat_mini_edit', '编辑小程序', '', 'F', '0', '1', 'wechat:mini:edit', 2, NOW(), NOW()),
(20023, 2002, 'wechat_mini_del', '删除小程序', '', 'F', '0', '1', 'wechat:mini:del', 3, NOW(), NOW());

-- =====================================================
-- 二、短信管理菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(210, 0, 'sms', '短信管理', 'menu.sms', '/sms', NULL, '/sms/list', 'icon-message', 'M', '0', '0', '1', 210, NOW(), NOW()),
(2101, 210, 'sms_list', '短信记录', 'menu.sms.list', '/sms/list', 'sms/list', NULL, 'icon-list', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2102, 210, 'sms_config', '短信配置', 'menu.sms.config', '/sms/config', 'sms/config', NULL, 'icon-settings', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2103, 210, 'sms_template', '短信模板', 'menu.sms.template', '/sms/template', 'sms/template', NULL, 'icon-file', 'C', '0', '0', '1', 3, NOW(), NOW());

-- 短信管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(21011, 2101, 'sms_send', '发送短信', '', 'F', '0', '1', 'sms:send', 1, NOW(), NOW()),
(21012, 2101, 'sms_view', '查看详情', '', 'F', '0', '1', 'sms:view', 2, NOW(), NOW()),
(21021, 2102, 'sms_config_add', '新增配置', '', 'F', '0', '1', 'sms:config:add', 1, NOW(), NOW()),
(21022, 2102, 'sms_config_edit', '编辑配置', '', 'F', '0', '1', 'sms:config:edit', 2, NOW(), NOW()),
(21031, 2103, 'sms_template_add', '新增模板', '', 'F', '0', '1', 'sms:template:add', 1, NOW(), NOW()),
(21032, 2103, 'sms_template_edit', '编辑模板', '', 'F', '0', '1', 'sms:template:edit', 2, NOW(), NOW()),
(21033, 2103, 'sms_template_del', '删除模板', '', 'F', '0', '1', 'sms:template:del', 3, NOW(), NOW());

-- =====================================================
-- 三、支付管理菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(220, 0, 'payment', '支付管理', 'menu.payment', '/payment', NULL, '/payment/order', 'icon-credit-card', 'M', '0', '0', '1', 220, NOW(), NOW()),
(2201, 220, 'payment_order', '支付订单', 'menu.payment.order', '/payment/order', 'payment/order', NULL, 'icon-list', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2202, 220, 'payment_refund', '退款管理', 'menu.payment.refund', '/payment/refund', 'payment/refund', NULL, 'icon-rollback', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2203, 220, 'payment_statistics', '支付统计', 'menu.payment.statistics', '/payment/statistics', 'payment/statistics', NULL, 'icon-bar-chart', 'C', '0', '0', '1', 3, NOW(), NOW());

-- 支付管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(22011, 2201, 'payment_order_view', '查看详情', '', 'F', '0', '1', 'payment:order:view', 1, NOW(), NOW()),
(22012, 2201, 'payment_order_close', '关闭订单', '', 'F', '0', '1', 'payment:order:close', 2, NOW(), NOW()),
(22021, 2202, 'payment_refund_audit', '退款审核', '', 'F', '0', '1', 'payment:refund:audit', 1, NOW(), NOW()),
(22022, 2202, 'payment_refund_confirm', '确认退款', '', 'F', '0', '1', 'payment:refund:confirm', 2, NOW(), NOW());

-- =====================================================
-- 四、订单管理菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(230, 0, 'order', '订单管理', 'menu.order', '/order', NULL, '/order/list', 'icon-shopping-cart', 'M', '0', '0', '1', 230, NOW(), NOW()),
(2301, 230, 'order_list', '订单列表', 'menu.order.list', '/order/list', 'order/list', NULL, 'icon-list', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2302, 230, 'order_detail', '订单详情', 'menu.order.detail', '/order/detail', 'order/detail', NULL, 'icon-file-text', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2303, 230, 'order_statistics', '订单统计', 'menu.order.statistics', '/order/statistics', 'order/statistics', NULL, 'icon-bar-chart', 'C', '0', '0', '1', 3, NOW(), NOW());

-- 订单管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(23011, 2301, 'order_view', '查看详情', '', 'F', '0', '1', 'order:view', 1, NOW(), NOW()),
(23012, 2301, 'order_ship', '订单发货', '', 'F', '0', '1', 'order:ship', 2, NOW(), NOW()),
(23013, 2301, 'order_cancel', '取消订单', '', 'F', '0', '1', 'order:cancel', 3, NOW(), NOW()),
(23014, 2301, 'order_export', '导出订单', '', 'F', '0', '1', 'order:export', 4, NOW(), NOW());

-- =====================================================
-- 五、物流管理菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(240, 0, 'logistics', '物流管理', 'menu.logistics', '/logistics', NULL, '/logistics/company', 'icon-car', 'M', '0', '0', '1', 240, NOW(), NOW()),
(2401, 240, 'logistics_company', '物流公司', 'menu.logistics.company', '/logistics/company', 'logistics/company', NULL, 'icon-home', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2402, 240, 'logistics_query', '物流查询', 'menu.logistics.query', '/logistics/query', 'logistics/query', NULL, 'icon-search', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2403, 240, 'logistics_track', '物流跟踪', 'menu.logistics.track', '/logistics/track', 'logistics/track', NULL, 'icon-location', 'C', '0', '0', '1', 3, NOW(), NOW());

-- 物流管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(24011, 2401, 'logistics_company_add', '新增公司', '', 'F', '0', '1', 'logistics:company:add', 1, NOW(), NOW()),
(24012, 2401, 'logistics_company_edit', '编辑公司', '', 'F', '0', '1', 'logistics:company:edit', 2, NOW(), NOW()),
(24013, 2401, 'logistics_company_del', '删除公司', '', 'F', '0', '1', 'logistics:company:del', 3, NOW(), NOW());

-- =====================================================
-- 六、OSS对象存储菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(250, 0, 'oss', '对象存储', 'menu.oss', '/oss', NULL, '/oss/file', 'icon-cloud', 'M', '0', '0', '1', 250, NOW(), NOW()),
(2501, 250, 'oss_file', '文件管理', 'menu.oss.file', '/oss/file', 'oss/file', NULL, 'icon-folder', 'C', '0', '0', '1', 1, NOW(), NOW()),
(2502, 250, 'oss_config', '存储配置', 'menu.oss.config', '/oss/config', 'oss/config', NULL, 'icon-settings', 'C', '0', '0', '1', 2, NOW(), NOW()),
(2503, 250, 'oss_statistics', '存储统计', 'menu.oss.statistics', '/oss/statistics', 'oss/statistics', NULL, 'icon-bar-chart', 'C', '0', '0', '1', 3, NOW(), NOW());

-- OSS管理按钮权限
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `path`, `menu_type`, `hidden`, `status`, `perms`, `order`, `created_at`, `updated_at`) VALUES
(25011, 2501, 'oss_file_upload', '上传文件', '', 'F', '0', '1', 'oss:file:upload', 1, NOW(), NOW()),
(25012, 2501, 'oss_file_del', '删除文件', '', 'F', '0', '1', 'oss:file:del', 2, NOW(), NOW()),
(25013, 2501, 'oss_file_download', '下载文件', '', 'F', '0', '1', 'oss:file:download', 3, NOW(), NOW());

-- =====================================================
-- 七、系统设置补充菜单
-- =====================================================
INSERT IGNORE INTO `sys_menu` (`id`, `pid`, `name`, `title`, `i18nkey`, `path`, `component`, `redirect`, `icon`, `menu_type`, `no_cache`, `hidden`, `status`, `order`, `created_at`, `updated_at`) VALUES
(1008, 10, 'system_setting', '系统设置', 'menu.system.setting', '/system/setting', 'system/setting/setting', NULL, 'icon-settings', 'C', '0', '0', '1', 8, NOW(), NOW()),
(1009, 10, 'system_server', '服务器监控', 'menu.system.server', '/system/server', 'system/server/server', NULL, 'icon-desktop', 'C', '0', '0', '1', 9, NOW(), NOW()),
(1010, 10, 'system_cache', '缓存管理', 'menu.system.cache', '/system/cache', 'system/cache/cache', NULL, 'icon-storage', 'C', '0', '0', '1', 10, NOW(), NOW());

-- =====================================================
-- 八、角色菜单关联（给管理员角色分配新菜单权限）
-- =====================================================
INSERT IGNORE INTO `sys_role_menu` (`role_id`, `menu_id`) VALUES
-- 微信管理
(1, 200), (1, 2001), (1, 2002), (1, 2003), (1, 2004),
(1, 20011), (1, 20012), (1, 20013), (1, 20021), (1, 20022), (1, 20023),
-- 短信管理
(1, 210), (1, 2101), (1, 2102), (1, 2103),
(1, 21011), (1, 21012), (1, 21021), (1, 21022), (1, 21031), (1, 21032), (1, 21033),
-- 支付管理
(1, 220), (1, 2201), (1, 2202), (1, 2203),
(1, 22011), (1, 22012), (1, 22021), (1, 22022),
-- 订单管理
(1, 230), (1, 2301), (1, 2302), (1, 2303),
(1, 23011), (1, 23012), (1, 23013), (1, 23014),
-- 物流管理
(1, 240), (1, 2401), (1, 2402), (1, 2403),
(1, 24011), (1, 24012), (1, 24013),
-- OSS对象存储
(1, 250), (1, 2501), (1, 2502), (1, 2503),
(1, 25011), (1, 25012), (1, 25013),
-- 系统设置补充
(1, 1008), (1, 1009), (1, 1010);

-- =====================================================
-- 九、API权限配置
-- =====================================================
INSERT IGNORE INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`) VALUES
-- 微信相关API
(9001, '/wechat/auth-url', 'GET', '微信授权URL', '0', '获取微信授权URL', 1, NOW()),
(9002, '/wechat/callback', 'GET', '微信授权回调', '0', '微信授权回调', 2, NOW()),
(9003, '/wechat/mini/login', 'POST', '小程序登录', '0', '小程序登录', 3, NOW()),
(9004, '/wechat/mp/list', 'GET', '公众号列表', '0', '公众号列表', 4, NOW()),
(9005, '/wechat/mini/list', 'GET', '小程序列表', '0', '小程序列表', 5, NOW()),
-- 短信相关API
(9011, '/sms/send-code', 'POST', '发送验证码', '0', '发送验证码', 11, NOW()),
(9012, '/sms/verify-code', 'POST', '验证验证码', '0', '验证验证码', 12, NOW()),
(9013, '/sms/logs', 'GET', '短信日志', '0', '短信日志列表', 13, NOW()),
(9014, '/sms/config/list', 'GET', '短信配置', '0', '短信配置列表', 14, NOW()),
(9015, '/sms/template/list', 'GET', '短信模板', '0', '短信模板列表', 15, NOW()),
-- 支付相关API
(9021, '/payment/create', 'POST', '创建支付', '0', '创建支付订单', 21, NOW()),
(9022, '/payment/list', 'GET', '支付列表', '0', '支付订单列表', 22, NOW()),
(9023, '/payment/refund', 'POST', '退款', '0', '退款', 23, NOW()),
(9024, '/payment/statistics', 'GET', '支付统计', '0', '支付统计', 24, NOW()),
-- 订单相关API
(9031, '/order/list', 'GET', '订单列表', '0', '订单列表', 31, NOW()),
(9032, '/order/detail/{id}', 'GET', '订单详情', '0', '订单详情', 32, NOW()),
(9033, '/order/ship', 'POST', '订单发货', '0', '订单发货', 33, NOW()),
(9034, '/order/cancel', 'POST', '取消订单', '0', '取消订单', 34, NOW()),
-- 物流相关API
(9041, '/logistics/company/list', 'GET', '物流公司', '0', '物流公司列表', 41, NOW()),
(9042, '/logistics/query', 'GET', '物流查询', '0', '物流查询', 42, NOW()),
(9043, '/logistics/track', 'GET', '物流跟踪', '0', '物流跟踪', 43, NOW()),
-- OSS相关API
(9051, '/oss/file/list', 'GET', '文件列表', '0', '文件列表', 51, NOW()),
(9052, '/oss/file/upload', 'POST', '文件上传', '0', '文件上传', 52, NOW()),
(9053, '/oss/file/delete', 'DELETE', '文件删除', '0', '文件删除', 53, NOW()),
(9054, '/oss/config/list', 'GET', '存储配置', '0', '存储配置列表', 54, NOW());

-- ROLLBACK:
-- DELETE FROM sys_api_permission WHERE id >= 9001 AND id <= 9054;
-- DELETE FROM sys_role_menu WHERE menu_id >= 200 AND menu_id <= 2503;
-- DELETE FROM sys_menu WHERE id >= 200 AND id <= 2503;
-- DELETE FROM sys_menu WHERE id IN (1008, 1009, 1010);
