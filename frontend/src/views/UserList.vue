<template>
  <div class="user-list-container">
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAddUser" class="add-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </el-icon>
        新增用户
      </el-button>
      <el-button 
        type="danger" 
        @click="handleBatchDelete" 
        :disabled="selectedIds.length === 0"
        class="batch-delete-button"
      >
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
          </svg>
        </el-icon>
        批量删除
      </el-button>
    </div>
    
    <!-- 搜索栏 -->
    <div class="search-bar card">
      <div class="search-form">
        <el-input 
          v-model="searchForm.username" 
          placeholder="用户名" 
          class="search-input"
        >
          <template #prefix>
            <el-icon class="search-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
            </el-icon>
          </template>
        </el-input>
        
        <el-input 
          v-model="searchForm.nickname" 
          placeholder="昵称" 
          class="search-input"
        >
          <template #prefix>
            <el-icon class="search-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
              </svg>
            </el-icon>
          </template>
        </el-input>
        
        <div class="search-buttons">
          <el-button type="primary" @click="handleSearch" class="search-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
            </el-icon>
            搜索
          </el-button>
          <el-button @click="resetSearch" class="reset-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
            </el-icon>
            重置
          </el-button>
        </div>
      </div>
    </div>
    
    <!-- 用户列表 -->
    <div class="table-container card">
      <el-table
        v-loading="loading"
        :data="userList"
        style="width: 100%"
        @selection-change="handleSelectionChange"
        class="user-table"
        :row-class-name="tableRowClassName"
      >
        <el-table-column type="selection" width="55" class="selection-column"></el-table-column>
        <el-table-column prop="id" label="用户ID" width="80" class="id-column"></el-table-column>
        <el-table-column prop="user_name" label="用户名" class="username-column"></el-table-column>
        <el-table-column prop="nick_name" label="昵称" class="nickname-column"></el-table-column>
        <el-table-column prop="email" label="邮箱" class="email-column"></el-table-column>
        <el-table-column prop="phonenumber" label="手机号" class="phone-column"></el-table-column>
        <el-table-column prop="dept_name" label="部门" class="dept-column"></el-table-column>
        <el-table-column prop="role_names" label="角色" class="role-column"></el-table-column>
        <el-table-column prop="status" label="状态" width="100" class="status-column">
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
        <el-table-column prop="create_time" label="创建时间" width="180" class="time-column"></el-table-column>
        <el-table-column label="操作" width="220" class="action-column">
          <template #default="scope">
            <div class="action-buttons">
              <el-button 
                size="small" 
                @click="handleEditUser(scope.row)"
                class="edit-button"
              >
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                  </svg>
                </el-icon>
                编辑
              </el-button>
              <el-button 
                size="small" 
                type="danger" 
                @click="handleDeleteUser(scope.row.id)"
                class="delete-button"
              >
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                  </svg>
                </el-icon>
                删除
              </el-button>
              <el-button 
                size="small" 
                @click="handleUserRoles(scope.row)"
                class="role-button"
              >
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                  </svg>
                </el-icon>
                角色
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
        class="pagination-control"
      ></el-pagination>
    </div>
    
    <!-- 新增/编辑用户对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'add' ? '新增用户' : '编辑用户'"
      width="500px"
      class="user-dialog"
      :close-on-click-modal="false"
    >
      <el-form :model="form" label-width="80px" class="user-form">
        <el-form-item label="用户名" :required="true">
          <el-input v-model="form.user_name" placeholder="请输入用户名" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="昵称" :required="true">
          <el-input v-model="form.nick_name" placeholder="请输入昵称" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="密码" v-if="dialogType === 'add'" :required="true">
          <el-input v-model="form.password" type="password" placeholder="请输入密码" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="form.email" placeholder="请输入邮箱" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="手机号">
          <el-input v-model="form.phonenumber" placeholder="请输入手机号" class="form-input"></el-input>
        </el-form-item>
        <el-form-item label="部门" :required="true">
          <el-select v-model="form.dept_id" placeholder="请选择部门" class="form-select">
            <el-option
              v-for="dept in deptList"
              :key="dept.id"
              :label="dept.dept_name"
              :value="dept.id"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="角色" :required="true">
          <el-select v-model="form.role_ids" multiple placeholder="请选择角色" class="form-select">
            <el-option
              v-for="role in roleList"
              :key="role.id"
              :label="role.role_name"
              :value="role.id"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-switch v-model="form.status" active-value="1" inactive-value="0" class="form-switch"></el-switch>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false" class="cancel-button">取消</el-button>
          <el-button type="primary" @click="handleSubmit" class="submit-button">确定</el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 角色管理对话框 -->
    <el-dialog
      v-model="roleDialogVisible"
      title="角色管理"
      width="600px"
      class="role-dialog"
      :close-on-click-modal="false"
    >
      <div class="role-dialog-content">
        <h3 class="role-dialog-title">{{ currentUser?.nick_name }}的角色</h3>
        <el-checkbox-group v-model="selectedRoles" class="role-checkbox-group">
          <el-checkbox
            v-for="role in roleList"
            :key="role.id"
            :label="role.id"
            class="role-checkbox"
          >
            {{ role.role_name }}
          </el-checkbox>
        </el-checkbox-group>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="roleDialogVisible = false" class="cancel-button">取消</el-button>
          <el-button type="primary" @click="handleSaveRoles" class="submit-button">保存</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { userApi } from '../api/user'
import { ElMessage, ElMessageBox } from 'element-plus'

// 响应式数据
const userList = ref([])
const loading = ref(false)
const selectedIds = ref<number[]>([])
const dialogVisible = ref(false)
const roleDialogVisible = ref(false)
const dialogType = ref<'add' | 'edit'>('add')
const currentUser = ref<any>(null)
const selectedRoles = ref<number[]>([])

// 搜索表单
const searchForm = reactive({
  username: '',
  nickname: ''
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
  user_name: '',
  nick_name: '',
  password: '',
  email: '',
  phonenumber: '',
  dept_id: 0,
  role_ids: [] as number[],
  dept_ids: [] as number[],
  status: '1'
})

// 部门和角色列表
const deptList = ref<any[]>([])
const roleList = ref<any[]>([])

// 表格行样式
const tableRowClassName = ({ rowIndex }: { rowIndex: number }) => {
  return rowIndex % 2 === 0 ? 'even-row' : 'odd-row'
}

// 生命周期
onMounted(() => {
  fetchUserList()
  fetchDeptList()
  fetchRoleList()
})

// 方法
const fetchUserList = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.current,
      size: pagination.size,
      username: searchForm.username,
      nickname: searchForm.nickname
    }
    const response = await userApi.getUserList(params)
    userList.value = response.data?.list || []
    pagination.total = response.data?.total || 0
  } catch (error) {
    console.error('获取用户列表失败', error)
    ElMessage.error('获取用户列表失败')
  } finally {
    loading.value = false
  }
}

const fetchDeptList = async () => {
  try {
    const response = await userApi.getDeptList()
    deptList.value = response.data || []
  } catch (error) {
    console.error('获取部门列表失败', error)
    ElMessage.error('获取部门列表失败')
  }
}

const fetchRoleList = async () => {
  try {
    const response = await userApi.getRoleList()
    roleList.value = response.data || []
  } catch (error) {
    console.error('获取角色列表失败', error)
    ElMessage.error('获取角色列表失败')
  }
}

const handleSearch = () => {
  pagination.current = 1
  fetchUserList()
}

const resetSearch = () => {
  searchForm.username = ''
  searchForm.nickname = ''
  pagination.current = 1
  fetchUserList()
}

const handleSizeChange = (size: number) => {
  pagination.size = size
  fetchUserList()
}

const handleCurrentChange = (current: number) => {
  pagination.current = current
  fetchUserList()
}

const handleSelectionChange = (val: any[]) => {
  selectedIds.value = val.map(item => item.id)
}

const handleAddUser = () => {
  dialogType.value = 'add'
  Object.assign(form, {
    id: 0,
    user_name: '',
    nick_name: '',
    password: '',
    email: '',
    phonenumber: '',
    dept_id: 0,
    role_ids: [],
    dept_ids: [],
    status: '1'
  })
  dialogVisible.value = true
}

const handleEditUser = (row: any) => {
  dialogType.value = 'edit'
  Object.assign(form, {
    id: row.id,
    user_name: row.user_name,
    nick_name: row.nick_name,
    email: row.email,
    phonenumber: row.phonenumber,
    dept_id: row.dept_id,
    role_ids: [],
    dept_ids: [row.dept_id],
    status: row.status
  })
  // 获取用户角色
  userApi.getUserDeptsRoles(row.id).then(response => {
    form.role_ids = response.data?.roles?.map((role: any) => role.id) || []
  }).catch(error => {
    console.error('获取用户角色失败', error)
    ElMessage.error('获取用户角色失败')
  })
  dialogVisible.value = true
}

const handleSubmit = async () => {
  try {
    // 确保 dept_ids 包含当前选择的部门
    form.dept_ids = form.dept_id ? [form.dept_id] : []
    
    if (dialogType.value === 'add') {
      await userApi.createUser(form)
      ElMessage.success('新增用户成功')
    } else {
      await userApi.updateUser(form)
      ElMessage.success('编辑用户成功')
    }
    dialogVisible.value = false
    fetchUserList()
  } catch (error) {
    console.error('保存用户失败', error)
    ElMessage.error('保存失败')
  }
}

const handleDeleteUser = async (id: number) => {
  try {
    await ElMessageBox.confirm('确定要删除这个用户吗？', '删除确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await userApi.deleteUser([id])
    ElMessage.success('删除用户成功')
    fetchUserList()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除用户失败', error)
      ElMessage.error('删除失败')
    }
  }
}

const handleBatchDelete = async () => {
  if (selectedIds.value.length === 0) {
    ElMessage.warning('请选择要删除的用户')
    return
  }
  
  try {
    await ElMessageBox.confirm(`确定要删除选中的 ${selectedIds.value.length} 个用户吗？`, '批量删除确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await userApi.deleteUser(selectedIds.value)
    ElMessage.success('批量删除成功')
    fetchUserList()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('批量删除失败', error)
      ElMessage.error('删除失败')
    }
  }
}

const handleStatusChange = async (row: any) => {
  try {
    await userApi.updateUser({
      id: row.id,
      user_name: row.user_name,
      nick_name: row.nick_name,
      email: row.email,
      phonenumber: row.phonenumber,
      dept_id: row.dept_id,
      role_ids: [],
      dept_ids: [row.dept_id],
      status: row.status
    })
    ElMessage.success('状态更新成功')
  } catch (error) {
    console.error('状态更新失败', error)
    ElMessage.error('更新失败')
    // 恢复原状态
    fetchUserList()
  }
}

const handleUserRoles = async (user: any) => {
  currentUser.value = user
  try {
    const response = await userApi.getUserDeptsRoles(user.id)
    selectedRoles.value = response.data?.roles?.map((role: any) => role.id) || []
    roleDialogVisible.value = true
  } catch (error) {
    console.error('获取用户角色失败', error)
    ElMessage.error('获取角色失败')
  }
}

const handleSaveRoles = async () => {
  if (!currentUser.value) return
  try {
    await userApi.updateUserRoleOrDept({
      user_id: currentUser.value.id,
      role_ids: selectedRoles.value,
      dept_ids: [currentUser.value.dept_id]
    })
    ElMessage.success('角色保存成功')
    roleDialogVisible.value = false
    fetchUserList()
  } catch (error) {
    console.error('保存角色失败', error)
    ElMessage.error('保存失败')
  }
}
</script>

<style scoped>
.user-list-container {
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
  flex-wrap: wrap;
}

.add-button,
.batch-delete-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
}

.add-button:hover,
.batch-delete-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: var(--shadow);
}

.batch-delete-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.button-icon {
  font-size: 16px;
  transition: all var(--transition-fast);
}

/* 搜索栏 */
.search-bar {
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.search-form {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
  flex-wrap: wrap;
}

.search-input {
  width: 240px;
  transition: all var(--transition);
}

.search-input:focus-within {
  width: 280px;
}

.search-icon {
  color: var(--text-placeholder);
  transition: color var(--transition-fast);
}

.search-input:focus-within .search-icon {
  color: var(--primary);
}

.search-buttons {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
}

.search-button,
.reset-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
}

.search-button:hover,
.reset-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 表格容器 */
.table-container {
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.user-table {
  border-radius: var(--radius-md);
  overflow: hidden;
  transition: all var(--transition);
}

.user-table .even-row {
  background-color: var(--bg-primary);
}

.user-table .odd-row {
  background-color: var(--bg-secondary);
}

.user-table th {
  background-color: var(--bg-tertiary);
  font-weight: 600;
  color: var(--text-primary);
  padding: var(--spacing-md);
}

.user-table td {
  padding: var(--spacing-md);
  transition: all var(--transition-fast);
}

.user-table tr:hover td {
  background-color: rgba(64, 158, 255, 0.05);
}

.status-switch {
  transition: all var(--transition-fast);
}

/* 操作按钮 */
.action-buttons {
  display: flex;
  gap: var(--spacing-xs);
  align-items: center;
}

.edit-button,
.delete-button,
.role-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
  padding: var(--spacing-xs) var(--spacing-sm);
}

.edit-button:hover {
  background-color: rgba(64, 158, 255, 0.1);
  color: var(--primary);
  transform: translateY(-1px);
}

.delete-button:hover {
  background-color: rgba(245, 108, 108, 0.1);
  transform: translateY(-1px);
}

.role-button:hover {
  background-color: rgba(103, 194, 58, 0.1);
  color: var(--success);
  transform: translateY(-1px);
}

/* 分页 */
.pagination {
  margin-top: var(--spacing-lg);
  display: flex;
  justify-content: flex-end;
  align-items: center;
  transition: all var(--transition);
}

.pagination-control {
  transition: all var(--transition);
}

/* 对话框 */
.user-dialog,
.role-dialog {
  border-radius: var(--radius-md);
  overflow: hidden;
  transition: all var(--transition);
}

.user-form {
  padding: var(--spacing-md);
  transition: all var(--transition);
}

.form-input,
.form-select {
  width: 100%;
  transition: all var(--transition-fast);
}

.form-input:focus,
.form-select:focus {
  border-color: var(--primary);
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
  transform: translateY(-1px);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border);
}

.cancel-button,
.submit-button {
  transition: all var(--transition-fast);
  padding: var(--spacing-sm) var(--spacing-lg);
}

.cancel-button:hover {
  background-color: var(--bg-tertiary);
  transform: translateY(-1px);
}

.submit-button:hover {
  background-color: var(--primary-dark);
  transform: translateY(-1px);
  box-shadow: var(--shadow);
}

/* 角色管理对话框 */
.role-dialog-content {
  padding: var(--spacing-md);
}

.role-dialog-title {
  margin-bottom: var(--spacing-lg);
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.role-checkbox-group {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
}

.role-checkbox {
  margin-right: var(--spacing-md);
  transition: all var(--transition-fast);
}

.role-checkbox:hover {
  color: var(--primary);
  transform: translateY(-1px);
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .user-list-container {
    padding: var(--spacing-md);
  }
  
  .action-bar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .add-button,
  .batch-delete-button {
    width: 100%;
    justify-content: center;
  }
  
  .search-form {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-input {
    width: 100%;
  }
  
  .search-input:focus-within {
    width: 100%;
  }
  
  .search-buttons {
    width: 100%;
    justify-content: space-between;
  }
  
  .search-button,
  .reset-button {
    flex: 1;
    justify-content: center;
  }
  
  .action-buttons {
    flex-direction: column;
    align-items: stretch;
  }
  
  .edit-button,
  .delete-button,
  .role-button {
    justify-content: center;
  }
  
  .pagination {
    justify-content: center;
  }
  
  .role-checkbox-group {
    flex-direction: column;
    gap: var(--spacing-sm);
  }
  
  .role-checkbox {
    margin-right: 0;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .user-list-container {
    padding: var(--spacing-md);
  }
  
  .search-input {
    width: 200px;
  }
  
  .search-input:focus-within {
    width: 240px;
  }
}
</style>