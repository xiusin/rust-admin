-- m20260326_005_cms_permission.sql
-- 描述：CMS内容管理系统权限配置
-- 作者：tuoke
-- 日期：2026-03-26

-- 权限类型说明：
-- F = 按钮/功能权限

START TRANSACTION;

-- =====================================================
-- 1. 模型管理权限
-- =====================================================
-- 模型列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3101, 'cms-model-query', '模型查询', 3001, 1, 'F', '1', 'cms:model:query', NOW());

-- 新增模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3102, 'cms-model-create', '新增模型', 3001, 2, 'F', '1', 'cms:model:create', NOW());

-- 编辑模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3103, 'cms-model-update', '编辑模型', 3001, 3, 'F', '1', 'cms:model:update', NOW());

-- 删除模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3104, 'cms-model-delete', '删除模型', 3001, 4, 'F', '1', 'cms:model:delete', NOW());

-- 复制模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3105, 'cms-model-copy', '复制模型', 3001, 5, 'F', '1', 'cms:model:copy', NOW());

-- 启用模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3106, 'cms-model-enable', '启用模型', 3001, 6, 'F', '1', 'cms:model:enable', NOW());

-- 禁用模型
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3107, 'cms-model-disable', '禁用模型', 3001, 7, 'F', '1', 'cms:model:disable', NOW());

-- =====================================================
-- 2. 字段管理权限
-- =====================================================
-- 字段列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3111, 'cms-field-query', '字段查询', 3001, 11, 'F', '1', 'cms:field:query', NOW());

-- 新增字段
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3112, 'cms-field-create', '新增字段', 3001, 12, 'F', '1', 'cms:field:create', NOW());

-- 编辑字段
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3113, 'cms-field-update', '编辑字段', 3001, 13, 'F', '1', 'cms:field:update', NOW());

-- 删除字段
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3114, 'cms-field-delete', '删除字段', 3001, 14, 'F', '1', 'cms:field:delete', NOW());

-- =====================================================
-- 3. 内容管理权限
-- =====================================================
-- 内容列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3121, 'cms-content-query', '内容查询', 3010, 1, 'F', '1', 'cms:content:query', NOW());

-- 新增内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3122, 'cms-content-create', '新增内容', 3010, 2, 'F', '1', 'cms:content:create', NOW());

-- 编辑内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3123, 'cms-content-update', '编辑内容', 3010, 3, 'F', '1', 'cms:content:update', NOW());

-- 删除内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3124, 'cms-content-delete', '删除内容', 3010, 4, 'F', '1', 'cms:content:delete', NOW());

-- 发布内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3125, 'cms-content-publish', '发布内容', 3010, 5, 'F', '1', 'cms:content:publish', NOW());

-- 下线内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3126, 'cms-content-offline', '下线内容', 3010, 6, 'F', '1', 'cms:content:offline', NOW());

-- 审核内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3127, 'cms-content-audit', '审核内容', 3010, 7, 'F', '1', 'cms:content:audit', NOW());

-- 恢复内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3128, 'cms-content-restore', '恢复内容', 3010, 8, 'F', '1', 'cms:content:restore', NOW());

-- 批量操作
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3129, 'cms-content-batch', '批量操作', 3010, 9, 'F', '1', 'cms:content:batch', NOW());

-- =====================================================
-- 4. 分类管理权限
-- =====================================================
-- 分类列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3131, 'cms-category-query', '分类查询', 3020, 1, 'F', '1', 'cms:category:query', NOW());

-- 新增分类
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3132, 'cms-category-create', '新增分类', 3020, 2, 'F', '1', 'cms:category:create', NOW());

-- 编辑分类
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3133, 'cms-category-update', '编辑分类', 3020, 3, 'F', '1', 'cms:category:update', NOW());

-- 删除分类
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3134, 'cms-category-delete', '删除分类', 3020, 4, 'F', '1', 'cms:category:delete', NOW());

-- 移动分类
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3135, 'cms-category-move', '移动分类', 3020, 5, 'F', '1', 'cms:category:move', NOW());

-- =====================================================
-- 5. 标签管理权限
-- =====================================================
-- 标签列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3141, 'cms-tag-query', '标签查询', 3030, 1, 'F', '1', 'cms:tag:query', NOW());

-- 新增标签
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3142, 'cms-tag-create', '新增标签', 3030, 2, 'F', '1', 'cms:tag:create', NOW());

-- 编辑标签
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3143, 'cms-tag-update', '编辑标签', 3030, 3, 'F', '1', 'cms:tag:update', NOW());

-- 删除标签
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3144, 'cms-tag-delete', '删除标签', 3030, 4, 'F', '1', 'cms:tag:delete', NOW());

-- 批量添加标签
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3145, 'cms-tag-batch', '批量添加', 3030, 5, 'F', '1', 'cms:tag:batch', NOW());

-- =====================================================
-- 6. 表单配置权限
-- =====================================================
-- 表单列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3151, 'cms-form-query', '表单查询', 3040, 1, 'F', '1', 'cms:form:query', NOW());

-- 新增表单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3152, 'cms-form-create', '新增表单', 3040, 2, 'F', '1', 'cms:form:create', NOW());

-- 编辑表单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3153, 'cms-form-update', '编辑表单', 3040, 3, 'F', '1', 'cms:form:update', NOW());

-- 删除表单
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3154, 'cms-form-delete', '删除表单', 3040, 4, 'F', '1', 'cms:form:delete', NOW());

-- =====================================================
-- 7. 表格配置权限
-- =====================================================
-- 表格列表
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3161, 'cms-table-query', '表格查询', 3050, 1, 'F', '1', 'cms:table:query', NOW());

-- 新增表格
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3162, 'cms-table-create', '新增表格', 3050, 2, 'F', '1', 'cms:table:create', NOW());

-- 编辑表格
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3163, 'cms-table-update', '编辑表格', 3050, 3, 'F', '1', 'cms:table:update', NOW());

-- 删除表格
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3164, 'cms-table-delete', '删除表格', 3050, 4, 'F', '1', 'cms:table:delete', NOW());

-- 复制表格
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3165, 'cms-table-copy', '复制表格', 3050, 5, 'F', '1', 'cms:table:copy', NOW());

-- =====================================================
-- 8. 代码生成权限
-- =====================================================
-- 代码预览
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3171, 'cms-codegen-preview', '代码预览', 3060, 1, 'F', '1', 'cms:codegen:preview', NOW());

-- 代码下载
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3172, 'cms-codegen-download', '代码下载', 3060, 2, 'F', '1', 'cms:codegen:download', NOW());

-- 生成代码
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (3173, 'cms-codegen-generate', '生成代码', 3060, 3, 'F', '1', 'cms:codegen:generate', NOW());

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 3100 AND id < 3200;
