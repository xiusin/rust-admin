-- m20260322_017_add_dict_deleted_at.sql
-- 描述：为字典类型表和字典数据表添加软删除字段 deleted_at 和状态字段 status
-- 作者：tuoke
-- 日期：2026-03-22

-- 为字典类型表添加 status 字段
ALTER TABLE `sys_dict_type` ADD COLUMN `status` char(1) NOT NULL DEFAULT '1' COMMENT '状态（0停用 1启用）' AFTER `order`;

-- 为字典类型表添加 deleted_at 字段
ALTER TABLE `sys_dict_type` ADD COLUMN `deleted_at` datetime DEFAULT NULL COMMENT '删除时间' AFTER `updated_at`;

-- 为字典数据表添加 status 字段
ALTER TABLE `sys_dict_data` ADD COLUMN `status` char(1) NOT NULL DEFAULT '1' COMMENT '状态（0停用 1启用）' AFTER `dict_type_id`;

-- 为字典数据表添加 deleted_at 字段
ALTER TABLE `sys_dict_data` ADD COLUMN `deleted_at` datetime DEFAULT NULL COMMENT '删除时间' AFTER `updated_at`;

-- 添加索引优化查询性能
ALTER TABLE `sys_dict_type` ADD KEY `idx_status` (`status`);
ALTER TABLE `sys_dict_type` ADD KEY `idx_deleted_at` (`deleted_at`);
ALTER TABLE `sys_dict_data` ADD KEY `idx_status` (`status`);
ALTER TABLE `sys_dict_data` ADD KEY `idx_deleted_at` (`deleted_at`);

-- ROLLBACK:
-- ALTER TABLE `sys_dict_type` DROP COLUMN `status`;
-- ALTER TABLE `sys_dict_type` DROP COLUMN `deleted_at`;
-- ALTER TABLE `sys_dict_data` DROP COLUMN `status`;
-- ALTER TABLE `sys_dict_data` DROP COLUMN `deleted_at`;
