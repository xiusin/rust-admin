-- m20260322_008_c_consumer_mock_data.sql
-- 描述：C端用户管理Mock数据（包含完整的OAuth绑定信息）
-- 作者：tuoke
-- 日期：2026-03-22

-- =====================================================
-- 一、用户基础数据（10个用户，包含完整信息）
-- =====================================================
INSERT IGNORE INTO `c_consumer` (`id`, `phone`, `email`, `nickname`, `avatar`, `password_hash`, `wechat_openid`, `wechat_unionid`, `status`, `risk_score`, `login_fail_count`, `locked_until`, `last_login_at`, `last_login_ip`, `deactivated_at`, `created_at`, `updated_at`) VALUES
(1001, '13800138001', 'zhangsan@example.com', '张三', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTK7rPEqO2S9A7p3zUnicNu2n5wCl7xhTTicBT9d0S8ia0RsiaB0UkSBh1JQKRpFiaKGc03vkU1nicOFbJLA/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1001', 'wx_unionid_1001', '0', 10, 0, NULL, '2026-03-22 08:30:00', '192.168.1.100', NULL, '2025-01-15 10:00:00', '2026-03-22 08:30:00'),
(1002, '13800138002', 'lisi@example.com', '李四', 'https://thirdwx.qlogo.cn/mmopen/vi_32/PiajxSqBRaEJ8yQC1Kibj1u7GxHpGPnADX1bHEHUiaWG6kNwbLKnQDtCrWiaAZ3mPBcqJ4XlK0vNjibeFCiaVTZcO2ibA/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1002', 'wx_unionid_1002', '0', 25, 0, NULL, '2026-03-21 15:45:00', '192.168.1.101', NULL, '2025-02-20 14:30:00', '2026-03-21 15:45:00'),
(1003, '13800138003', 'wangwu@example.com', '王五', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTIB8BV9bUw2KNP7c7z90ySNu6RuWfGLdHBPKGicwO5C2S0Uib9BVNZOiaEJO3FgibiaLeaqv0Ou4ELVqng/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1003', 'wx_unionid_1003', '0', 5, 0, NULL, '2026-03-20 09:15:00', '192.168.1.102', NULL, '2025-03-10 09:00:00', '2026-03-20 09:15:00'),
(1004, '13800138004', NULL, '赵六', NULL, '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', NULL, NULL, '1', 80, 5, '2026-04-01 00:00:00', '2026-03-18 20:00:00', '192.168.1.103', '2026-03-19 10:00:00', '2025-04-05 16:20:00', '2026-03-19 10:00:00'),
(1005, '13800138005', 'qianqi@example.com', '钱七', 'https://lh3.googleusercontent.com/a/ACg8ocLzN0Jf3bNwQ5HYJhKxWC5mOv0SYiZJvNwRjWvZ8Yz=s96-c', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1005', 'wx_unionid_1005', '0', 15, 0, NULL, '2026-03-22 07:00:00', '192.168.1.104', NULL, '2025-05-12 11:45:00', '2026-03-22 07:00:00'),
(1006, '13800138006', 'sunba@example.com', '孙八', 'https://thirdwx.qlogo.cn/mmopen/vi_32/DYAIOgq83er63uOO5OwC9h5MSeM0GFgZFcibmGbS9JwPxSLcWTJ4rHOCLJ0gDwibGOV9X3LBJJ7OFdKL5fCZqZicg/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1006', 'wx_unionid_1006', '0', 30, 0, NULL, '2026-03-21 18:30:00', '192.168.1.105', NULL, '2025-06-18 08:30:00', '2026-03-21 18:30:00'),
(1007, '13800138007', NULL, '周九', NULL, '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', NULL, NULL, '0', 8, 0, NULL, '2026-03-19 12:00:00', '192.168.1.106', NULL, '2025-07-22 13:15:00', '2026-03-19 12:00:00'),
(1008, '13800138008', 'wushi@example.com', '吴十', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTK7rPEqO2S9A7p3zUnicNu2n5wCl7xhTTicBT9d0S8ia0RsiaB0UkSBh1JQKRpFiaKGc03vkU1nicOFbJLA/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1008', 'wx_unionid_1008', '0', 12, 0, NULL, '2026-03-22 09:00:00', '192.168.1.107', NULL, '2025-08-30 10:00:00', '2026-03-22 09:00:00'),
(1009, '13800138009', 'zheng11@example.com', '郑十一', 'https://thirdwx.qlogo.cn/mmopen/vi_32/PiajxSqBRaEJ8yQC1Kibj1u7GxHpGPnADX1bHEHUiaWG6kNwbLKnQDtCrWiaAZ3mPBcqJ4XlK0vNjibeFCiaVTZcO2ibA/132', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1009', 'wx_unionid_1009', '0', 20, 0, NULL, '2026-03-20 16:45:00', '192.168.1.108', NULL, '2025-09-15 15:30:00', '2026-03-20 16:45:00'),
(1010, '13800138010', 'wang12@example.com', '王十二', 'https://avatars.githubusercontent.com/u/12345678?v=4', '$2a$10$N.zmdr9k7uOCQb376NoUnuTJ8iAt6Z5EHsM8lE9lBOsl7iAt6Z5EH', 'wx_openid_1010', 'wx_unionid_1010', '0', 35, 0, NULL, '2026-03-21 21:00:00', '192.168.1.109', NULL, '2025-10-08 09:45:00', '2026-03-21 21:00:00');

-- =====================================================
-- 二、等级配置数据（6个等级）
-- =====================================================
INSERT IGNORE INTO `c_user_level_config` (`id`, `level`, `level_name`, `min_exp`, `max_exp`, `icon`, `color`, `discount_rate`, `privileges`, `is_active`, `sort_order`, `created_at`, `updated_at`) VALUES
(1, 1, '普通会员', 0, 100, 'star', '#999999', 1.00, '{"points_rate": 1, "free_shipping": false}', 1, 1, NOW(), NOW()),
(2, 2, '铜牌会员', 101, 500, 'medal', '#cd7f32', 0.98, '{"points_rate": 1.2, "free_shipping": false, "birthday_gift": true}', 1, 2, NOW(), NOW()),
(3, 3, '银牌会员', 501, 2000, 'medal', '#c0c0c0', 0.95, '{"points_rate": 1.5, "free_shipping": true, "birthday_gift": true, "exclusive_coupon": true}', 1, 3, NOW(), NOW()),
(4, 4, '金牌会员', 2001, 5000, 'crown', '#ffd700', 0.90, '{"points_rate": 2.0, "free_shipping": true, "birthday_gift": true, "exclusive_coupon": true, "priority_service": true}', 1, 4, NOW(), NOW()),
(5, 5, '钻石会员', 5001, 10000, 'diamond', '#b9f2ff', 0.85, '{"points_rate": 2.5, "free_shipping": true, "birthday_gift": true, "exclusive_coupon": true, "priority_service": true, "vip_hotline": true}', 1, 5, NOW(), NOW()),
(6, 6, '至尊会员', 10001, 999999, 'king', '#ff6b6b', 0.80, '{"points_rate": 3.0, "free_shipping": true, "birthday_gift": true, "exclusive_coupon": true, "priority_service": true, "vip_hotline": true, "personal_manager": true}', 1, 6, NOW(), NOW());

-- =====================================================
-- 三、用户等级数据（每个用户对应等级信息）
-- =====================================================
INSERT IGNORE INTO `c_user_level` (`id`, `consumer_id`, `level`, `exp`, `total_exp`, `level_up_at`, `created_at`, `updated_at`) VALUES
(1, 1001, 4, 500, 3500, '2026-03-01 10:00:00', '2025-01-15 10:00:00', '2026-03-22 08:30:00'),
(2, 1002, 3, 500, 1500, '2026-02-15 14:00:00', '2025-02-20 14:30:00', '2026-03-21 15:45:00'),
(3, 1003, 2, 100, 300, '2026-01-20 09:00:00', '2025-03-10 09:00:00', '2026-03-20 09:15:00'),
(4, 1004, 1, 50, 50, NULL, '2025-04-05 16:20:00', '2026-03-19 10:00:00'),
(5, 1005, 5, 500, 6500, '2026-03-15 14:30:00', '2025-05-12 11:45:00', '2026-03-22 07:00:00'),
(6, 1006, 3, 300, 1800, '2026-02-28 16:00:00', '2025-06-18 08:30:00', '2026-03-21 18:30:00'),
(7, 1007, 2, 150, 450, '2026-02-10 11:00:00', '2025-07-22 13:15:00', '2026-03-19 12:00:00'),
(8, 1008, 4, 300, 2800, '2026-03-10 09:15:00', '2025-08-30 10:00:00', '2026-03-22 09:00:00'),
(9, 1009, 3, 200, 1200, '2026-02-25 15:00:00', '2025-09-15 15:30:00', '2026-03-20 16:45:00'),
(10, 1010, 5, 1000, 8000, '2026-03-20 16:45:00', '2025-10-08 09:45:00', '2026-03-21 21:00:00');

-- =====================================================
-- 四、用户标签数据（系统预设标签）
-- =====================================================
INSERT IGNORE INTO `c_user_tag` (`id`, `name`, `tag_type`, `category`, `color`, `icon`, `description`, `is_active`, `sort_order`, `created_by`, `created_at`, `updated_at`) VALUES
(1, 'VIP客户', 'system', '会员', '#ff6b6b', 'crown', '高价值VIP客户，享受专属服务', 1, 1, 1, NOW(), NOW()),
(2, '活跃用户', 'system', '行为', '#52c41a', 'fire', '近30天活跃用户，经常使用平台', 1, 2, 1, NOW(), NOW()),
(3, '新用户', 'system', '生命周期', '#1890ff', 'user-add', '注册30天内新用户，需要引导', 1, 3, 1, NOW(), NOW()),
(4, '高消费', 'system', '消费', '#faad14', 'dollar', '累计消费超过1000元的高价值用户', 1, 4, 1, NOW(), NOW()),
(5, '复购用户', 'system', '行为', '#722ed1', 'sync', '有多次购买记录，忠诚度高', 1, 5, 1, NOW(), NOW()),
(6, '风险用户', 'system', '风控', '#f5222d', 'warning', '存在风险行为，需要关注', 1, 6, 1, NOW(), NOW()),
(7, '沉默用户', 'system', '生命周期', '#8c8c8c', 'meh', '超过30天未活跃，需要召回', 1, 7, 1, NOW(), NOW()),
(8, '首购用户', 'system', '生命周期', '#13c2c2', 'gift', '仅完成首次购买，需要促复购', 1, 8, 1, NOW(), NOW()),
(9, '退款频繁', 'system', '风控', '#eb2f96', 'rollback', '退款次数较多，需要关注', 1, 9, 1, NOW(), NOW()),
(10, '大客户', 'system', '会员', '#2f54eb', 'star', '消费金额超过5000元的重点客户', 1, 10, 1, NOW(), NOW());

-- =====================================================
-- 五、用户标签关联数据（用户被打上的标签）
-- =====================================================
INSERT IGNORE INTO `c_user_tag_relation` (`id`, `consumer_id`, `tag_id`, `source`, `source_desc`, `created_at`) VALUES
-- 张三：VIP、活跃、高消费、复购、大客户
(1, 1001, 1, 'system', '系统自动标记', NOW()),
(2, 1001, 2, 'system', '近30天活跃', NOW()),
(3, 1001, 4, 'system', '累计消费超过1000元', NOW()),
(4, 1001, 5, 'system', '多次购买', NOW()),
(5, 1001, 10, 'system', '消费金额超过5000元', NOW()),
-- 李四：活跃、新用户
(6, 1002, 2, 'system', '近30天活跃', NOW()),
(7, 1002, 3, 'system', '注册30天内', NOW()),
-- 王五：新用户
(8, 1003, 3, 'system', '注册30天内', NOW()),
-- 赵六：风险用户、退款频繁
(9, 1004, 6, 'system', '风险评分超过阈值', NOW()),
(10, 1004, 9, 'system', '退款次数超过3次', NOW()),
-- 钱七：VIP、活跃、高消费、复购、大客户
(11, 1005, 1, 'system', '系统自动标记', NOW()),
(12, 1005, 2, 'system', '近30天活跃', NOW()),
(13, 1005, 4, 'system', '累计消费超过1000元', NOW()),
(14, 1005, 5, 'system', '多次购买', NOW()),
(15, 1005, 10, 'system', '消费金额超过5000元', NOW()),
-- 孙八：活跃、复购
(16, 1006, 2, 'system', '近30天活跃', NOW()),
(17, 1006, 5, 'system', '多次购买', NOW()),
-- 周九：新用户
(18, 1007, 3, 'system', '注册30天内', NOW()),
-- 吴十：VIP、活跃、高消费、复购、大客户
(19, 1008, 1, 'system', '系统自动标记', NOW()),
(20, 1008, 2, 'system', '近30天活跃', NOW()),
(21, 1008, 4, 'system', '累计消费超过1000元', NOW()),
(22, 1008, 5, 'system', '多次购买', NOW()),
(23, 1008, 10, 'system', '消费金额超过5000元', NOW()),
-- 郑十一：活跃
(24, 1009, 2, 'system', '近30天活跃', NOW()),
-- 王十二：VIP、活跃、高消费、复购、大客户
(25, 1010, 1, 'system', '系统自动标记', NOW()),
(26, 1010, 2, 'system', '近30天活跃', NOW()),
(27, 1010, 4, 'system', '累计消费超过1000元', NOW()),
(28, 1010, 5, 'system', '多次购买', NOW()),
(29, 1010, 10, 'system', '消费金额超过5000元', NOW());

-- =====================================================
-- 六、OAuth绑定数据（第三方账号绑定信息）
-- =====================================================
INSERT IGNORE INTO `c_user_oauth` (`id`, `consumer_id`, `oauth_type`, `oauth_id`, `oauth_name`, `oauth_avatar`, `oauth_token`, `refresh_token`, `token_expires_at`, `bind_at`, `unbind_at`, `is_primary`, `status`, `created_at`, `updated_at`) VALUES
-- 张三：微信(主)、GitHub
(1, 1001, 'wechat', 'wx_openid_1001_xxxxx', '张三', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTK7rPEqO2S9A7p3zUnicNu2n5wCl7xhTTicBT9d0S8ia0RsiaB0UkSBh1JQKRpFiaKGc03vkU1nicOFbJLA/132', 'wx_access_token_1001_xxx', 'wx_refresh_token_1001_xxx', '2026-06-22 08:30:00', '2025-01-15 10:00:00', NULL, 1, 'active', '2025-01-15 10:00:00', '2026-03-22 08:30:00'),
(2, 1001, 'github', 'github_1001_yyyyy', 'zhangsan_dev', 'https://avatars.githubusercontent.com/u/1001?v=4', 'gho_xxxxxxxxxxxxxxxxxxxx', NULL, NULL, '2025-02-20 14:00:00', NULL, 0, 'active', '2025-02-20 14:00:00', '2025-02-20 14:00:00'),
-- 李四：微信(主)、Apple
(3, 1002, 'wechat', 'wx_openid_1002_xxxxx', '李四', 'https://thirdwx.qlogo.cn/mmopen/vi_32/PiajxSqBRaEJ8yQC1Kibj1u7GxHpGPnADX1bHEHUiaWG6kNwbLKnQDtCrWiaAZ3mPBcqJ4XlK0vNjibeFCiaVTZcO2ibA/132', 'wx_access_token_1002_xxx', 'wx_refresh_token_1002_xxx', '2026-06-21 15:45:00', '2025-02-20 14:30:00', NULL, 1, 'active', '2025-02-20 14:30:00', '2026-03-21 15:45:00'),
(4, 1002, 'apple', 'apple_1002_zzzzz', '李四', NULL, 'apple_token_1002_xxx', 'apple_refresh_token_1002_xxx', NULL, '2025-03-01 09:00:00', NULL, 0, 'active', '2025-03-01 09:00:00', '2026-03-21 15:45:00'),
-- 王五：微信(主)
(5, 1003, 'wechat', 'wx_openid_1003_xxxxx', '王五', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTIB8BV9bUw2KNP7c7z90ySNu6RuWfGLdHBPKGicwO5C2S0Uib9BVNZOiaEJO3FgibiaLeaqv0Ou4ELVqng/132', 'wx_access_token_1003_xxx', 'wx_refresh_token_1003_xxx', '2026-06-20 09:15:00', '2025-03-10 09:00:00', NULL, 1, 'active', '2025-03-10 09:00:00', '2026-03-20 09:15:00'),
-- 赵六：无第三方绑定（已禁用）
-- 钱七：微信(主)、GitHub、Google
(6, 1005, 'wechat', 'wx_openid_1005_xxxxx', '钱七', 'https://thirdwx.qlogo.cn/mmopen/vi_32/DYAIOgq83er63uOO5OwC9h5MSeM0GFgZFcibmGbS9JwPxSLcWTJ4rHOCLJ0gDwibGOV9X3LBJJ7OFdKL5fCZqZicg/132', 'wx_access_token_1005_xxx', 'wx_refresh_token_1005_xxx', '2026-06-22 07:00:00', '2025-05-12 11:45:00', NULL, 1, 'active', '2025-05-12 11:45:00', '2026-03-22 07:00:00'),
(7, 1005, 'github', 'github_1005_yyyyy', 'qianqi_coder', 'https://avatars.githubusercontent.com/u/1005?v=4', 'gho_yyyyyyyyyyyyyyyyyyyy', NULL, NULL, '2025-06-01 10:00:00', NULL, 0, 'active', '2025-06-01 10:00:00', '2025-06-01 10:00:00'),
(8, 1005, 'google', 'google_1005_aaaaa', 'qianqi@gmail.com', 'https://lh3.googleusercontent.com/a/ACg8ocLzN0Jf3bNwQ5HYJhKxWC5mOv0SYiZJvNwRjWvZ8Yz=s96-c', 'google_token_1005_xxx', 'google_refresh_token_1005_xxx', '2026-06-22 07:00:00', '2025-07-15 16:00:00', NULL, 0, 'active', '2025-07-15 16:00:00', '2026-03-22 07:00:00'),
-- 孙八：微信(主)
(9, 1006, 'wechat', 'wx_openid_1006_xxxxx', '孙八', 'https://thirdwx.qlogo.cn/mmopen/vi_32/DYAIOgq83er63uOO5OwC9h5MSeM0GFgZFcibmGbS9JwPxSLcWTJ4rHOCLJ0gDwibGOV9X3LBJJ7OFdKL5fCZqZicg/132', 'wx_access_token_1006_xxx', 'wx_refresh_token_1006_xxx', '2026-06-21 18:30:00', '2025-06-18 08:30:00', NULL, 1, 'active', '2025-06-18 08:30:00', '2026-03-21 18:30:00'),
-- 周九：无第三方绑定
-- 吴十：微信(主)、Apple
(10, 1008, 'wechat', 'wx_openid_1008_xxxxx', '吴十', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTK7rPEqO2S9A7p3zUnicNu2n5wCl7xhTTicBT9d0S8ia0RsiaB0UkSBh1JQKRpFiaKGc03vkU1nicOFbJLA/132', 'wx_access_token_1008_xxx', 'wx_refresh_token_1008_xxx', '2026-06-22 09:00:00', '2025-08-30 10:00:00', NULL, 1, 'active', '2025-08-30 10:00:00', '2026-03-22 09:00:00'),
(11, 1008, 'apple', 'apple_1008_zzzzz', '吴十', NULL, 'apple_token_1008_xxx', 'apple_refresh_token_1008_xxx', NULL, '2025-09-15 14:00:00', NULL, 0, 'active', '2025-09-15 14:00:00', '2026-03-22 09:00:00'),
-- 郑十一：微信(主)
(12, 1009, 'wechat', 'wx_openid_1009_xxxxx', '郑十一', 'https://thirdwx.qlogo.cn/mmopen/vi_32/PiajxSqBRaEJ8yQC1Kibj1u7GxHpGPnADX1bHEHUiaWG6kNwbLKnQDtCrWiaAZ3mPBcqJ4XlK0vNjibeFCiaVTZcO2ibA/132', 'wx_access_token_1009_xxx', 'wx_refresh_token_1009_xxx', '2026-06-20 16:45:00', '2025-09-15 15:30:00', NULL, 1, 'active', '2025-09-15 15:30:00', '2026-03-20 16:45:00'),
-- 王十二：微信(主)、GitHub
(13, 1010, 'wechat', 'wx_openid_1010_xxxxx', '王十二', 'https://thirdwx.qlogo.cn/mmopen/vi_32/Q0j4TwGTfTK7rPEqO2S9A7p3zUnicNu2n5wCl7xhTTicBT9d0S8ia0RsiaB0UkSBh1JQKRpFiaKGc03vkU1nicOFbJLA/132', 'wx_access_token_1010_xxx', 'wx_refresh_token_1010_xxx', '2026-06-21 21:00:00', '2025-10-08 09:45:00', NULL, 1, 'active', '2025-10-08 09:45:00', '2026-03-21 21:00:00'),
(14, 1010, 'github', 'github_1010_yyyyy', 'wang12_dev', 'https://avatars.githubusercontent.com/u/1010?v=4', 'gho_zzzzzzzzzzzzzzzzzzzz', NULL, NULL, '2025-11-01 11:00:00', NULL, 0, 'active', '2025-11-01 11:00:00', '2025-11-01 11:00:00');

-- =====================================================
-- 七、用户禁用记录
-- =====================================================
INSERT IGNORE INTO `c_user_ban` (`id`, `consumer_id`, `ban_type`, `reason`, `start_at`, `end_at`, `banned_by`, `unban_at`, `unban_by`, `unban_reason`, `status`, `created_at`, `updated_at`) VALUES
(1, 1004, 'permanent', '多次恶意刷单，违反平台规则，涉及虚假交易金额累计超过5000元', '2026-03-19 10:00:00', NULL, 1, NULL, NULL, NULL, 'active', '2026-03-19 10:00:00', '2026-03-19 10:00:00');

-- =====================================================
-- 八、消费统计数据
-- =====================================================
INSERT IGNORE INTO `c_consumer_statistics` (`id`, `consumer_id`, `total_consume`, `month_consume`, `year_consume`, `order_count`, `refund_count`, `refund_amount`, `last_order_at`, `statistics_month`, `created_at`, `updated_at`) VALUES
(1, 1001, 3580.00, 1200.00, 3580.00, 15, 0, 0.00, '2026-03-21 16:30:00', '2026-03', NOW(), NOW()),
(2, 1002, 1200.00, 800.00, 1200.00, 8, 0, 0.00, '2026-03-20 14:20:00', '2026-03', NOW(), NOW()),
(3, 1003, 450.00, 450.00, 450.00, 3, 0, 0.00, '2026-03-18 10:15:00', '2026-03', NOW(), NOW()),
(4, 1004, 280.00, 0.00, 280.00, 2, 1, 280.00, '2026-03-15 09:00:00', '2026-03', NOW(), NOW()),
(5, 1005, 8900.00, 2500.00, 8900.00, 25, 1, 150.00, '2026-03-22 08:45:00', '2026-03', NOW(), NOW()),
(6, 1006, 1800.00, 600.00, 1800.00, 10, 0, 0.00, '2026-03-19 17:30:00', '2026-03', NOW(), NOW()),
(7, 1007, 680.00, 300.00, 680.00, 5, 0, 0.00, '2026-03-17 11:20:00', '2026-03', NOW(), NOW()),
(8, 1008, 4200.00, 1500.00, 4200.00, 18, 0, 0.00, '2026-03-21 20:10:00', '2026-03', NOW(), NOW()),
(9, 1009, 920.00, 400.00, 920.00, 6, 0, 0.00, '2026-03-18 15:45:00', '2026-03', NOW(), NOW()),
(10, 1010, 12500.00, 3500.00, 12500.00, 32, 1, 200.00, '2026-03-22 10:30:00', '2026-03', NOW(), NOW());

-- =====================================================
-- 九、登录日志数据（最近登录记录）
-- =====================================================
INSERT IGNORE INTO `c_login_log` (`id`, `consumer_id`, `phone`, `login_type`, `success`, `fail_reason`, `ip_address`, `user_agent`, `device_type`, `login_at`) VALUES
(1, 1001, '13800138001', 'wechat', 1, NULL, '192.168.1.100', 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15', 'mobile', '2026-03-22 08:30:00'),
(2, 1001, '13800138001', 'password', 1, NULL, '192.168.1.100', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36', 'desktop', '2026-03-21 09:15:00'),
(3, 1002, '13800138002', 'wechat', 1, NULL, '192.168.1.101', 'Mozilla/5.0 (Linux; Android 14; Huawei) AppleWebKit/537.36', 'mobile', '2026-03-21 15:45:00'),
(4, 1002, '13800138002', 'apple', 1, NULL, '192.168.1.101', 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X)', 'mobile', '2026-03-20 10:30:00'),
(5, 1003, '13800138003', 'wechat', 1, NULL, '192.168.1.102', 'Mozilla/5.0 (Linux; Android 14; Xiaomi) AppleWebKit/537.36', 'mobile', '2026-03-20 09:15:00'),
(6, 1004, '13800138004', 'password', 0, '密码错误', '192.168.1.103', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'desktop', '2026-03-18 20:00:00'),
(7, 1004, '13800138004', 'password', 0, '密码错误', '192.168.1.103', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'desktop', '2026-03-18 19:55:00'),
(8, 1004, '13800138004', 'password', 0, '密码错误', '192.168.1.103', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'desktop', '2026-03-18 19:50:00'),
(9, 1005, '13800138005', 'wechat', 1, NULL, '192.168.1.104', 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)', 'mobile', '2026-03-22 07:00:00'),
(10, 1005, '13800138005', 'google', 1, NULL, '192.168.1.104', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'desktop', '2026-03-21 14:20:00'),
(11, 1006, '13800138006', 'wechat', 1, NULL, '192.168.1.105', 'Mozilla/5.0 (Linux; Android 14; Oppo) AppleWebKit/537.36', 'mobile', '2026-03-21 18:30:00'),
(12, 1007, '13800138007', 'password', 1, NULL, '192.168.1.106', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Safari/605.1.15', 'desktop', '2026-03-19 12:00:00'),
(13, 1008, '13800138008', 'wechat', 1, NULL, '192.168.1.107', 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X)', 'mobile', '2026-03-22 09:00:00'),
(14, 1008, '13800138008', 'apple', 1, NULL, '192.168.1.107', 'Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X)', 'tablet', '2026-03-20 21:30:00'),
(15, 1009, '13800138009', 'wechat', 1, NULL, '192.168.1.108', 'Mozilla/5.0 (Linux; Android 14; Vivo) AppleWebKit/537.36', 'mobile', '2026-03-20 16:45:00'),
(16, 1010, '13800138010', 'wechat', 1, NULL, '192.168.1.109', 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)', 'mobile', '2026-03-21 21:00:00'),
(17, 1010, '13800138010', 'github', 1, NULL, '192.168.1.109', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36', 'desktop', '2026-03-20 16:00:00');

-- =====================================================
-- ROLLBACK (执行以下SQL可回滚本次Mock数据)
-- =====================================================
DELETE FROM c_login_log WHERE id >= 1 AND id <= 17;
DELETE FROM c_consumer_statistics WHERE id >= 1 AND id <= 10;
DELETE FROM c_user_ban WHERE id >= 1 AND id <= 1;
DELETE FROM c_user_oauth WHERE id >= 1 AND id <= 14;
DELETE FROM c_user_tag_relation WHERE id >= 1 AND id <= 29;
DELETE FROM c_user_tag WHERE id >= 1 AND id <= 10;
DELETE FROM c_user_level WHERE id >= 1 AND id <= 10;
DELETE FROM c_user_level_config WHERE id >= 1 AND id <= 6;
DELETE FROM c_consumer WHERE id >= 1001 AND id <= 1010;
