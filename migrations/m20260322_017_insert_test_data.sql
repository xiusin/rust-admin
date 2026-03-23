-- m20260322_017_insert_test_data.sql
-- 描述：插入各模块测试数据
-- 作者：tuoke
-- 日期：2026-03-22

-- ============================================
-- 一、支付渠道配置数据（使用INSERT IGNORE避免重复）
-- ============================================
INSERT IGNORE INTO `pay_channel` (`id`, `name`, `code`, `channel_type`, `scenes`, `config`, `sort`, `is_active`, `remark`, `created_at`) VALUES
(101, '微信支付-小程序', 'wechat_miniapp', 'wechat', '["miniapp"]', '{"app_id":"wx1234567890abcdef","mch_id":"1234567890","notify_url":"https://api.example.com/pay/notify/wechat"}', 1, 1, '小程序支付渠道', NOW()),
(102, '微信支付-H5', 'wechat_h5', 'wechat', '["h5"]', '{"app_id":"wx1234567890abcdef","mch_id":"1234567890","notify_url":"https://api.example.com/pay/notify/wechat"}', 2, 1, 'H5支付渠道', NOW()),
(103, '微信支付-APP', 'wechat_app', 'wechat', '["app"]', '{"app_id":"wx1234567890abcdef","mch_id":"1234567890","notify_url":"https://api.example.com/pay/notify/wechat"}', 3, 1, 'APP支付渠道', NOW()),
(104, '支付宝-H5', 'alipay_h5', 'alipay', '["h5"]', '{"app_id":"2021001234567890","notify_url":"https://api.example.com/pay/notify/alipay"}', 4, 1, '支付宝H5支付', NOW()),
(105, '支付宝-APP', 'alipay_app', 'alipay', '["app"]', '{"app_id":"2021001234567890","notify_url":"https://api.example.com/pay/notify/alipay"}', 5, 1, '支付宝APP支付', NOW()),
(106, '银联支付', 'unionpay', 'unionpay', '["h5","pc"]', '{"mer_id":"777290058110097","notify_url":"https://api.example.com/pay/notify/unionpay"}', 6, 0, '银联支付渠道', NOW()),
(107, '余额支付', 'balance', 'balance', '["h5","app","miniapp","pc"]', NULL, 7, 1, '用户余额支付', NOW());

-- ============================================
-- 二、短信记录测试数据
-- ============================================
INSERT IGNORE INTO `c_sms_log` (`id`, `phone`, `sms_type`, `content`, `code`, `channel`, `status`, `error_message`, `sent_at`, `expires_at`) VALUES
(10001, '13800138001', '"verification"', '您的验证码是：123456，有效期5分钟', '123456', 'aliyun', 'verified', NULL, DATE_SUB(NOW(), INTERVAL 1 HOUR), DATE_SUB(NOW(), INTERVAL 55 MINUTE)),
(10002, '13800138002', '"verification"', '您的验证码是：654321，有效期5分钟', '654321', 'aliyun', 'sent', NULL, NOW(), DATE_ADD(NOW(), INTERVAL 5 MINUTE)),
(10003, '13800138003', '"verification"', '您的验证码是：111111，有效期5分钟', '111111', 'aliyun', 'failed', '手机号格式错误', DATE_SUB(NOW(), INTERVAL 2 HOUR), DATE_SUB(NOW(), INTERVAL 115 MINUTE)),
(10004, '13800138004', '"notification"', '您的订单已发货，快递单号：SF1234567890', NULL, 'aliyun', 'sent', NULL, DATE_SUB(NOW(), INTERVAL 30 MINUTE), NULL),
(10005, '13800138005', '"verification"', '您的验证码是：888888，有效期5分钟', '888888', 'aliyun', 'expired', NULL, DATE_SUB(NOW(), INTERVAL 10 MINUTE), DATE_SUB(NOW(), INTERVAL 5 MINUTE));

-- ROLLBACK:
-- DELETE FROM c_sms_log WHERE id IN (10001, 10002, 10003, 10004, 10005);
-- DELETE FROM pay_channel WHERE id IN (101, 102, 103, 104, 105, 106, 107);
