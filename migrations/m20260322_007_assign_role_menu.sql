-- m20260322_007_assign_role_menu.sql
-- 描述：为超级管理员角色分配所有C端用户新菜单权限
-- 作者：tuoke
-- 日期：2026-03-22

-- 为超级管理员角色(role_id=1)分配所有新菜单
-- 插入所有ID >= 99 的菜单（C端用户管理目录及其子菜单）
INSERT INTO sys_role_menu (role_id, menu_id)
SELECT 1, id FROM sys_menu WHERE id >= 99 AND id < 100000;

-- 为超级管理员角色分配所有新API权限
-- 首先获取所有新API的ID
-- 然后插入到角色API关联表（如果存在的话）

-- ROLLBACK:
-- DELETE FROM sys_role_menu WHERE role_id = 1 AND menu_id >= 99 AND menu_id < 100000;
