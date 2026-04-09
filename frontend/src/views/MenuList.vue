<template>
  <div class="menu-list">
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAdd" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </el-icon>
        新增菜单
      </el-button>
    </div>
    
    <!-- 菜单树 -->
    <div class="tree-card">
      <div class="tree-header">
        <h3 class="tree-title">菜单结构</h3>
        <p class="tree-description">点击节点展开/收起，悬停显示操作按钮</p>
      </div>
      <div class="menu-tree-container">
        <el-tree
          v-loading="loading"
          :data="menuTree"
          node-key="id"
          :props="defaultProps"
          @node-click="handleNodeClick"
          @node-contextmenu="handleContextMenu"
          class="menu-tree"
        >
          <template #default="{ data }">
            <div class="tree-node">
              <div class="node-info">
                <el-icon v-if="data.meta.icon" class="node-icon" v-html="getIcon(data.meta.icon)"></el-icon>
                <span class="node-title">{{ data.meta.title }}</span>
                <span class="node-type" :class="`type-${data.menu_type}`">
                  {{ data.menu_type === 'M' ? '目录' : data.menu_type === 'C' ? '菜单' : '按钮' }}
                </span>
              </div>
              <div class="node-actions">
                <el-button size="small" @click.stop="handleEdit(data)" class="action-button edit-button">
                  <el-icon class="button-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                    </svg>
                  </el-icon>
                  编辑
                </el-button>
                <el-button size="small" type="danger" @click.stop="handleDelete(data.id)" class="action-button delete-button">
                  <el-icon class="button-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                    </svg>
                  </el-icon>
                  删除
                </el-button>
                <el-button size="small" @click.stop="handleAdd(data.id)" class="action-button add-button">
                  <el-icon class="button-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                    </svg>
                  </el-icon>
                  新增子菜单
                </el-button>
              </div>
            </div>
          </template>
        </el-tree>
      </div>
    </div>
    
    <!-- 新增/编辑菜单对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
      class="menu-dialog"
    >
      <el-form :model="form" :rules="rules" ref="formRef" label-width="100px" class="menu-form">
        <el-form-item label="父菜单" prop="pid">
          <el-select v-model="form.pid" placeholder="请选择父菜单" class="form-select">
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
          <el-input v-model="form.meta.title" placeholder="请输入菜单名称" class="form-input" />
        </el-form-item>
        
        <el-form-item label="菜单类型" prop="menu_type">
          <el-select v-model="form.menu_type" placeholder="请选择菜单类型" class="form-select">
            <el-option label="目录" value="M" />
            <el-option label="菜单" value="C" />
            <el-option label="按钮" value="F" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="路由路径" prop="path">
          <el-input v-model="form.path" placeholder="请输入路由路径" class="form-input" />
        </el-form-item>
        
        <el-form-item label="组件路径" prop="component">
          <el-input v-model="form.component" placeholder="请输入组件路径" class="form-input" />
        </el-form-item>
        
        <el-form-item label="排序" prop="order">
          <el-input-number v-model="form.order" :min="0" class="form-input" />
        </el-form-item>
        
        <el-form-item label="图标" prop="meta.icon">
          <el-input v-model="form.meta.icon" placeholder="请输入图标名称" class="form-input" />
        </el-form-item>
        
        <el-form-item label="权限标识" prop="perms">
          <el-input v-model="form.perms" placeholder="请输入权限标识" class="form-input" />
        </el-form-item>
        
        <el-form-item label="状态" prop="status">
          <el-radio-group v-model="form.status" class="status-radio">
            <el-radio label="0" class="radio-item">正常</el-radio>
            <el-radio label="1" class="radio-item">禁用</el-radio>
          </el-radio-group>
        </el-form-item>
        
        <el-form-item label="备注">
          <el-input type="textarea" v-model="form.remark" placeholder="请输入备注" class="form-textarea" />
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
import { request } from '../api/request'

// 图标映射
const getIcon = (iconName: string) => {
  const icons: Record<string, string> = {
    home: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>',
    dashboard: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>',
    settings: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    user: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>',
    building: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14h-4v-4h4v4zm0-6h-4v-2h4v2zm0-4h-4V5h4v2z"/></svg>',
    role: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>',
    menu: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/></svg>',
    lock: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 17c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2s-2 .9-2 2v6c0 1.1.9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H8.9V6z"/></svg>',
    api: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    category: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M11 7h6v2h-6zm0 4h6v2h-6zm0 4h6v2h-6zM7 7h2v2H7zm0 4h2v2H7zm0 4h2v2H7zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14z"/></svg>',
    monitor: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M20 3H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h3l-1 1v1h10v-1l-1-1h3c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 13H4V5h16v11z"/></svg>',
    log: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/></svg>',
    operation: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    server: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M4 2h16c1.1 0 2 .9 2 2v16c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2zm0 2v16h16V4H4zm2 4h12v2H6zm0 4h12v2H6zm0 4h12v2H6z"/></svg>',
    job: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 14h-2v-4H8v-2h2V9h2v2h2v2h-2v4z"/></svg>',
    about: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>',
    info: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>',
    team: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/></svg>'
  };
  return icons[iconName] || '';
};

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

.menu-tree-container {
  min-height: 500px;
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius);
}

.menu-tree {
  background-color: var(--bg-primary);
  border-radius: var(--radius);
  padding: var(--spacing-md);
  box-shadow: var(--shadow-sm);
}

.tree-node {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.tree-node:hover {
  background-color: var(--bg-secondary);
}

.node-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  flex: 1;
}

.node-icon {
  font-size: 16px;
  color: var(--primary);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
}

.node-title {
  font-size: var(--font-size);
  color: var(--text-primary);
  font-weight: 500;
  transition: color var(--transition-fast);
  flex: 1;
}

.tree-node:hover .node-title {
  color: var(--primary);
}

.node-type {
  font-size: var(--font-size-xs);
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.type-M {
  background-color: var(--primary-light);
  color: var(--primary-dark);
}

.type-C {
  background-color: var(--success-light);
  color: var(--success-dark);
}

.type-F {
  background-color: var(--warning-light);
  color: var(--warning-dark);
}

.node-actions {
  display: none;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
}

.tree-node:hover .node-actions {
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
.menu-dialog {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.menu-dialog .el-dialog__header {
  background-color: var(--bg-tertiary);
  padding: var(--spacing-lg);
}

.menu-dialog .el-dialog__title {
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-lg);
}

.menu-dialog .el-dialog__body {
  padding: var(--spacing-lg);
}

.menu-form {
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
.menu-tree .el-tree-node {
  transition: all var(--transition-fast);
}

.menu-tree .el-tree-node__content {
  height: 48px;
  align-items: center;
}

.menu-tree .el-tree-node__expand-icon {
  transition: all var(--transition-fast);
}

.menu-tree .el-tree-node__expand-icon:hover {
  color: var(--primary);
  transform: scale(1.1);
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .menu-list {
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
  
  .menu-tree-container {
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
  
  .menu-dialog {
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
  
  .node-info {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-xs);
  }
  
  .node-type {
    align-self: flex-start;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .menu-list {
    padding: var(--spacing-md);
  }
  
  .tree-card {
    padding: var(--spacing-md);
  }
  
  .menu-tree-container {
    min-height: 450px;
  }
}
</style>