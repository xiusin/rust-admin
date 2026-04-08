import { request } from './request'

// 部门相关 API 接口
export const deptApi = {
  // 获取部门树结构
  getDeptTree: () => {
    return request.get('/sys/dept/list')
  },
  // 添加部门
  addDept: (data: any) => {
    return request.post('/sys/dept/add', data)
  },
  // 编辑部门
  editDept: (data: any) => {
    return request.put('/sys/dept/edit', data)
  },
  // 删除部门
  deleteDept: (deptId: number) => {
    return request.delete('/sys/dept/del', { params: { dept_id: deptId } })
  },
  // 获取部门列表（用于选择父部门）
  getDeptList: () => {
    return request.get('/sys/dept/list')
  }
}
