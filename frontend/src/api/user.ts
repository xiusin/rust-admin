import { request } from './request'

// 用户相关 API 接口
export const userApi = {
  // 获取用户列表
  getUserList: (params: any) => {
    return request.get('/sys/user/list', { params })
  },
  // 添加用户
  createUser: (data: any) => {
    return request.post('/sys/user/add', data)
  },
  // 编辑用户
  updateUser: (data: any) => {
    return request.put('/sys/user/edit', data)
  },
  // 删除用户
  deleteUser: (ids: number[]) => {
    return request.delete('/sys/user/delusers', { data: { uids: ids } })
  },
  // 获取用户部门和角色
  getUserDeptsRoles: (userId: number) => {
    return request.get('/sys/user/depts_roles', { params: { uid: userId } })
  },
  // 更新用户部门和角色
  updateUserRoleOrDept: (data: any) => {
    return request.post('/sys/user/update_role_or_dept', data)
  },
  // 获取部门列表
  getDeptList: () => {
    return request.get('/sys/dept/list')
  },
  // 获取角色列表
  getRoleList: () => {
    return request.get('/sys/role/list')
  }
}