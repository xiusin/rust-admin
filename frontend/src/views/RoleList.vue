<template>
  <div class="role-list-container">
    <h1>角色管理</h1>
    
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAddRole">新增角色</el-button>
      <el-button type="danger" @click="handleBatchDelete" :disabled="selectedIds.length === 0">批量删除</el-button>
    </div>
    
    <!-- 搜索栏 -->
    <div class="search-bar">
      <el-input v-model="searchForm.roleName" placeholder="角色名称" style="width: 200px; margin-right: 10px;"></el-input>
      <el-input v-model="searchForm.roleKey" placeholder="角色标识" style="width: 200px; margin-right: 10px;"></el-input>
      <el-button type="primary" @click="handleSearch">搜索</el-button>
      <el-button @click="resetSearch">重置</el-button>
    </div>
    
    <!-- 角色列表 -->
    <el-table
      v-loading="loading"
      :data="roleList"
      style="width: 100%"
      @selection-change="handleSelectionChange"
    >
      <el-table-column type="selection" width="55"></el-table-column>
      <el-table-column prop="id" label="角色ID" width="80"></el-table-column>
      <el-table-column prop="role_name" label="角色名称"></el-table-column>
      <el-table-column prop="role_key" label="角色标识"></el-table-column>
      <el-table-column prop="sort" label="排序" width="80"></el-table-column>
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
      <el-table-column label="操作" width="200">
        <template #default="scope">
          <el-button size="small" @click="handleEditRole(scope.row)">编辑</el-button>
          <el-button size="small" type="danger" @click="handleDeleteRole(scope.row.id)">删除</el-button>
          <el-button size="small" type="primary" @click="handleRolePermissions(scope.row)">权限分配</el-button>
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
    
    <!-- 新增/编辑角色对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'add' ? '新增角色' : '编辑角色'"
      width="500px"
    >
      <el-form :model="form" label-width="80px">
        <el-form-item label="角色名称">
          <el-input v-model="form.role_name" placeholder="请输入角色名称"></el-input>
        </el-form-item>
        <el-form-item label="角色标识">
          <el-input v-model="form.role_key" placeholder="请输入角色标识"></el-input>
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="form.sort" :min="0" :max="9999"></el-input-number>
        </el-form-item>
        <el-form-item label="状态">
          <el-switch v-model="form.status" active-value="1" inactive-value="0"></el-switch>
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入备注"></el-input>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 权限分配对话框 -->
    <el-dialog
      v-model="permissionDialogVisible"
      title="权限分配"
      width="800px"
      height="600px"
    >
      <div class="permission-container">
        <h3>{{ currentRole?.role_name }}的权限</h3>
        <el-checkbox v-model="allChecked">全选</el-checkbox>
        <el-tree
          :data="menuTree"
          show-checkbox
          node-key="id"
          default-expand-all
          :checked-keys="checkedMenuIds"
          @check-change="handleMenuCheck"
        >
          <template #default="{ node }">
            {{ node.label }}
          </template>
        </el-tree>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="permissionDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSavePermissions">保存</el-button>
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

.permission-container {
  max-height: 400px;
  overflow-y: auto;
}

.permission-container h3 {
  margin-bottom: 20px;
}

.permission-container .el-checkbox {
  margin-bottom: 15px;
}
</style>
