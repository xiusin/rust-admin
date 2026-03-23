-- m20260322_012_fix_menu_status.sql
-- 描述：修复菜单状态字段，将所有菜单状态改为正常(0)
-- 作者：tuoke
-- 日期：2026-03-22

-- 修复所有菜单状态为正常(0)
UPDATE sys_menu SET status = '0' WHERE status = '1' AND deleted_at IS NULL;

-- ROLLBACK:
-- UPDATE sys_menu SET status = '1' WHERE status = '0' AND deleted_at IS NULL;
