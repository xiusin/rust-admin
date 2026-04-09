<template>
  <div class="api-permission-list">
    <!-- 操作栏 -->
    <div class="action-bar">
      <el-button type="primary" @click="handleAdd" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
          </svg>
        </el-icon>
        新增权限
      </el-button>
      <el-button type="info" @click="handleRefresh" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
        </el-icon>
        刷新权限
      </el-button>
    </div>
    
    <!-- 搜索栏 -->
    <div class="search-card">
      <div class="search-content">
        <el-form :inline="true" :model="searchForm" class="search-form">
          <el-form-item label="API路径">
            <el-input v-model="searchForm.api" placeholder="请输入API路径" class="search-input">
              <template #prefix>
                <el-icon class="search-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 14h-2v-4H8v-2h2V9h2v2h2v2h-2v4z"/>
                  </svg>
                </el-icon>
              </template>
            </el-input>
          </el-form-item>
          <el-form-item label="方法">
            <el-select v-model="searchForm.method" placeholder="请选择方法" class="search-select">
              <el-option label="GET" value="GET" />
              <el-option label="POST" value="POST" />
              <el-option label="PUT" value="PUT" />
              <el-option label="DELETE" value="DELETE" />
            </el-select>
          </el-form-item>
          <el-form-item label="权限名称">
            <el-input v-model="searchForm.apiname" placeholder="请输入权限名称" class="search-input">
              <template #prefix>
                <el-icon class="search-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                  </svg>
                </el-icon>
              </template>
            </el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="handleSearch" class="search-button">搜索</el-button>
            <el-button @click="resetSearch" class="reset-button">重置</el-button>
          </el-form-item>
        </el-form>
      </div>
    </div>
    
    <!-- 权限列表 -->
    <div class="table-card">
      <el-table :data="apiPermissions" style="width: 100%" class="api-table">
        <el-table-column prop="id" label="ID" width="80" class="table-id" />
        <el-table-column prop="api" label="API路径" min-width="200" class="table-api">
          <template #default="{ row }">
            <span class="api-path">{{ row.api }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="method" label="方法" width="100" class="table-method">
          <template #default="{ row }">
            <el-tag :type="getMethodType(row.method)" class="method-tag">{{ row.method }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="apiname" label="权限名称" min-width="150" class="table-name" />
        <el-table-column prop="sort" label="排序" width="80" class="table-sort" />
        <el-table-column prop="remark" label="备注" min-width="150" class="table-remark" />
        <el-table-column prop="created_at" label="创建时间" width="180" class="table-time" />
        <el-table-column label="操作" width="180" fixed="right" class="table-actions">
          <template #default="{ row }">
            <div class="action-buttons">
              <el-button size="small" @click="handleEdit(row)" class="edit-button">
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                  </svg>
                </el-icon>
                编辑
              </el-button>
              <el-button size="small" type="danger" @click="handleDelete(row.id)" class="delete-button">
                <el-icon class="button-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                  </svg>
                </el-icon>
                删除
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          :total="total"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
          class="api-pagination"
        />
      </div>
    </div>
    
    <!-- 新增/编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
      class="api-dialog"
    >
      <el-form :model="form" :rules="rules" ref="formRef" class="api-form">
        <el-form-item label="API路径" prop="api">
          <el-input v-model="form.api" placeholder="请输入API路径" class="form-input" />
        </el-form-item>
        <el-form-item label="方法" prop="method">
          <el-select v-model="form.method" placeholder="请选择方法" class="form-select">
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
          </el-select>
        </el-form-item>
        <el-form-item label="权限名称" prop="apiname">
          <el-input v-model="form.apiname" placeholder="请输入权限名称" class="form-input" />
        </el-form-item>
        <el-form-item label="排序" prop="sort">
          <el-input-number v-model="form.sort" :min="1" class="form-input" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入备注" class="form-textarea" />
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
import { ref, reactive, onMounted } from 'vue';
import type { FormInstance } from 'element-plus';

// 根据HTTP方法返回标签类型
const getMethodType = (method: string) => {
  const methodTypes: Record<string, string> = {
    GET: 'success',
    POST: 'primary',
    PUT: 'warning',
    DELETE: 'danger'
  };
  return methodTypes[method] || 'info';
};

// 搜索表单
const searchForm = reactive({
  api: '',
  method: '',
  apiname: ''
});

// 分页
const currentPage = ref(1);
const pageSize = ref(10);
const total = ref(0);

// API权限列表
const apiPermissions = ref<any[]>([]);

// 对话框
const dialogVisible = ref(false);
const dialogTitle = ref('新增权限');
const form = reactive({
  id: undefined,
  api: '',
  method: 'GET',
  apiname: '',
  sort: 1,
  remark: ''
});
const formRef = ref<FormInstance>();

// 表单验证规则
const rules = reactive({
  api: [{ required: true, message: '请输入API路径', trigger: 'blur' }],
  method: [{ required: true, message: '请选择方法', trigger: 'change' }],
  apiname: [{ required: true, message: '请输入权限名称', trigger: 'blur' }],
  sort: [{ required: true, message: '请输入排序', trigger: 'blur' }]
});

// 加载API权限列表
const loadApiPermissions = async () => {
  try {
    const response = await fetch(`/api/sys/api_permission/list?page=${currentPage.value}&limit=${pageSize.value}&api=${searchForm.api}&method=${searchForm.method}&apiname=${searchForm.apiname}`);
    const data = await response.json();
    if (data.code === 200) {
      apiPermissions.value = data.data.list;
      total.value = data.data.total;
    }
  } catch (error) {
    console.error('加载API权限失败:', error);
  }
};

// 处理新增
const handleAdd = () => {
  dialogTitle.value = '新增权限';
  Object.assign(form, {
    id: undefined,
    api: '',
    method: 'GET',
    apiname: '',
    sort: 1,
    remark: ''
  });
  dialogVisible.value = true;
};

// 处理编辑
const handleEdit = (row: any) => {
  dialogTitle.value = '编辑权限';
  Object.assign(form, row);
  dialogVisible.value = true;
};

// 处理删除
const handleDelete = async (id: number) => {
  if (confirm('确定要删除这个权限吗？')) {
    try {
      const response = await fetch(`/api/sys/api_permission/delete/${id}`, {
        method: 'DELETE'
      });
      const data = await response.json();
      if (data.code === 200) {
        loadApiPermissions();
      }
    } catch (error) {
      console.error('删除权限失败:', error);
    }
  }
};

// 处理提交
const handleSubmit = async () => {
  if (!formRef.value) return;
  
  try {
    await formRef.value.validate();
    
    const url = form.id ? `/api/sys/api_permission/edit` : `/api/sys/api_permission/add`;
    const method = form.id ? 'PUT' : 'POST';
    
    const response = await fetch(url, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(form)
    });
    
    const data = await response.json();
    if (data.code === 200) {
      dialogVisible.value = false;
      loadApiPermissions();
    }
  } catch (error) {
    console.error('提交失败:', error);
  }
};

// 处理搜索
const handleSearch = () => {
  currentPage.value = 1;
  loadApiPermissions();
};

// 重置搜索
const resetSearch = () => {
  Object.assign(searchForm, {
    api: '',
    method: '',
    apiname: ''
  });
  currentPage.value = 1;
  loadApiPermissions();
};

// 处理分页
const handleSizeChange = (size: number) => {
  pageSize.value = size;
  loadApiPermissions();
};

const handleCurrentChange = (current: number) => {
  currentPage.value = current;
  loadApiPermissions();
};

// 刷新权限
const handleRefresh = async () => {
  try {
    const response = await fetch(`/api/sys/api_permission/update_all`, {
      method: 'POST'
    });
    const data = await response.json();
    if (data.code === 200) {
      loadApiPermissions();
    }
  } catch (error) {
    console.error('刷新权限失败:', error);
  }
};

// 初始化
onMounted(() => {
  loadApiPermissions();
});
</script>

<style scoped>
.api-permission-list {
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
  flex-wrap: wrap;
  gap: var(--spacing-md);
  align-items: center;
}

.search-form {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  align-items: center;
  width: 100%;
}

.search-input, .search-select {
  min-width: 200px;
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
}

.search-input:focus-within, .search-select:focus-within {
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

.search-icon {
  color: var(--text-placeholder);
  transition: color var(--transition-fast);
}

.search-input:focus-within .search-icon {
  color: var(--primary);
}

.search-button, .reset-button {
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
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
  transition: all var(--transition);
}

.table-card:hover {
  box-shadow: var(--shadow);
}

.api-table {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.api-table .el-table__header-wrapper th {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-sm);
  padding: var(--spacing-md);
  border-bottom: 2px solid var(--primary);
}

.api-table .el-table__row {
  transition: all var(--transition-fast);
}

.api-table .el-table__row:hover {
  background-color: var(--bg-secondary);
}

.api-table .el-table__row:nth-child(even) {
  background-color: var(--bg-lighter);
}

.api-path {
  font-family: var(--mono);
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  word-break: break-all;
}

.method-tag {
  font-size: var(--font-size-xs);
  font-weight: 600;
  border-radius: var(--radius-sm);
  padding: 2px 8px;
  transition: all var(--transition-fast);
}

.table-actions {
  text-align: center;
}

.action-buttons {
  display: flex;
  gap: var(--spacing-sm);
  justify-content: center;
}

.edit-button, .delete-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
  border-radius: var(--radius);
  padding: var(--spacing-xs) var(--spacing-sm);
}

.edit-button:hover, .delete-button:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* 分页 */
.pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: var(--spacing-lg);
}

.api-pagination {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.api-pagination .el-pagination__item {
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.api-pagination .el-pagination__item:hover {
  color: var(--primary);
  border-color: var(--primary);
}

.api-pagination .el-pagination__item.is-active {
  background-color: var(--primary);
  border-color: var(--primary);
}

/* 对话框 */
.api-dialog {
  border-radius: var(--radius-md);
  overflow: hidden;
}

.api-dialog .el-dialog__header {
  background-color: var(--bg-tertiary);
  padding: var(--spacing-lg);
}

.api-dialog .el-dialog__title {
  color: var(--text-primary);
  font-weight: 600;
  font-size: var(--font-size-lg);
}

.api-dialog .el-dialog__body {
  padding: var(--spacing-lg);
}

.api-form {
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

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .api-permission-list {
    padding: var(--spacing-md);
  }
  
  .action-bar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .action-button {
    justify-content: center;
  }
  
  .search-card {
    padding: var(--spacing-md);
  }
  
  .search-form {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-input, .search-select {
    width: 100%;
    min-width: unset;
  }
  
  .table-card {
    padding: var(--spacing-md);
  }
  
  .action-buttons {
    flex-direction: column;
    align-items: center;
  }
  
  .edit-button, .delete-button {
    width: 100%;
    justify-content: center;
  }
  
  .api-dialog {
    width: 95% !important;
  }
  
  .api-path {
    font-size: var(--font-size-xs);
  }
  
  .method-tag {
    font-size: 10px;
    padding: 1px 6px;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .api-permission-list {
    padding: var(--spacing-md);
  }
  
  .search-input, .search-select {
    min-width: 180px;
  }
  
  .table-card {
    padding: var(--spacing-md);
  }
}
</style>