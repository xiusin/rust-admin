-- m20260322_005_c_user_extension_menu_permission.sql
-- 描述：C端用户扩展功能菜单和权限（售后、等级、标签、禁用、统计）
-- 作者：tuoke
-- 日期：2026-03-22

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

-- =====================================================
-- 一、售后管理模块
-- =====================================================

-- 售后管理菜单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (9906, 'consumer-after-sale', '售后管理', NULL, 99, 6, '/consumer/after-sale', 'consumer/after-sale', '', '', '1', 'C', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:list', 'refund', NOW(), NULL, NULL, 'C端售后管理', NULL, NULL);

-- 售后管理按钮权限
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990601, 'consumer-afterSale-query', '售后查询', NULL, 9906, 1, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:query', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990602, 'consumer-afterSale-apply', '申请售后', NULL, 9906, 2, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:apply', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990603, 'consumer-afterSale-audit', '审核售后', NULL, 9906, 3, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:audit', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990604, 'consumer-afterSale-close', '关闭售后', NULL, 9906, 4, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:close', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990605, 'consumer-afterSale-refund', '退款处理', NULL, 9906, 5, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:refund', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990606, 'consumer-afterSale-logistics', '物流管理', NULL, 9906, 6, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:logistics', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990607, 'consumer-afterSale-export', '导出售后', NULL, 9906, 7, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:afterSale:export', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 二、用户等级管理模块
-- =====================================================

-- 用户等级管理菜单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (9907, 'consumer-level', '等级管理', NULL, 99, 7, '/consumer/level', 'consumer/level', '', '', '1', 'C', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:level:list', 'star', NOW(), NULL, NULL, 'C端用户等级管理', NULL, NULL);

-- 用户等级按钮权限
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990701, 'consumer-level-query', '等级查询', NULL, 9907, 1, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:level:query', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990702, 'consumer-level-config', '等级配置', NULL, 9907, 2, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:level:config', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990703, 'consumer-level-addExp', '增加经验', NULL, 9907, 3, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:level:addExp', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990704, 'consumer-level-records', '变更记录', NULL, 9907, 4, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:level:records', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 三、用户标签管理模块
-- =====================================================

-- 用户标签管理菜单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (9908, 'consumer-tag', '标签管理', NULL, 99, 8, '/consumer/tag', 'consumer/tag', '', '', '1', 'C', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:list', 'label', NOW(), NULL, NULL, 'C端用户标签管理', NULL, NULL);

-- 用户标签按钮权限
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990801, 'consumer-tag-query', '标签查询', NULL, 9908, 1, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:query', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990802, 'consumer-tag-create', '创建标签', NULL, 9908, 2, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:create', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990803, 'consumer-tag-edit', '编辑标签', NULL, 9908, 3, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:edit', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990804, 'consumer-tag-delete', '删除标签', NULL, 9908, 4, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:delete', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990805, 'consumer-tag-assign', '分配标签', NULL, 9908, 5, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:tag:assign', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 四、用户禁用管理模块
-- =====================================================

-- 用户禁用管理菜单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (9909, 'consumer-ban', '禁用管理', NULL, 99, 9, '/consumer/ban', 'consumer/ban', '', '', '1', 'C', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:ban:list', 'lock', NOW(), NULL, NULL, 'C端用户禁用管理', NULL, NULL);

-- 用户禁用按钮权限
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990901, 'consumer-ban-query', '禁用查询', NULL, 9909, 1, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:ban:query', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990902, 'consumer-ban-ban', '禁用用户', NULL, 9909, 2, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:ban:ban', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990903, 'consumer-ban-unban', '解禁用户', NULL, 9909, 3, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:ban:unban', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990904, 'consumer-ban-history', '禁用历史', NULL, 9909, 4, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:ban:history', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 五、消费统计模块
-- =====================================================

-- 消费统计菜单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (9910, 'consumer-statistics', '消费统计', NULL, 99, 10, '/consumer/statistics', 'consumer/statistics', '', '', '1', 'C', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:statistics:list', 'chart', NOW(), NULL, NULL, 'C端用户消费统计', NULL, NULL);

-- 消费统计按钮权限
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (991001, 'consumer-statistics-query', '统计查询', NULL, 9910, 1, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:statistics:query', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (991002, 'consumer-statistics-trend', '消费趋势', NULL, 9910, 2, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:statistics:trend', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (991003, 'consumer-statistics-export', '导出统计', NULL, 9910, 3, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:statistics:export', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 六、OAuth绑定管理（作为用户管理的子功能）
-- =====================================================

-- OAuth绑定按钮权限（挂在用户管理下）
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990204, 'consumer-user-oauth', 'OAuth绑定', NULL, 9902, 4, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:user:oauth', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990205, 'consumer-user-updatePhone', '更新手机', NULL, 9902, 5, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:user:updatePhone', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990206, 'consumer-user-updateEmail', '更新邮箱', NULL, 9902, 6, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:user:updateEmail', '', NOW(), NULL, NULL, '', NULL, NULL);
INSERT INTO `sys_menu` (`id`, `name`, `title`, `i18nkey`, `pid`, `order`, `path`, `component`, `redirect`, `href`, `no_cache`, `menu_type`, `hidden`, `active_menu`, `always_show`, `breadcrumb`, `affix`, `no_tags_view`, `can_to`, `status`, `perms`, `icon`, `created_at`, `updated_by`, `updated_at`, `remark`, `created_by`, `deleted_at`) VALUES (990207, 'consumer-user-deactivate', '注销账号', NULL, 9902, 7, '', '', '', '', '1', 'F', '0', '', '1', '1', '1', '0', '1', '1', 'consumer:user:deactivate', '', NOW(), NULL, NULL, '', NULL, NULL);

-- =====================================================
-- 七、API权限配置
-- =====================================================

-- 售后管理API权限
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6001, '申请售后', '/after-sale/apply', 'POST', '0', 6001, 'C端售后申请', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6002, '审核售后', '/after-sale/audit', 'POST', '0', 6002, 'C端售后审核', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6003, '售后详情', '/after-sale/detail', 'GET', '0', 6003, 'C端售后详情', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6004, '售后列表', '/after-sale/list', 'GET', '0', 6004, 'C端售后列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6005, '关闭售后', '/after-sale/close', 'POST', '0', 6005, 'C端关闭售后', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6006, '创建退款', '/after-sale/refund/create', 'POST', '0', 6006, 'C端创建退款', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6007, '退款回调', '/after-sale/refund/callback', 'POST', '0', 6007, 'C端退款回调', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6008, '提交物流', '/after-sale/logistics/submit', 'POST', '0', 6008, 'C端提交物流', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6009, '确认收货', '/after-sale/logistics/confirm', 'POST', '0', 6009, 'C端确认收货', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6010, '获取物流', '/after-sale/logistics/get', 'GET', '0', 6010, 'C端获取物流', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6011, '售后统计', '/after-sale/statistics', 'GET', '0', 6011, 'C端售后统计', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (6012, '超时配置', '/after-sale/timeout-configs', 'GET', '0', 6012, 'C端超时配置', NOW());

-- 用户扩展API权限
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7001, '绑定OAuth', '/user-ext/oauth/bind', 'POST', '0', 7001, 'C端绑定OAuth', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7002, '解绑OAuth', '/user-ext/oauth/unbind', 'POST', '0', 7002, 'C端解绑OAuth', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7003, 'OAuth列表', '/user-ext/oauth/list', 'GET', '0', 7003, 'C端OAuth列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7004, '设置主绑定', '/user-ext/oauth/set-primary', 'POST', '0', 7004, 'C端设置主绑定', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7005, '获取用户等级', '/user-ext/level/get', 'GET', '0', 7005, 'C端获取用户等级', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7006, '增加经验值', '/user-ext/level/add-exp', 'POST', '0', 7006, 'C端增加经验值', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7007, '等级配置列表', '/user-ext/level/configs', 'GET', '0', 7007, 'C端等级配置列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7008, '等级变更记录', '/user-ext/level/records', 'GET', '0', 7008, 'C端等级变更记录', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7009, '创建等级配置', '/user-ext/level/create-config', 'POST', '0', 7009, 'C端创建等级配置', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7010, '创建标签', '/user-ext/tag/create', 'POST', '0', 7010, 'C端创建标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7011, '更新标签', '/user-ext/tag/update', 'PUT', '0', 7011, 'C端更新标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7012, '删除标签', '/user-ext/tag/delete', 'DELETE', '0', 7012, 'C端删除标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7013, '标签列表', '/user-ext/tag/list', 'GET', '0', 7013, 'C端标签列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7014, '给用户添加标签', '/user-ext/tag/add-user', 'POST', '0', 7014, 'C端给用户添加标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7015, '移除用户标签', '/user-ext/tag/remove-user', 'POST', '0', 7015, 'C端移除用户标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7016, '获取用户标签', '/user-ext/tag/user-tags', 'GET', '0', 7016, 'C端获取用户标签', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7017, '禁用用户', '/user-ext/ban/user', 'POST', '0', 7017, 'C端禁用用户', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7018, '解禁用户', '/user-ext/ban/unban', 'POST', '0', 7018, 'C端解禁用户', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7019, '检查禁用状态', '/user-ext/ban/check', 'GET', '0', 7019, 'C端检查禁用状态', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7020, '禁用历史', '/user-ext/ban/history', 'GET', '0', 7020, 'C端禁用历史', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7021, '获取消费统计', '/user-ext/statistics/get', 'GET', '0', 7021, 'C端获取消费统计', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7022, '消费趋势', '/user-ext/statistics/trend', 'GET', '0', 7022, 'C端消费趋势', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7023, '年度统计', '/user-ext/statistics/year', 'GET', '0', 7023, 'C端年度统计', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7024, '更新手机号', '/user-ext/update-phone', 'POST', '0', 7024, 'C端更新手机号', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7025, '更新邮箱', '/user-ext/update-email', 'POST', '0', 7025, 'C端更新邮箱', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7026, '重置密码', '/user-ext/reset-password', 'POST', '0', 7026, 'C端重置密码', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (7027, '注销账号', '/user-ext/deactivate', 'POST', '0', 7027, 'C端注销账号', NOW());

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 9906 AND id <= 991003;
-- DELETE FROM sys_api_permission WHERE id >= 6001 AND id <= 7027;
