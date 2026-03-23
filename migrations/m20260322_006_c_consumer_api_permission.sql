-- m20260322_006_c_consumer_api_permission.sql
-- 描述：补全C端用户管理其他模块的API权限（支付、短信、物流、微信、媒体、运费、财务）
-- 作者：tuoke
-- 日期：2026-03-22

-- =====================================================
-- 一、支付模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8001, '创建支付订单', '/payment/create', 'POST', '0', 8001, 'C端创建支付订单', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8002, '获取支付订单', '/payment/{order_no}', 'GET', '0', 8002, 'C端获取支付订单', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8003, '支付订单列表', '/payment/list', 'GET', '0', 8003, 'C端支付订单列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8004, '支付退款', '/payment/refund', 'POST', '0', 8004, 'C端支付退款', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8005, '关闭支付订单', '/payment/close/{order_no}', 'POST', '0', 8005, 'C端关闭支付订单', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8006, '支付统计', '/payment/statistics', 'GET', '0', 8006, 'C端支付统计', NOW());

-- =====================================================
-- 二、短信模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8101, '发送验证码', '/sms/send-code', 'POST', '0', 8101, 'C端发送验证码', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8102, '验证验证码', '/sms/verify-code', 'POST', '0', 8102, 'C端验证验证码', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8103, '短信日志列表', '/sms/logs', 'GET', '0', 8103, 'C端短信日志列表', NOW());

-- =====================================================
-- 三、物流模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8201, '查询物流', '/logistics/query', 'POST', '0', 8201, 'C端查询物流', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8202, '订阅物流', '/logistics/subscribe', 'POST', '0', 8202, 'C端订阅物流', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8203, '物流历史', '/logistics/history', 'GET', '0', 8203, 'C端物流历史', NOW());

-- =====================================================
-- 四、微信模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8301, '获取微信授权URL', '/wechat/auth-url', 'POST', '0', 8301, 'C端获取微信授权URL', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8302, '微信授权回调', '/wechat/callback', 'GET', '0', 8302, 'C端微信授权回调', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8303, '小程序登录', '/wechat/mini/login', 'POST', '0', 8303, 'C端小程序登录', NOW());

-- =====================================================
-- 五、媒体模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8401, '生成上传URL', '/media/upload-url', 'POST', '0', 8401, 'C端生成上传URL', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8402, '确认上传', '/media/confirm', 'POST', '0', 8402, 'C端确认上传', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8403, '媒体文件列表', '/media/list', 'GET', '0', 8403, 'C端媒体文件列表', NOW());

-- =====================================================
-- 六、运费模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8501, '计算运费', '/freight/calculate', 'POST', '0', 8501, 'C端计算运费', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8502, '创建运费模板', '/freight/template', 'POST', '0', 8502, 'C端创建运费模板', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8503, '运费模板列表', '/freight/templates', 'GET', '0', 8503, 'C端运费模板列表', NOW());

-- =====================================================
-- 七、财务模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8601, '获取账户信息', '/finance/account/{id}', 'GET', '0', 8601, 'C端获取账户信息', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8602, '用户充值', '/finance/recharge/{id}', 'POST', '0', 8602, 'C端用户充值', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8603, '用户提现', '/finance/withdraw/{id}', 'POST', '0', 8603, 'C端用户提现', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8604, '交易流水列表', '/finance/transactions', 'GET', '0', 8604, 'C端交易流水列表', NOW());

-- =====================================================
-- 八、消费者基础模块API权限
-- =====================================================
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8701, '用户注册', '/consumer/register', 'POST', '0', 8701, 'C端用户注册', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8702, '用户登录', '/consumer/login', 'POST', '0', 8702, 'C端用户登录', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8703, '获取用户信息', '/consumer/info/{id}', 'GET', '0', 8703, 'C端获取用户信息', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8704, '更新用户信息', '/consumer/update/{id}', 'PUT', '0', 8704, 'C端更新用户信息', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8705, '用户列表', '/consumer/list', 'GET', '0', 8705, 'C端用户列表', NOW());
INSERT INTO `sys_api_permission` (`id`, `apiname`, `api`, `method`, `logcache`, `sort`, `remark`, `created_at`) VALUES (8706, '登录日志列表', '/consumer/login-logs', 'GET', '0', 8706, 'C端登录日志列表', NOW());

-- ROLLBACK:
-- DELETE FROM sys_api_permission WHERE id >= 8001 AND id <= 8706;
