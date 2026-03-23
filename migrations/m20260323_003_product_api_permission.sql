-- m20260323_003_product_api_permission.sql
-- 描述：商品管理模块API权限配置
-- 作者：tuoke
-- 日期：2026-03-23

-- =====================================================
-- API权限配置
-- =====================================================

-- 商品分类API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27101, '/api/product/category/tree', 'GET', 'product-category', '0', '获取分类树', 1, NOW()),
(27102, '/api/product/category/list', 'GET', 'product-category', '0', '获取分类列表', 2, NOW()),
(27103, '/api/product/category/:id', 'GET', 'product-category', '0', '获取分类详情', 3, NOW()),
(27104, '/api/product/category/add', 'POST', 'product-category', '0', '新增分类', 4, NOW()),
(27105, '/api/product/category/edit', 'PUT', 'product-category', '0', '编辑分类', 5, NOW()),
(27106, '/api/product/category/delete', 'DELETE', 'product-category', '0', '删除分类', 6, NOW()),
(27107, '/api/product/category/updateStatus', 'PUT', 'product-category', '0', '更新分类状态', 7, NOW());

-- 商品品牌API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27201, '/api/product/brand/list', 'GET', 'product-brand', '0', '获取品牌列表', 1, NOW()),
(27202, '/api/product/brand/:id', 'GET', 'product-brand', '0', '获取品牌详情', 2, NOW()),
(27203, '/api/product/brand/add', 'POST', 'product-brand', '0', '新增品牌', 3, NOW()),
(27204, '/api/product/brand/edit', 'PUT', 'product-brand', '0', '编辑品牌', 4, NOW()),
(27205, '/api/product/brand/delete', 'DELETE', 'product-brand', '0', '删除品牌', 5, NOW()),
(27206, '/api/product/brand/updateStatus', 'PUT', 'product-brand', '0', '更新品牌状态', 6, NOW());

-- 商品SPU API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27301, '/api/product/list', 'GET', 'product-list', '0', '获取商品列表', 1, NOW()),
(27302, '/api/product/:id', 'GET', 'product-list', '0', '获取商品详情', 2, NOW()),
(27303, '/api/product/add', 'POST', 'product-list', '0', '新增商品', 3, NOW()),
(27304, '/api/product/edit', 'PUT', 'product-list', '0', '编辑商品', 4, NOW()),
(27305, '/api/product/delete', 'DELETE', 'product-list', '0', '删除商品', 5, NOW()),
(27306, '/api/product/updateStatus', 'PUT', 'product-list', '0', '更新商品状态', 6, NOW()),
(27307, '/api/product/audit', 'PUT', 'product-list', '0', '审核商品', 7, NOW()),
(27308, '/api/product/batchUpdateStatus', 'PUT', 'product-list', '0', '批量更新状态', 8, NOW()),
(27309, '/api/product/batchDelete', 'DELETE', 'product-list', '0', '批量删除商品', 9, NOW()),
(27310, '/api/product/batchUpdateCategory', 'PUT', 'product-list', '0', '批量更新分类', 10, NOW());

-- 商品SKU API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27401, '/api/product/sku/list/:product_id', 'GET', 'product-sku', '0', '获取SKU列表', 1, NOW()),
(27402, '/api/product/sku/:id', 'GET', 'product-sku', '0', '获取SKU详情', 2, NOW()),
(27403, '/api/product/sku/add', 'POST', 'product-sku', '0', '新增SKU', 3, NOW()),
(27404, '/api/product/sku/edit', 'PUT', 'product-sku', '0', '编辑SKU', 4, NOW()),
(27405, '/api/product/sku/delete', 'DELETE', 'product-sku', '0', '删除SKU', 5, NOW()),
(27406, '/api/product/sku/generate', 'POST', 'product-sku', '0', '生成SKU组合', 6, NOW());

-- 商品分组API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27501, '/api/product/group/list', 'GET', 'product-group', '0', '获取分组列表', 1, NOW()),
(27502, '/api/product/group/:id', 'GET', 'product-group', '0', '获取分组详情', 2, NOW()),
(27503, '/api/product/group/add', 'POST', 'product-group', '0', '新增分组', 3, NOW()),
(27504, '/api/product/group/edit', 'PUT', 'product-group', '0', '编辑分组', 4, NOW()),
(27505, '/api/product/group/delete', 'DELETE', 'product-group', '0', '删除分组', 5, NOW());

-- 运费模板API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27601, '/api/product/shipping/list', 'GET', 'product-shipping', '0', '获取运费模板列表', 1, NOW()),
(27602, '/api/product/shipping/:id', 'GET', 'product-shipping', '0', '获取运费模板详情', 2, NOW()),
(27603, '/api/product/shipping/add', 'POST', 'product-shipping', '0', '新增运费模板', 3, NOW()),
(27604, '/api/product/shipping/edit', 'PUT', 'product-shipping', '0', '编辑运费模板', 4, NOW()),
(27605, '/api/product/shipping/delete', 'DELETE', 'product-shipping', '0', '删除运费模板', 5, NOW()),
(27606, '/api/product/shipping/calculate', 'POST', 'product-shipping', '0', '计算运费', 6, NOW());

-- 库存管理API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27701, '/api/product/stock/list', 'GET', 'product-stock', '0', '获取库存列表', 1, NOW()),
(27702, '/api/product/stock/log', 'GET', 'product-stock', '0', '获取库存日志', 2, NOW()),
(27703, '/api/product/stock/adjust', 'POST', 'product-stock', '0', '库存调整', 3, NOW()),
(27704, '/api/product/stock/alert/list', 'GET', 'product-stock', '0', '获取库存预警列表', 4, NOW()),
(27705, '/api/product/stock/alert/config', 'POST', 'product-stock', '0', '配置库存预警', 5, NOW());

-- 门店管理API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27801, '/api/product/store/list', 'GET', 'product-store', '0', '获取门店列表', 1, NOW()),
(27802, '/api/product/store/:id', 'GET', 'product-store', '0', '获取门店详情', 2, NOW()),
(27803, '/api/product/store/add', 'POST', 'product-store', '0', '新增门店', 3, NOW()),
(27804, '/api/product/store/edit', 'PUT', 'product-store', '0', '编辑门店', 4, NOW()),
(27805, '/api/product/store/delete', 'DELETE', 'product-store', '0', '删除门店', 5, NOW()),
(27806, '/api/product/store/stock/:store_id', 'GET', 'product-store', '0', '获取门店库存', 6, NOW()),
(27807, '/api/product/store/stock/adjust', 'POST', 'product-store', '0', '调整门店库存', 7, NOW());

-- 核销管理API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(27901, '/api/product/verification/list', 'GET', 'product-verification', '0', '获取核销码列表', 1, NOW()),
(27902, '/api/product/verification/verify', 'POST', 'product-verification', '0', '核销操作', 2, NOW()),
(27903, '/api/product/verification/log', 'GET', 'product-verification', '0', '获取核销记录', 3, NOW()),
(27904, '/api/product/verification/query', 'GET', 'product-verification', '0', '查询核销码', 4, NOW());

-- 属性模板API
INSERT INTO `sys_api_permission` (`id`, `api`, `method`, `apiname`, `logcache`, `remark`, `sort`, `created_at`)
VALUES
(28001, '/api/product/attribute/list', 'GET', 'product-attribute', '0', '获取属性模板列表', 1, NOW()),
(28002, '/api/product/attribute/:id', 'GET', 'product-attribute', '0', '获取属性模板详情', 2, NOW()),
(28003, '/api/product/attribute/add', 'POST', 'product-attribute', '0', '新增属性模板', 3, NOW()),
(28004, '/api/product/attribute/edit', 'PUT', 'product-attribute', '0', '编辑属性模板', 4, NOW()),
(28005, '/api/product/attribute/delete', 'DELETE', 'product-attribute', '0', '删除属性模板', 5, NOW()),
(28006, '/api/product/attribute/byCategory/:category_id', 'GET', 'product-attribute', '0', '根据分类获取属性', 6, NOW());

-- ROLLBACK:
-- DELETE FROM sys_api_permission WHERE id >= 27100 AND id < 28100;
