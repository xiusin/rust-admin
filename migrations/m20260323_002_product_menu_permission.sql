-- m20260323_002_product_menu_permission.sql
-- 描述：商品管理模块菜单和权限配置
-- 作者：tuoke
-- 日期：2026-03-23

-- 菜单类型说明：
-- M = 目录
-- C = 菜单
-- F = 按钮

-- =====================================================
-- 1. 商品管理目录
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `menu_type`, `status`, `perms`, `icon`, `created_at`, `remark`)
VALUES (2700, 'product', '商品管理', 0, 20, '/product', 'M', '1', NULL, 'shopping', NOW(), '商品管理目录');

-- =====================================================
-- 2. 商品分类管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2701, 'product-category', '商品分类', 2700, 1, '/product/category', 'product/category/index', 'C', '1', 'product:category:list', 'folder', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270101, 'product-category-query', '分类查询', 2701, 1, 'F', '1', 'product:category:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270102, 'product-category-create', '分类新增', 2701, 2, 'F', '1', 'product:category:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270103, 'product-category-edit', '分类编辑', 2701, 3, 'F', '1', 'product:category:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270104, 'product-category-delete', '分类删除', 2701, 4, 'F', '1', 'product:category:delete', NOW());

-- =====================================================
-- 3. 商品品牌管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2702, 'product-brand', '商品品牌', 2700, 2, '/product/brand', 'product/brand/index', 'C', '1', 'product:brand:list', 'star', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270201, 'product-brand-query', '品牌查询', 2702, 1, 'F', '1', 'product:brand:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270202, 'product-brand-create', '品牌新增', 2702, 2, 'F', '1', 'product:brand:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270203, 'product-brand-edit', '品牌编辑', 2702, 3, 'F', '1', 'product:brand:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270204, 'product-brand-delete', '品牌删除', 2702, 4, 'F', '1', 'product:brand:delete', NOW());

-- =====================================================
-- 4. 商品管理（SPU）
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2703, 'product-list', '商品列表', 2700, 3, '/product/list', 'product/list/index', 'C', '1', 'product:list:list', 'goods', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270301, 'product-list-query', '商品查询', 2703, 1, 'F', '1', 'product:list:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270302, 'product-list-create', '商品新增', 2703, 2, 'F', '1', 'product:list:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270303, 'product-list-edit', '商品编辑', 2703, 3, 'F', '1', 'product:list:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270304, 'product-list-delete', '商品删除', 2703, 4, 'F', '1', 'product:list:delete', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270305, 'product-list-status', '商品上下架', 2703, 5, 'F', '1', 'product:list:status', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270306, 'product-list-audit', '商品审核', 2703, 6, 'F', '1', 'product:list:audit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270307, 'product-list-batch', '批量操作', 2703, 7, 'F', '1', 'product:list:batch', NOW());

-- =====================================================
-- 5. 商品分组管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2704, 'product-group', '商品分组', 2700, 4, '/product/group', 'product/group/index', 'C', '1', 'product:group:list', 'tags', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270401, 'product-group-query', '分组查询', 2704, 1, 'F', '1', 'product:group:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270402, 'product-group-create', '分组新增', 2704, 2, 'F', '1', 'product:group:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270403, 'product-group-edit', '分组编辑', 2704, 3, 'F', '1', 'product:group:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270404, 'product-group-delete', '分组删除', 2704, 4, 'F', '1', 'product:group:delete', NOW());

-- =====================================================
-- 6. 运费模板管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2705, 'product-shipping', '运费模板', 2700, 5, '/product/shipping', 'product/shipping/index', 'C', '1', 'product:shipping:list', 'car', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270501, 'product-shipping-query', '模板查询', 2705, 1, 'F', '1', 'product:shipping:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270502, 'product-shipping-create', '模板新增', 2705, 2, 'F', '1', 'product:shipping:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270503, 'product-shipping-edit', '模板编辑', 2705, 3, 'F', '1', 'product:shipping:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270504, 'product-shipping-delete', '模板删除', 2705, 4, 'F', '1', 'product:shipping:delete', NOW());

-- =====================================================
-- 7. 库存管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2706, 'product-stock', '库存管理', 2700, 6, '/product/stock', 'product/stock/index', 'C', '1', 'product:stock:list', 'storage', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270601, 'product-stock-query', '库存查询', 2706, 1, 'F', '1', 'product:stock:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270602, 'product-stock-adjust', '库存调整', 2706, 2, 'F', '1', 'product:stock:adjust', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270603, 'product-stock-log', '库存日志', 2706, 3, 'F', '1', 'product:stock:log', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270604, 'product-stock-alert', '库存预警', 2706, 4, 'F', '1', 'product:stock:alert', NOW());

-- =====================================================
-- 8. 门店管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2707, 'product-store', '门店管理', 2700, 7, '/product/store', 'product/store/index', 'C', '1', 'product:store:list', 'shop', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270701, 'product-store-query', '门店查询', 2707, 1, 'F', '1', 'product:store:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270702, 'product-store-create', '门店新增', 2707, 2, 'F', '1', 'product:store:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270703, 'product-store-edit', '门店编辑', 2707, 3, 'F', '1', 'product:store:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270704, 'product-store-delete', '门店删除', 2707, 4, 'F', '1', 'product:store:delete', NOW());

-- =====================================================
-- 9. 核销管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2708, 'product-verification', '核销管理', 2700, 8, '/product/verification', 'product/verification/index', 'C', '1', 'product:verification:list', 'check-circle', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270801, 'product-verification-query', '核销查询', 2708, 1, 'F', '1', 'product:verification:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270802, 'product-verification-verify', '核销操作', 2708, 2, 'F', '1', 'product:verification:verify', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270803, 'product-verification-log', '核销记录', 2708, 3, 'F', '1', 'product:verification:log', NOW());

-- =====================================================
-- 10. 属性模板管理
-- =====================================================
INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `path`, `component`, `menu_type`, `status`, `perms`, `icon`, `created_at`)
VALUES (2709, 'product-attribute', '属性模板', 2700, 9, '/product/attribute', 'product/attribute/index', 'C', '1', 'product:attribute:list', 'filter', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270901, 'product-attribute-query', '属性查询', 2709, 1, 'F', '1', 'product:attribute:query', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270902, 'product-attribute-create', '属性新增', 2709, 2, 'F', '1', 'product:attribute:create', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270903, 'product-attribute-edit', '属性编辑', 2709, 3, 'F', '1', 'product:attribute:edit', NOW());

INSERT INTO `sys_menu` (`id`, `name`, `title`, `pid`, `order`, `menu_type`, `status`, `perms`, `created_at`)
VALUES (270904, 'product-attribute-delete', '属性删除', 2709, 4, 'F', '1', 'product:attribute:delete', NOW());

-- ROLLBACK:
-- DELETE FROM sys_menu WHERE id >= 2700 AND id < 2800;
