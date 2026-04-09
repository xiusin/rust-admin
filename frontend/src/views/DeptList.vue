<template>
  <div class="dept-management">
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAdd" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </el-icon>
        新增部门
      </el-button>
    </div>
    
    <!-- 部门树 -->
    <div class="tree-card">
      <div class="tree-header">
        <h3 class="tree-title">部门结构</h3>
        <p class="tree-description">点击节点展开/收起，悬停显示操作按钮</p>
      </div>
      <div class="dept-tree-container">
        <el-tree
          :data="deptTree"
          :props="treeProps"
          :expand-on-click-node="false"
          node-key="deptId"
          @node-click="handleNodeClick"
          @node-contextmenu="handleContextMenu"
          class="dept-tree"
        >
          <template #default="{ data }">
            <div class="tree-node-content">
              <span class="node-name">{{ data.deptName }}</span>
              <div class="node-actions">
                <el-button
                  size="small"
                  @click.stop="handleEdit(data)"
                  class="action-button edit-button"
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
                  @click.stop="handleDelete(data.deptId)"
                  type="danger"
                  class="action-button delete-button"
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
                  @click.stop="handleAddChild(data)"
                  class="action-button add-button"
                >
                  <el-icon class="button-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                    </svg>
                  </el-icon>
                  新增子部门
                </el-button>
              </div>
            </div>
          </template>
        </el-tree>
      </div>
    </div>
    
    <!-- 新增/编辑部门对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
      class="dept-dialog"
    >
      <el-form :model="formData" :rules="rules" ref="formRef" label-width="120px" class="dept-form">
        <el-form-item label="部门名称" prop="deptName">
          <el-input v-model="formData.deptName" placeholder="请输入部门名称" class="form-input" />
        </el-form-item>
        <el-form-item label="父部门" prop="parentId">
          <el-select v-model="formData.parentId" placeholder="请选择父部门" class="form-select">
            <el-option
              v-for="dept in deptOptions"
              :key="dept.deptId"
              :label="dept.deptName"
              :value="dept.deptId"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="formData.order" :min="0" class="form-input" />
        </el-form-item>
        <el-form-item label="负责人">
          <el-input v-model="formData.leader" placeholder="请输入负责人ID" class="form-input" />
        </el-form-item>
        <el-form-item label="联系电话">
          <el-input v-model="formData.phone" placeholder="请输入联系电话" class="form-input" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="formData.email" placeholder="请输入邮箱" class="form-input" />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="formData.status" class="status-radio">
            <el-radio label="0" class="radio-item">正常</el-radio>
            <el-radio label="1" class="radio-item">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="备注">
          <el-input
            v-model="formData.remark"
            type="textarea"
            placeholder="请输入备注"
            :rows="3"
            class="form-textarea"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false" class="dialog-button cancel-button">取消</el-button>
          <el-button type="primary" @click="handleSubmit" class="dialog-button confirm-button">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
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

/* 树卡片 */
.tree-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  transition: all var(--transition);
}

.tree-card:hover {
  box-shadow: var(--shadow);
}

.tree-header {
  margin-bottom: var(--spacing-lg);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.tree-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.tree-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
}

.dept-tree-container {
  min-height: 500px;
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius);
}

.dept-tree {
  background-color: var(--bg-primary);
  border-radius: var(--radius);
  padding: var(--spacing-md);
  box-shadow: var(--shadow-sm);
}

.tree-node-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.tree-node-content:hover {
  background-color: var(--bg-secondary);
}

.node-name {
  font-size: var(--font-size);
  color: var(--text-primary);
  font-weight: 500;
  transition: color var(--transition-fast);
}

.tree-node-content:hover .node-name {
  color: var(--primary);
}

.node-actions {
  display: none;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
}

.tree-node-content:hover .node-actions {
  display: flex;
  animation: fadeIn var(--transition-fast);
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.edit-button, .delete-button, .add-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
  border-radius: var(--radius);
  padding: var(--spacing-xs) var(--spacing-sm);
  font-size: var(--font-size-sm);
}

.edit-button:hover, .delete-button:hover, .add-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 对话框 */
.dept-dialog {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.dept-dialog .el-dialog__header {
  background-color: var(--bg-tertiary);
  padding: var(--spacing-lg);
}

.dept-dialog .el-dialog__title {
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-lg);
}

.dept-dialog .el-dialog__body {
  padding: var(--spacing-lg);
}

.dept-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-input, .form-select, .form-textarea {
  width: 100%;
  transition: all var(--transition-fast);
  border-radius: var(--radius);
}

.form-input:focus, .form-select:focus, .form-textarea:focus {
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
  transform: translateY(-1px);
}

.status-radio {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
}

.radio-item {
  transition: all var(--transition-fast);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius);
}

.radio-item:hover {
  background-color: var(--bg-secondary);
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

/* 树节点样式 */
.dept-tree .el-tree-node {
  transition: all var(--transition-fast);
}

.dept-tree .el-tree-node__content {
  height: 48px;
  align-items: center;
}

.dept-tree .el-tree-node__expand-icon {
  transition: all var(--transition-fast);
}

.dept-tree .el-tree-node__expand-icon:hover {
  color: var(--primary);
  transform: scale(1.1);
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .dept-management {
    padding: var(--spacing-md);
  }
  
  .action-bar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .action-button {
    justify-content: center;
  }
  
  .tree-card {
    padding: var(--spacing-md);
  }
  
  .dept-tree-container {
    min-height: 400px;
    padding: var(--spacing-sm);
  }
  
  .node-actions {
    flex-direction: column;
    align-items: center;
  }
  
  .edit-button, .delete-button, .add-button {
    width: 100%;
    justify-content: center;
  }
  
  .dept-dialog {
    width: 95% !important;
  }
  
  .status-radio {
    flex-direction: column;
    align-items: stretch;
  }
  
  .radio-item {
    width: 100%;
    text-align: center;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .dept-management {
    padding: var(--spacing-md);
  }
  
  .tree-card {
    padding: var(--spacing-md);
  }
  
  .dept-tree-container {
    min-height: 450px;
  }
}
</style>
