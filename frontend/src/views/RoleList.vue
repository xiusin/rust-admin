<template>
  <div class="role-list-container">
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAddRole" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </el-icon>
        新增角色
      </el-button>
      <el-button type="danger" @click="handleBatchDelete" :disabled="selectedIds.length === 0" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
          </svg>
        </el-icon>
        批量删除
      </el-button>
    </div>
    
    <!-- 搜索栏 -->
    <div class="search-card">
      <div class="search-content">
        <el-input v-model="searchForm.roleName" placeholder="角色名称" class="search-input">
          <template #prefix>
            <el-icon class="search-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
            </el-icon>
          </template>
        </el-input>
        <el-input v-model="searchForm.roleKey" placeholder="角色标识" class="search-input">
          <template #prefix>
            <el-icon class="search-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 14h-2v-4H8v-2h2V9h2v2h2v2h-2v4z"/>
              </svg>
            </el-icon>
          </template>
        </el-input>
        <div class="search-actions">
          <el-button type="primary" @click="handleSearch" class="search-button">搜索</el-button>
          <el-button @click="resetSearch" class="reset-button">重置</el-button>
        </div>
      </div>
    </div>
    
    <!-- 角色列表 -->
    <div class="table-card">
      <el-table
        v-loading="loading"
        :data="roleList"
        style="width: 100%"
        @selection-change="handleSelectionChange"
        class="role-table"
      >
        <el-table-column type="selection" width="55" class="table-selection"></el-table-column>
        <el-table-column prop="id" label="角色ID" width="80" class="table-id"></el-table-column>
        <el-table-column prop="role_name" label="角色名称" class="table-name"></el-table-column>
        <el-table-column prop="role_key" label="角色标识" class="table-key"></el-table-column>
        <el-table-column prop="sort" label="排序" width="80" class="table-sort"></el-table-column>
        <el-table-column prop="status" label="状态" width="80" class="table-status">
          <template #default="scope">
            <el-switch
              v-model="scope.row.status"
              active-value="1"
              inactive-value="0"
              @change="handleStatusChange(scope.row)"
              class="status-switch"
            ></el-switch>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="180" class="table-time"></el-table-column>
        <el-table-column label="操作" width="240" class="table-actions">
          <template #default="scope">
            <div class="action-buttons">
              <el-button size="small" @click="handleEditRole(scope.row)" class="edit-button">
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                  </svg>
                </el-icon>
                编辑
              </el-button>
              <el-button size="small" type="danger" @click="handleDeleteRole(scope.row.id)" class="delete-button">
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                  </svg>
                </el-icon>
                删除
              </el-button>
              <el-button size="small" type="primary" @click="handleRolePermissions(scope.row)" class="permission-button">
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 17c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2s-2 .9-2 2v6c0 1.1.9 2 2 2zm6-9h-4V3h-4v5H6v2h5v11h2V7h5V5z"/>
                  </svg>
                </el-icon>
                权限
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </div>
    
    <!-- 分页 -->
    <div class="pagination">
      <el-pagination
        v-model:current-page="pagination.current"
        v-model:page-size="pagination.size"
        :page-sizes="[10, 20, 50, 100]"
        layout="total, sizes, prev, pager, next, jumper"
        :total="pagination.total"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
        class="role-pagination"
      ></el-pagination>
    </div>
    
    <!-- 新增/编辑角色对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'add' ? '新增角色' : '编辑角色'"
      width="500px"
      class="role-dialog"
    >
      <el-form :model="form" label-width="80px" class="role-form">
        <el-form-item label="角色名称" prop="role_name">
          <el-input v-model="form.role_name" placeholder="请输入角色名称" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="角色标识" prop="role_key">
          <el-input v-model="form.role_key" placeholder="请输入角色标识" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="排序" prop="sort">
          <el-input-number v-model="form.sort" :min="0" :max="9999" class="form-input"></el-input-number>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch v-model="form.status" active-value="1" inactive-value="0" class="form-switch"></el-switch>
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入备注" class="form-textarea"></el-input>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false" class="dialog-button cancel-button">取消</el-button>
          <el-button type="primary" @click="handleSubmit" class="dialog-button confirm-button">确定</el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 权限分配对话框 -->
    <el-dialog
      v-model="permissionDialogVisible"
      title="权限分配"
      width="800px"
      class="permission-dialog"
    >
      <div class="permission-container">
        <h3 class="permission-title">{{ currentRole?.role_name }}的权限</h3>
        <el-checkbox v-model="allChecked" class="all-checkbox">全选</el-checkbox>
        <el-tree
          :data="menuTree"
          show-checkbox
          node-key="id"
          default-expand-all
          :checked-keys="checkedMenuIds"
          @check-change="handleMenuCheck"
          class="permission-tree"
        >
          <template #default="{ node }">
            <span class="tree-node-label">{{ node.label }}</span>
          </template>
        </el-tree>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="permissionDialogVisible = false" class="dialog-button cancel-button">取消</el-button>
          <el-button type="primary" @click="handleSavePermissions" class="dialog-button confirm-button">保存</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive, computed } from 'vue'
import { roleApi } from '../api/role'
import { ElMessage } from 'element-plus'

// 响应式数据
const roleList = ref([])
const loading = ref(false)
const selectedIds = ref<number[]>([])
const dialogVisible = ref(false)
const permissionDialogVisible = ref(false)
const dialogType = ref<'add' | 'edit'>('add')
const currentRole = ref<any>(null)
const menuTree = ref<any[]>([])
const checkedMenuIds = ref<number[]>([])

// 搜索表单
const searchForm = reactive({
  roleName: '',
  roleKey: ''
})

// 分页
const pagination = reactive({
  current: 1,
  size: 10,
  total: 0
})

// 表单数据
const form = reactive({
  id: 0,
  role_name: '',
  role_key: '',
  sort: 0,
  status: '1',
  remark: ''
})

// 全选状态
const allChecked = computed({
  get: () => {
    if (menuTree.value.length === 0) return false
    return checkedMenuIds.value.length === menuTree.value.reduce((total, node) => {
      const countChildren = (node: any): number => {
        if (!node.children || node.children.length === 0) return 1
        return 1 + node.children.reduce((acc: number, child: any) => acc + countChildren(child), 0)
      }
      return total + countChildren(node)
    }, 0)
  },
  set: (value) => {
    const getAllMenuIds = (nodes: any[]): number[] => {
      let ids: number[] = []
      nodes.forEach(node => {
        ids.push(node.id)
        if (node.children && node.children.length > 0) {
          ids = [...ids, ...getAllMenuIds(node.children)]
        }
      })
      return ids
    }
    checkedMenuIds.value = value ? getAllMenuIds(menuTree.value) : []
  }
})

// 生命周期
onMounted(() => {
  fetchRoleList()
})

// 方法
const fetchRoleList = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.current,
      size: pagination.size,
      role_name: searchForm.roleName,
      role_key: searchForm.roleKey
    }
    const response = await roleApi.getRoleList(params)
    roleList.value = response.data?.list || []
    pagination.total = response.data?.total || 0
  } catch (error) {
    console.error('获取角色列表失败', error)
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  pagination.current = 1
  fetchRoleList()
}

const resetSearch = () => {
  searchForm.roleName = ''
  searchForm.roleKey = ''
  pagination.current = 1
  fetchRoleList()
}

const handleSizeChange = (size: number) => {
  pagination.size = size
  fetchRoleList()
}

const handleCurrentChange = (current: number) => {
  pagination.current = current
  fetchRoleList()
}

const handleSelectionChange = (val: any[]) => {
  selectedIds.value = val.map(item => item.id)
}

const handleAddRole = () => {
  dialogType.value = 'add'
  Object.assign(form, {
    id: 0,
    role_name: '',
    role_key: '',
    sort: 0,
    status: '1',
    remark: ''
  })
  dialogVisible.value = true
}

const handleEditRole = (row: any) => {
  dialogType.value = 'edit'
  Object.assign(form, {
    id: row.id,
    role_name: row.role_name,
    role_key: row.role_key,
    sort: row.sort,
    status: row.status,
    remark: row.remark
  })
  dialogVisible.value = true
}

const handleSubmit = async () => {
  try {
    if (dialogType.value === 'add') {
      await roleApi.createRole(form)
      ElMessage.success('新增角色成功')
    } else {
      await roleApi.updateRole(form)
      ElMessage.success('编辑角色成功')
    }
    dialogVisible.value = false
    fetchRoleList()
  } catch (error) {
    console.error('保存角色失败', error)
    ElMessage.error('保存失败')
  }
}

const handleDeleteRole = async (id: number) => {
  try {
    await roleApi.deleteRole(id)
    ElMessage.success('删除角色成功')
    fetchRoleList()
  } catch (error) {
    console.error('删除角色失败', error)
    ElMessage.error('删除失败')
  }
}

const handleBatchDelete = async () => {
  if (selectedIds.value.length === 0) return
  try {
    for (const id of selectedIds.value) {
      await roleApi.deleteRole(id)
    }
    ElMessage.success('批量删除成功')
    fetchRoleList()
  } catch (error) {
    console.error('批量删除失败', error)
    ElMessage.error('删除失败')
  }
}

const handleStatusChange = async (row: any) => {
  try {
    await roleApi.updateRole({
      id: row.id,
      role_name: row.role_name,
      role_key: row.role_key,
      sort: row.sort,
      status: row.status,
      remark: row.remark
    })
    ElMessage.success('状态更新成功')
  } catch (error) {
    console.error('状态更新失败', error)
    ElMessage.error('更新失败')
    // 恢复原状态
    fetchRoleList()
  }
}

const handleRolePermissions = async (role: any) => {
  currentRole.value = role
  try {
    // 获取菜单树
    const menuResponse = await roleApi.getMenuTree()
    menuTree.value = menuResponse.data || []
    
    // 获取角色已有权限
    const permissionResponse = await roleApi.getRoleMenus(role.id)
    checkedMenuIds.value = permissionResponse.data?.map((menu: any) => menu.id) || []
    
    permissionDialogVisible.value = true
  } catch (error) {
    console.error('获取权限失败', error)
    ElMessage.error('获取权限失败')
  }
}

const handleMenuCheck = (data: any, checked: boolean) => {
  // 处理菜单勾选逻辑
  const handleChildNodes = (node: any, check: boolean) => {
    if (node.children && node.children.length > 0) {
      node.children.forEach((child: any) => {
        const index = checkedMenuIds.value.indexOf(child.id)
        if (check && index === -1) {
          checkedMenuIds.value.push(child.id)
        } else if (!check && index !== -1) {
          checkedMenuIds.value.splice(index, 1)
        }
        handleChildNodes(child, check)
      })
    }
  }
  
  if (checked) {
    if (!checkedMenuIds.value.includes(data.id)) {
      checkedMenuIds.value.push(data.id)
    }
    handleChildNodes(data, true)
  } else {
    const index = checkedMenuIds.value.indexOf(data.id)
    if (index !== -1) {
      checkedMenuIds.value.splice(index, 1)
    }
    handleChildNodes(data, false)
  }
}

const handleSavePermissions = async () => {
  if (!currentRole.value) return
  try {
    // 调用API保存权限
    await roleApi.updateRole({
      ...currentRole.value,
      menu_ids: checkedMenuIds.value
    })
    
    ElMessage.success('权限保存成功')
    permissionDialogVisible.value = false
  } catch (error) {
    console.error('保存权限失败', error)
    ElMessage.error('保存失败')
  }
}
</script>

<style scoped>
.role-list-container {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

/* 操作栏 */
.action-bar {
  margin-bottom: var(--spacing-lg);
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
}

.action-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
}

.action-button:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow);
}

.button-icon {
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 搜索卡片 */
.search-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.search-card:hover {
  box-shadow: var(--shadow);
}

.search-content {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
  flex-wrap: wrap;
}

.search-input {
  width: 240px;
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
}

.search-input:focus-within {
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

.search-icon {
  color: var(--text-placeholder);
  transition: color var(--transition-fast);
}

.search-input:focus-within .search-icon {
  color: var(--primary);
}

.search-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.search-button, .reset-button {
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
}

.search-button:hover, .reset-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 表格卡片 */
.table-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.table-card:hover {
  box-shadow: var(--shadow);
}

.role-table {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.role-table .el-table__header-wrapper th {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-sm);
  padding: var(--spacing-md);
  border-bottom: 2px solid var(--primary);
}

.role-table .el-table__row {
  transition: all var(--transition-fast);
}

.role-table .el-table__row:hover {
  background-color: var(--bg-secondary);
}

.role-table .el-table__row:nth-child(even) {
  background-color: var(--bg-lighter);
}

.table-actions {
  text-align: center;
}

.action-buttons {
  display: flex;
  gap: var(--spacing-sm);
  justify-content: center;
}

.edit-button, .delete-button, .permission-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
  border-radius: var(--radius);
  padding: var(--spacing-xs) var(--spacing-sm);
}

.edit-button:hover, .delete-button:hover, .permission-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.status-switch {
  transition: all var(--transition-fast);
}

/* 分页 */
.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: var(--spacing-lg);
}

.role-pagination {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.role-pagination .el-pagination__item {
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.role-pagination .el-pagination__item:hover {
  color: var(--primary);
  border-color: var(--primary);
}

.role-pagination .el-pagination__item.is-active {
  background-color: var(--primary);
  border-color: var(--primary);
}

/* 对话框 */
.role-dialog, .permission-dialog {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.role-dialog .el-dialog__header, .permission-dialog .el-dialog__header {
  background-color: var(--bg-tertiary);
  padding: var(--spacing-lg);
}

.role-dialog .el-dialog__title, .permission-dialog .el-dialog__title {
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-lg);
}

.role-dialog .el-dialog__body, .permission-dialog .el-dialog__body {
  padding: var(--spacing-lg);
}

.role-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-input, .form-textarea, .form-switch {
  width: 100%;
  transition: all var(--transition-fast);
  border-radius: var(--radius);
}

.form-input:focus, .form-textarea:focus {
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
  transform: translateY(-1px);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
}

.dialog-button {
  transition: all var(--transition-fast);
  border-radius: var(--radius);
  padding: var(--spacing-sm) var(--spacing-lg);
}

.dialog-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 权限分配 */
.permission-container {
  max-height: 500px;
  overflow-y: auto;
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius);
}

.permission-title {
  margin-bottom: var(--spacing-lg);
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-lg);
}

.all-checkbox {
  margin-bottom: var(--spacing-md);
  font-weight: 500;
  color: var(--text-primary);
}

.permission-tree {
  background-color: var(--bg-primary);
  border-radius: var(--radius);
  padding: var(--spacing-md);
  box-shadow: var(--shadow-sm);
}

.tree-node-label {
  transition: color var(--transition-fast);
}

.permission-tree .el-tree-node:hover .tree-node-label {
  color: var(--primary);
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .role-list-container {
    padding: var(--spacing-md);
  }
  
  .search-content {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-input {
    width: 100%;
  }
  
  .search-actions {
    justify-content: center;
  }
  
  .action-bar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .action-button {
    justify-content: center;
  }
  
  .table-card {
    padding: var(--spacing-md);
  }
  
  .action-buttons {
    flex-direction: column;
    align-items: center;
  }
  
  .edit-button, .delete-button, .permission-button {
    width: 100%;
    justify-content: center;
  }
  
  .role-dialog, .permission-dialog {
    width: 95% !important;
  }
  
  .permission-container {
    max-height: 300px;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .role-list-container {
    padding: var(--spacing-md);
  }
  
  .search-input {
    width: 200px;
  }
  
  .table-card {
    padding: var(--spacing-md);
  }
}
</style>
