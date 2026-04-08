import { request } from './request'

// 角色相关 API 接口
export const roleApi = {
  // 获取角色列表
  getRoleList: (params: any) => {
    return request.get('/sys/role/list', { params })
  },
  // 添加角色
  createRole: (data: any) => {
    return request.post('/sys/role/add', data)
  },
  // 编辑角色
  updateRole: (data: any) => {
    return request.put('/sys/role/edit', data)
  },
  // 删除角色
  deleteRole: (roleId: number) => {
    return request.delete('/sys/role/del', { params: { role_id: roleId } })
  },
  // 获取角色菜单权限
  getRoleMenus: (roleId: number) => {
    return request.get('/sys/role/role_menus', { params: { role_id: roleId } })
  },
  // 获取角色部门权限
  getRoleDepts: (roleId: number) => {
    return request.get('/sys/role/role_depts', { params: { role_id: roleId } })
  },
  // 获取菜单树
  getMenuTree: () => {
    return request.get('/sys/role/menu')
  }
}
