-- m20260322_015_reorganize_menu_structure.sql
-- 描述：重新规划菜单结构，修正命名规范
-- 作者：tuoke
-- 日期：2026-03-22

-- ============================================
-- 一、修正支付管理模块
-- ============================================
-- 删除原有的支付订单相关菜单（支付订单应该在订单管理中查看）
DELETE FROM sys_role_menu WHERE menu_id IN (2201, 2203, 22011, 22012, 22021, 22022);
DELETE FROM sys_menu WHERE id IN (2201, 2203, 22011, 22012, 22021, 22022);

-- 重新添加支付管理子菜单（支付渠道管理）
INSERT INTO sys_menu (id, pid, name, title, path, component, menu_type, icon, `order`, status, created_at) VALUES
(2201, 220, 'payment_channel', '支付渠道', '/payment/channel', 'payment/channel', 'C', 'icon-credit-card', 1, '0', NOW()),
(2202, 220, 'payment_record', '支付记录', '/payment/record', 'payment/record', 'C', 'icon-list', 2, '0', NOW()),
(2203, 220, 'payment_refund', '退款管理', '/payment/refund', 'payment/refund', 'C', 'icon-rollback', 3, '0', NOW()),
(2204, 220, 'payment_statistics', '支付统计', '/payment/statistics', 'payment/statistics', 'C', 'icon-bar-chart', 4, '0', NOW());

-- 支付渠道按钮权限
INSERT INTO sys_menu (id, pid, name, title, menu_type, `order`, status, created_at) VALUES
(22011, 2201, 'payment_channel_add', '新增渠道', 'F', 1, '0', NOW()),
(22012, 2201, 'payment_channel_edit', '编辑渠道', 'F', 2, '0', NOW()),
(22013, 2201, 'payment_channel_del', '删除渠道', 'F', 3, '0', NOW()),
(22014, 2201, 'payment_channel_config', '渠道配置', 'F', 4, '0', NOW());

-- 支付记录按钮权限
INSERT INTO sys_menu (id, pid, name, title, menu_type, `order`, status, created_at) VALUES
(22021, 2202, 'payment_record_view', '查看详情', 'F', 1, '0', NOW()),
(22022, 2202, 'payment_record_export', '导出记录', 'F', 2, '0', NOW());

-- 退款管理按钮权限
INSERT INTO sys_menu (id, pid, name, title, menu_type, `order`, status, created_at) VALUES
(22031, 2203, 'payment_refund_audit', '退款审核', 'F', 1, '0', NOW()),
(22032, 2203, 'payment_refund_confirm', '确认退款', 'F', 2, '0', NOW());

-- ============================================
-- 二、修正订单管理模块
-- ============================================
-- 删除独立的一级菜单 order_detail
DELETE FROM sys_role_menu WHERE menu_id = 2302;
DELETE FROM sys_menu WHERE id = 2302;

-- 订单详情作为订单列表的子页面，不需要单独菜单

-- ============================================
-- 三、修正微信管理模块命名
-- ============================================
UPDATE sys_menu SET name = 'wechat_mp', title = '公众号管理' WHERE id = 2001;
UPDATE sys_menu SET name = 'wechat_mini', title = '小程序管理' WHERE id = 2002;
UPDATE sys_menu SET name = 'wechat_oauth', title = '授权记录' WHERE id = 2003;
UPDATE sys_menu SET name = 'wechat_template', title = '消息模板' WHERE id = 2004;

-- ============================================
-- 四、修正短信管理模块命名
-- ============================================
UPDATE sys_menu SET name = 'sms_record', title = '短信记录' WHERE id = 2101;
UPDATE sys_menu SET name = 'sms_config', title = '短信配置' WHERE id = 2102;
UPDATE sys_menu SET name = 'sms_template', title = '短信模板' WHERE id = 2103;

-- ============================================
-- 五、修正物流管理模块命名
-- ============================================
UPDATE sys_menu SET name = 'logistics_company', title = '物流公司' WHERE id = 2401;
UPDATE sys_menu SET name = 'logistics_query', title = '物流查询' WHERE id = 2402;
UPDATE sys_menu SET name = 'logistics_track', title = '物流跟踪' WHERE id = 2403;

-- ============================================
-- 六、修正OSS管理模块命名
-- ============================================
UPDATE sys_menu SET name = 'oss_file', title = '文件管理' WHERE id = 2501;
UPDATE sys_menu SET name = 'oss_config', title = '存储配置' WHERE id = 2502;
UPDATE sys_menu SET name = 'oss_statistics', title = '存储统计' WHERE id = 2503;

-- ============================================
-- 七、为超级管理员角色添加新菜单权限
-- ============================================
INSERT INTO sys_role_menu (role_id, menu_id) 
SELECT 1, id FROM sys_menu WHERE id >= 2201 AND id <= 22032 AND deleted_at IS NULL
ON DUPLICATE KEY UPDATE role_id = role_id;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id BETWEEN 2201 AND 22032;
