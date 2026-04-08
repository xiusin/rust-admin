<template>
  <div class="user-list-container">
    <h1>用户管理</h1>
    
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAddUser">新增用户</el-button>
      <el-button type="danger" @click="handleBatchDelete" :disabled="selectedIds.length === 0">批量删除</el-button>
    </div>
    
    <!-- 搜索栏 -->
    <div class="search-bar">
      <el-input v-model="searchForm.username" placeholder="用户名" style="width: 200px; margin-right: 10px;"></el-input>
      <el-input v-model="searchForm.nickname" placeholder="昵称" style="width: 200px; margin-right: 10px;"></el-input>
      <el-button type="primary" @click="handleSearch">搜索</el-button>
      <el-button @click="resetSearch">重置</el-button>
    </div>
    
    <!-- 用户列表 -->
    <el-table
      v-loading="loading"
      :data="userList"
      style="width: 100%"
      @selection-change="handleSelectionChange"
    >
      <el-table-column type="selection" width="55"></el-table-column>
      <el-table-column prop="id" label="用户ID" width="80"></el-table-column>
      <el-table-column prop="user_name" label="用户名"></el-table-column>
      <el-table-column prop="nick_name" label="昵称"></el-table-column>
      <el-table-column prop="email" label="邮箱"></el-table-column>
      <el-table-column prop="phonenumber" label="手机号"></el-table-column>
      <el-table-column prop="dept_name" label="部门"></el-table-column>
      <el-table-column prop="role_names" label="角色"></el-table-column>
      <el-table-column prop="status" label="状态" width="80">
        <template #default="scope">
          <el-switch
            v-model="scope.row.status"
            active-value="1"
            inactive-value="0"
            @change="handleStatusChange(scope.row)"
          ></el-switch>
        </template>
      </el-table-column>
      <el-table-column prop="create_time" label="创建时间" width="180"></el-table-column>
      <el-table-column label="操作" width="180">
        <template #default="scope">
          <el-button size="small" @click="handleEditUser(scope.row)">编辑</el-button>
          <el-button size="small" type="danger" @click="handleDeleteUser(scope.row.id)">删除</el-button>
          <el-button size="small" @click="handleUserRoles(scope.row)">角色管理</el-button>
        </template>
      </el-table-column>
    </el-table>
    
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
      ></el-pagination>
    </div>
    
    <!-- 新增/编辑用户对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'add' ? '新增用户' : '编辑用户'"
      width="500px"
    >
      <el-form :model="form" label-width="80px">
        <el-form-item label="用户名">
          <el-input v-model="form.user_name" placeholder="请输入用户名"></el-input>
        </el-form-item>
        <el-form-item label="昵称">
          <el-input v-model="form.nick_name" placeholder="请输入昵称"></el-input>
        </el-form-item>
        <el-form-item label="密码" v-if="dialogType === 'add'">
          <el-input v-model="form.password" type="password" placeholder="请输入密码"></el-input>
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="form.email" placeholder="请输入邮箱"></el-input>
        </el-form-item>
        <el-form-item label="手机号">
          <el-input v-model="form.phonenumber" placeholder="请输入手机号"></el-input>
        </el-form-item>
        <el-form-item label="部门">
          <el-select v-model="form.dept_id" placeholder="请选择部门">
            <el-option
              v-for="dept in deptList"
              :key="dept.id"
              :label="dept.dept_name"
              :value="dept.id"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="form.role_ids" multiple placeholder="请选择角色">
            <el-option
              v-for="role in roleList"
              :key="role.id"
              :label="role.role_name"
              :value="role.id"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-switch v-model="form.status" active-value="1" inactive-value="0"></el-switch>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 角色管理对话框 -->
    <el-dialog
      v-model="roleDialogVisible"
      title="角色管理"
      width="600px"
    >
      <div>
        <h3>{{ currentUser?.nick_name }}的角色</h3>
        <el-checkbox-group v-model="selectedRoles">
          <el-checkbox
            v-for="role in roleList"
            :key="role.id"
            :label="role.id"
          >
            {{ role.role_name }}
          </el-checkbox>
        </el-checkbox-group>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="roleDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSaveRoles">保存</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { userApi } from '../api/user'
import { ElMessage } from 'element-plus'

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
  }
}

const fetchRoleList = async () => {
  try {
    const response = await userApi.getRoleList()
    roleList.value = response.data || []
  } catch (error) {
    console.error('获取角色列表失败', error)
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
    await userApi.deleteUser([id])
    ElMessage.success('删除用户成功')
    fetchUserList()
  } catch (error) {
    console.error('删除用户失败', error)
    ElMessage.error('删除失败')
  }
}

const handleBatchDelete = async () => {
  if (selectedIds.value.length === 0) return
  try {
    await userApi.deleteUser(selectedIds.value)
    ElMessage.success('批量删除成功')
    fetchUserList()
  } catch (error) {
    console.error('批量删除失败', error)
    ElMessage.error('删除失败')
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
  padding: 20px;
  background-color: #f5f7fa;
  min-height: 100vh;
}

h1 {
  margin-bottom: 20px;
  color: #303133;
}

.action-bar {
  margin-bottom: 20px;
}

.search-bar {
  margin-bottom: 20px;
  padding: 20px;
  background-color: white;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.pagination {
  margin-top: 20px;
  text-align: right;
}

.dialog-footer {
  text-align: right;
}
</style>