<template>
  <div class="dept-management">
    <h1>部门管理</h1>
    
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAdd">
        <el-icon><Plus /></el-icon> 新增部门
      </el-button>
    </div>
    
    <!-- 部门树 -->
    <div class="dept-tree-container">
      <el-tree
        :data="deptTree"
        :props="treeProps"
        :expand-on-click-node="false"
        node-key="deptId"
        @node-click="handleNodeClick"
        @node-contextmenu="handleContextMenu"
      >
        <template #default="{ data }">
          <div class="tree-node-content">
            <span>{{ data.deptName }}</span>
            <div class="node-actions">
              <el-button
                size="small"
                @click.stop="handleEdit(data)"
                :icon="Edit"
                circle
              />
              <el-button
                size="small"
                @click.stop="handleDelete(data.deptId)"
                :icon="Delete"
                circle
                type="danger"
              />
              <el-button
                size="small"
                @click.stop="handleAddChild(data)"
                :icon="Plus"
                circle
              />
            </div>
          </div>
        </template>
      </el-tree>
    </div>
    
    <!-- 新增/编辑部门对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
    >
      <el-form :model="formData" :rules="rules" ref="formRef" label-width="120px">
        <el-form-item label="部门名称" prop="deptName">
          <el-input v-model="formData.deptName" placeholder="请输入部门名称" />
        </el-form-item>
        <el-form-item label="父部门" prop="parentId">
          <el-select v-model="formData.parentId" placeholder="请选择父部门">
            <el-option
              v-for="dept in deptOptions"
              :key="dept.deptId"
              :label="dept.deptName"
              :value="dept.deptId"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="formData.order" :min="0" />
        </el-form-item>
        <el-form-item label="负责人">
          <el-input v-model="formData.leader" placeholder="请输入负责人ID" />
        </el-form-item>
        <el-form-item label="联系电话">
          <el-input v-model="formData.phone" placeholder="请输入联系电话" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="formData.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="formData.status">
            <el-radio label="0">正常</el-radio>
            <el-radio label="1">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="备注">
          <el-input
            v-model="formData.remark"
            type="textarea"
            placeholder="请输入备注"
            :rows="3"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Edit, Delete } from '@element-plus/icons-vue'
import { deptApi } from '../api/dept'

// 部门树数据
const deptTree = ref<any[]>([])

// 部门选择器选项
const deptOptions = ref<any[]>([])

// 对话框状态
const dialogVisible = ref(false)
const dialogTitle = ref('新增部门')

// 表单数据
const formData = reactive({
  deptId: 0,
  parentId: 0,
  deptName: '',
  order: 0,
  leader: '',
  phone: '',
  email: '',
  status: '0',
  remark: ''
})

// 表单验证规则
const rules = {
  deptName: [
    { required: true, message: '请输入部门名称', trigger: 'blur' }
  ],
  parentId: [
    { required: true, message: '请选择父部门', trigger: 'change' }
  ],
  order: [
    { required: true, message: '请输入排序', trigger: 'blur' }
  ],
  status: [
    { required: true, message: '请选择状态', trigger: 'change' }
  ]
}

const formRef = ref<any>(null)

// 树结构配置
const treeProps = {
  children: 'children',
  label: (data: any) => data.deptName
}

// 获取部门树
const getDeptTree = async () => {
  try {
    const response = await deptApi.getDeptTree()
    deptTree.value = response.data.list
    // 转换为部门选择器选项格式
    deptOptions.value = flattenTree(response.data.list)
  } catch (error) {
    ElMessage.error('获取部门树失败')
  }
}

// 扁平化树结构为选项列表
const flattenTree = (tree: any[]): any[] => {
  let result: any[] = []
  
  const traverse = (nodes: any[], level = 0) => {
    for (const node of nodes) {
      result.push({
        deptId: node.dept.deptId,
        deptName: '├' + '─'.repeat(level * 2) + ' ' + node.dept.deptName,
        parentId: node.dept.parentId
      })
      if (node.children && node.children.length > 0) {
        traverse(node.children, level + 1)
      }
    }
  }
  
  traverse(tree)
  return result
}

// 处理节点点击
const handleNodeClick = (data: any) => {
  console.log('点击节点:', data)
}

// 处理右键菜单
const handleContextMenu = (event: MouseEvent, data: any) => {
  event.preventDefault()
  console.log('右键菜单:', data)
}

// 处理新增部门
const handleAdd = () => {
  resetForm()
  dialogTitle.value = '新增部门'
  dialogVisible.value = true
}

// 处理新增子部门
const handleAddChild = (data: any) => {
  resetForm()
  formData.parentId = data.dept.deptId
  dialogTitle.value = '新增子部门'
  dialogVisible.value = true
}

// 处理编辑部门
const handleEdit = (data: any) => {
  formData.deptId = data.dept.deptId
  formData.parentId = data.dept.parentId
  formData.deptName = data.dept.deptName
  formData.order = data.dept.order
  formData.leader = data.dept.leader
  formData.phone = data.dept.phone
  formData.email = data.dept.email
  formData.status = data.dept.status
  formData.remark = data.dept.remark
  dialogTitle.value = '编辑部门'
  dialogVisible.value = true
}

// 处理删除部门
const handleDelete = async (deptId: number) => {
  try {
    await ElMessageBox.confirm('确定要删除该部门吗？', '删除确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await deptApi.deleteDept(deptId)
    ElMessage.success('删除成功')
    getDeptTree()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

// 重置表单
const resetForm = () => {
  formData.deptId = 0
  formData.parentId = 0
  formData.deptName = ''
  formData.order = 0
  formData.leader = ''
  formData.phone = ''
  formData.email = ''
  formData.status = '0'
  formData.remark = ''
  if (formRef.value) {
    formRef.value.resetFields()
  }
}

// 处理表单提交
const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    
    const formDataToSubmit = { ...formData }
    
    if (formData.deptId) {
      // 编辑部门
      await deptApi.editDept(formDataToSubmit)
      ElMessage.success('编辑成功')
    } else {
      // 新增部门
      await deptApi.addDept(formDataToSubmit)
      ElMessage.success('新增成功')
    }
    
    dialogVisible.value = false
    getDeptTree()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

// 组件挂载时获取部门树
onMounted(() => {
  getDeptTree()
})
</script>

<style scoped>
.dept-management {
  padding: 20px;
}

.action-bar {
  margin-bottom: 20px;
}

.dept-tree-container {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 20px;
  min-height: 400px;
}

.tree-node-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.node-actions {
  display: none;
  gap: 5px;
}

.tree-node-content:hover .node-actions {
  display: flex;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
