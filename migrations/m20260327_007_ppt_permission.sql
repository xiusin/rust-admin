-- m20260327_007_ppt_permission.sql
-- 描述：AI PPT生成系统权限配置
-- 作者：tuoke
-- 日期：2026-03-27

-- 权限类型说明：
-- F = 按钮/功能权限

START TRANSACTION;

-- =====================================================
-- 1. PPT项目管理权限
-- =====================================================
-- 项目查询
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4101, 'ai-ppt-project-query', '项目查询', 4001, 1, 'F', '1', 'ppt:project:query', NOW());

-- 项目新增
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4102, 'ai-ppt-project-create', '项目新增', 4001, 2, 'F', '1', 'ppt:project:create', NOW());

-- 项目编辑
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4103, 'ai-ppt-project-update', '项目编辑', 4001, 3, 'F', '1', 'ppt:project:update', NOW());

-- 项目删除
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4104, 'ai-ppt-project-delete', '项目删除', 4001, 4, 'F', '1', 'ppt:project:delete', NOW());

-- 项目复制
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4105, 'ai-ppt-project-copy', '项目复制', 4001, 5, 'F', '1', 'ppt:project:copy', NOW());

-- 项目导出
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4106, 'ai-ppt-project-export', '项目导出', 4001, 6, 'F', '1', 'ppt:project:export', NOW());

-- 项目分享
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4107, 'ai-ppt-project-share', '项目分享', 4001, 7, 'F', '1', 'ppt:project:share', NOW());

-- =====================================================
-- 2. PPT页面管理权限
-- =====================================================
-- 页面查询
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4111, 'ai-ppt-page-query', '页面查询', 4001, 11, 'F', '1', 'ppt:page:query', NOW());

-- 页面新增
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4112, 'ai-ppt-page-create', '页面新增', 4001, 12, 'F', '1', 'ppt:page:create', NOW());

-- 页面编辑
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4113, 'ai-ppt-page-update', '页面编辑', 4001, 13, 'F', '1', 'ppt:page:update', NOW());

-- 页面删除
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4114, 'ai-ppt-page-delete', '页面删除', 4001, 14, 'F', '1', 'ppt:page:delete', NOW());

-- 页面排序
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4115, 'ai-ppt-page-sort', '页面排序', 4001, 15, 'F', '1', 'ppt:page:sort', NOW());

-- =====================================================
-- 3. 页面元素管理权限
-- =====================================================
-- 元素查询
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4121, 'ai-ppt-element-query', '元素查询', 4001, 21, 'F', '1', 'ppt:element:query', NOW());

-- 元素新增
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4122, 'ai-ppt-element-create', '元素新增', 4001, 22, 'F', '1', 'ppt:element:create', NOW());

-- 元素编辑
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4123, 'ai-ppt-element-update', '元素编辑', 4001, 23, 'F', '1', 'ppt:element:update', NOW());

-- 元素删除
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4124, 'ai-ppt-element-delete', '元素删除', 4001, 24, 'F', '1', 'ppt:element:delete', NOW());

-- 元素锁定
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4125, 'ai-ppt-element-lock', '元素锁定', 4001, 25, 'F', '1', 'ppt:element:lock', NOW());

-- =====================================================
-- 4. 风格模板管理权限
-- =====================================================
-- 模板查询
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4131, 'ai-ppt-template-query', '模板查询', 4010, 1, 'F', '1', 'ppt:template:query', NOW());

-- 模板新增
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4132, 'ai-ppt-template-create', '模板新增', 4010, 2, 'F', '1', 'ppt:template:create', NOW());

-- 模板编辑
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4133, 'ai-ppt-template-update', '模板编辑', 4010, 3, 'F', '1', 'ppt:template:update', NOW());

-- 模板删除
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4134, 'ai-ppt-template-delete', '模板删除', 4010, 4, 'F', '1', 'ppt:template:delete', NOW());

-- 模板复制
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4135, 'ai-ppt-template-copy', '模板复制', 4010, 5, 'F', '1', 'ppt:template:copy', NOW());

-- 模板启用
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4136, 'ai-ppt-template-enable', '模板启用', 4010, 6, 'F', '1', 'ppt:template:enable', NOW());

-- 模板禁用
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4137, 'ai-ppt-template-disable', '模板禁用', 4010, 7, 'F', '1', 'ppt:template:disable', NOW());

-- =====================================================
-- 5. AI生成权限
-- =====================================================
-- AI生成大纲
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4141, 'ai-ppt-generate-outline', '生成大纲', 4001, 31, 'F', '1', 'ppt:generate:outline', NOW());

-- AI生成内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4142, 'ai-ppt-generate-content', '生成内容', 4001, 32, 'F', '1', 'ppt:generate:content', NOW());

-- AI生成页面
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4143, 'ai-ppt-generate-page', '生成页面', 4001, 33, 'F', '1', 'ppt:generate:page', NOW());

-- AI优化内容
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4144, 'ai-ppt-generate-optimize', '优化内容', 4001, 34, 'F', '1', 'ppt:generate:optimize', NOW());

-- AI识别行业
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4145, 'ai-ppt-generate-industry', '识别行业', 4001, 35, 'F', '1', 'ppt:generate:industry', NOW());

-- =====================================================
-- 6. 生成历史权限
-- =====================================================
-- 历史查询
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4151, 'ai-ppt-history-query', '历史查询', 4020, 1, 'F', '1', 'ppt:history:query', NOW());

-- 历史详情
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4152, 'ai-ppt-history-detail', '历史详情', 4020, 2, 'F', '1', 'ppt:history:detail', NOW());

-- 历史删除
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4153, 'ai-ppt-history-delete', '历史删除', 4020, 3, 'F', '1', 'ppt:history:delete', NOW());

-- 历史导出
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (4154, 'ai-ppt-history-export', '历史导出', 4020, 4, 'F', '1', 'ppt:history:export', NOW());

COMMIT;

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 4100 AND id < 4200;
