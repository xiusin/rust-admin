<template>
  <div class="menu-list">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>菜单管理</span>
          <el-button type="primary" @click="handleAdd">
            <el-icon><Plus /></el-icon>
            新增菜单
          </el-button>
        </div>
      </template>
      
      <el-tree
        v-loading="loading"
        :data="menuTree"
        node-key="id"
        :props="defaultProps"
        @node-click="handleNodeClick"
        @node-contextmenu="handleContextMenu"
      >
        <template #default="{ data }">
          <div class="tree-node">
            <span>{{ data.meta.title }}</span>
            <div class="node-actions">
              <el-button size="small" @click.stop="handleEdit(data)">
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-button size="small" type="danger" @click.stop="handleDelete(data.id)">
                <el-icon><Delete /></el-icon>
              </el-button>
              <el-button size="small" @click.stop="handleAdd(data.id)">
                <el-icon><Plus /></el-icon>
              </el-button>
            </div>
          </div>
        </template>
      </el-tree>
    </el-card>
    
    <!-- 新增/编辑菜单对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
    >
      <el-form :model="form" :rules="rules" ref="formRef" label-width="80px">
        <el-form-item label="父菜单" prop="pid">
          <el-select v-model="form.pid" placeholder="请选择父菜单">
            <el-option label="顶级菜单" value="0" />
            <el-option
              v-for="menu in menuOptions"
              :key="menu.id"
              :label="menu.meta.title"
              :value="menu.id"
            />
          </el-select>
        </el-form-item>
        
        <el-form-item label="菜单名称" prop="meta.title">
          <el-input v-model="form.meta.title" placeholder="请输入菜单名称" />
        </el-form-item>
        
        <el-form-item label="菜单类型" prop="menu_type">
          <el-select v-model="form.menu_type" placeholder="请选择菜单类型">
            <el-option label="目录" value="M" />
            <el-option label="菜单" value="C" />
            <el-option label="按钮" value="F" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="路由路径" prop="path">
          <el-input v-model="form.path" placeholder="请输入路由路径" />
        </el-form-item>
        
        <el-form-item label="组件路径" prop="component">
          <el-input v-model="form.component" placeholder="请输入组件路径" />
        </el-form-item>
        
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="form.order" :min="0" />
        </el-form-item>
        
        <el-form-item label="图标" prop="meta.icon">
          <el-input v-model="form.meta.icon" placeholder="请输入图标名称" />
        </el-form-item>
        
        <el-form-item label="权限标识" prop="perms">
          <el-input v-model="form.perms" placeholder="请输入权限标识" />
        </el-form-item>
        
        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="form.status">
            <el-radio label="0">正常</el-radio>
            <el-radio label="1">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
        
        <el-form-item label="备注">
          <el-input type="textarea" v-model="form.remark" placeholder="请输入备注" />
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
import { request } from '../api/request'

// 定义菜单类型
interface Menu {
  id: string
  pid: string
  path?: string
  order: number
  redirect?: string
  name?: string
  menu_type: string
  component?: string
  status: string
  meta: {
    title: string
    icon?: string
    no_cache: boolean
    breadcrumb: boolean
    affix: boolean
    no_tags_view: boolean
    hidden: boolean
    active_menu: boolean
    can_to: boolean
    always_show: boolean
    i18nkey?: string
  }
  children?: Menu[]
  perms?: string
  remark?: string
}

// 响应式数据
const loading = ref(false)
const menuTree = ref<Menu[]>([])
const menuOptions = ref<Menu[]>([])
const dialogVisible = ref(false)
const dialogTitle = ref('新增菜单')
const formRef = ref()

// 表单数据
const form = reactive({
  id: '',
  pid: '0',
  path: '',
  redirect: '',
  name: '',
  menu_type: 'M',
  component: '',
  status: '0',
  order: 0,
  remark: '',
  perms: '',
  meta: {
    title: '',
    icon: '',
    no_cache: false,
    breadcrumb: true,
    affix: false,
    no_tags_view: false,
    hidden: false,
    active_menu: false,
    can_to: true,
    always_show: false,
    i18nkey: ''
  }
})

// 表单验证规则
const rules = reactive({
  'meta.title': [
    { required: true, message: '请输入菜单名称', trigger: 'blur' }
  ],
  menu_type: [
    { required: true, message: '请选择菜单类型', trigger: 'change' }
  ],
  order: [
    { required: true, message: '请输入排序', trigger: 'blur' }
  ]
})

// 树结构配置
const defaultProps = {
  children: 'children',
  label: (node: Menu) => node.meta.title
}

// 获取菜单树
const getMenuTree = async () => {
  loading.value = true
  try {
    const response = await request.get('/sys/menu/tree')
    menuTree.value = response.data
    // 处理菜单选项（扁平结构）
    const flattenMenu = (menus: Menu[]): Menu[] => {
      let result: Menu[] = []
      menus.forEach(menu => {
        result.push(menu)
        if (menu.children && menu.children.length > 0) {
          result = result.concat(flattenMenu(menu.children))
        }
      })
      return result
    }
    menuOptions.value = flattenMenu(response.data)
  } catch (error) {
    ElMessage.error('获取菜单树失败')
  } finally {
    loading.value = false
  }
}

// 节点点击事件
const handleNodeClick = (data: Menu) => {
  console.log('点击节点:', data)
}

// 右键菜单事件
const handleContextMenu = (event: MouseEvent, data: Menu) => {
  event.preventDefault()
  console.log('右键菜单:', data)
}

// 新增菜单
const handleAdd = (pid: string = '0') => {
  dialogTitle.value = '新增菜单'
  // 重置表单
  Object.assign(form, {
    id: '',
    pid,
    path: '',
    redirect: '',
    name: '',
    menu_type: 'M',
    component: '',
    status: '0',
    order: 0,
    remark: '',
    perms: '',
    meta: {
      title: '',
      icon: '',
      no_cache: false,
      breadcrumb: true,
      affix: false,
      no_tags_view: false,
      hidden: false,
      active_menu: false,
      can_to: true,
      always_show: false,
      i18nkey: ''
    }
  })
  dialogVisible.value = true
}

// 编辑菜单
const handleEdit = (data: Menu) => {
  dialogTitle.value = '编辑菜单'
  // 复制数据到表单
  Object.assign(form, {
    id: data.id,
    pid: data.pid,
    path: data.path || '',
    redirect: data.redirect || '',
    name: data.name || '',
    menu_type: data.menu_type,
    component: data.component || '',
    status: data.status,
    order: data.order,
    remark: data.remark || '',
    perms: data.perms || '',
    meta: { ...data.meta }
  })
  dialogVisible.value = true
}

// 删除菜单
const handleDelete = async (id: string) => {
  try {
    await ElMessageBox.confirm('确定要删除该菜单吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    await request.delete('/sys/menu/del', { params: { id } })
    ElMessage.success('删除成功')
    getMenuTree()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    
    if (form.id) {
      // 编辑
      await request.put('/sys/menu/edit', form)
      ElMessage.success('编辑成功')
    } else {
      // 新增
      await request.post('/sys/menu/add', form)
      ElMessage.success('新增成功')
    }
    
    dialogVisible.value = false
    getMenuTree()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败')
    }
  }
}

// 生命周期钩子
onMounted(() => {
  getMenuTree()
})
</script>

<style scoped>
.menu-list {
  padding: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.tree-node {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.node-actions {
  display: none;
}

.tree-node:hover .node-actions {
  display: flex;
  gap: 8px;
}

.dialog-footer {
  text-align: right;
}
</style>