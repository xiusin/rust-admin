<template>
  <div class="permission-category-list">
    <div class="page-header">
      <h1 class="page-title">权限分类管理</h1>
      <div class="actions">
        <el-button type="primary" @click="handleAdd" class="action-button">
          <el-icon class="button-icon">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
            </svg>
          </el-icon>
          新增分类
        </el-button>
      </div>
    </div>
    
    <div class="search-card">
      <el-form :inline="true" :model="searchForm" class="search-form">
        <el-form-item label="分类名称">
          <el-input v-model="searchForm.name" placeholder="请输入分类名称" clearable />
        </el-form-item>
        <el-form-item label="分类编码">
          <el-input v-model="searchForm.code" placeholder="请输入分类编码" clearable />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch" class="search-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
            </el-icon>
            查询
          </el-button>
          <el-button @click="resetSearch" class="search-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
            </el-icon>
            重置
          </el-button>
        </el-form-item>
      </el-form>
    </div>
    
    <div class="table-card">
      <el-table :data="categories" style="width: 100%" class="data-table">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="分类名称" min-width="150" />
        <el-table-column prop="code" label="分类编码" min-width="150" />
        <el-table-column prop="sort" label="排序" width="80" />
        <el-table-column prop="remark" label="备注" min-width="150" />
        <el-table-column prop="created_at" label="创建时间" width="180" />
        <el-table-column label="操作" width="180" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="handleEdit(row)" class="table-action-button">
              <el-icon class="button-icon">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
                </svg>
              </el-icon>
              编辑
            </el-button>
            <el-button type="danger" size="small" @click="handleDelete(row.id)" class="table-action-button">
              <el-icon class="button-icon">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                </svg>
              </el-icon>
              删除
            </el-button>
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
        />
      </div>
    </div>
    
    <!-- 新增/编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
      class="custom-dialog"
    >
      <el-form :model="form" :rules="rules" ref="formRef" class="dialog-form">
        <el-form-item label="分类名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入分类名称" />
        </el-form-item>
        <el-form-item label="分类编码" prop="code">
          <el-input v-model="form.code" placeholder="请输入分类编码" />
        </el-form-item>
        <el-form-item label="排序" prop="sort">
          <el-input-number v-model="form.sort" :min="1" style="width: 100%" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.remark" type="textarea" :rows="3" placeholder="请输入备注" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="dialogVisible = false" class="dialog-button">取消</el-button>
          <el-button type="primary" @click="handleSubmit" class="dialog-button primary">确定</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import type { FormInstance } from 'element-plus';

// 搜索表单
const searchForm = reactive({
  name: '',
  code: ''
});

// 分页
const currentPage = ref(1);
const pageSize = ref(10);
const total = ref(0);

// 分类列表
const categories = ref<any[]>([]);

// 对话框
const dialogVisible = ref(false);
const dialogTitle = ref('新增分类');
const form = reactive({
  id: undefined,
  name: '',
  code: '',
  sort: 1,
  remark: ''
});
const formRef = ref<FormInstance>();

// 表单验证规则
const rules = reactive({
  name: [{ required: true, message: '请输入分类名称', trigger: 'blur' }],
  code: [{ required: true, message: '请输入分类编码', trigger: 'blur' }],
  sort: [{ required: true, message: '请输入排序', trigger: 'blur' }]
});

// 加载分类列表
const loadCategories = async () => {
  try {
    const response = await fetch(`/api/sys/permission_category/list?page=${currentPage.value}&limit=${pageSize.value}&name=${searchForm.name}&code=${searchForm.code}`);
    const data = await response.json();
    if (data.code === 200) {
      categories.value = data.data.list;
      total.value = data.data.total;
    }
  } catch (error) {
    console.error('加载分类失败:', error);
  }
};

// 处理新增
const handleAdd = () => {
  dialogTitle.value = '新增分类';
  Object.assign(form, {
    id: undefined,
    name: '',
    code: '',
    sort: 1,
    remark: ''
  });
  dialogVisible.value = true;
};

// 处理编辑
const handleEdit = (row: any) => {
  dialogTitle.value = '编辑分类';
  Object.assign(form, row);
  dialogVisible.value = true;
};

// 处理删除
const handleDelete = async (id: number) => {
  if (confirm('确定要删除这个分类吗？')) {
    try {
      const response = await fetch(`/api/sys/permission_category/delete/${id}`, {
        method: 'DELETE'
      });
      const data = await response.json();
      if (data.code === 200) {
        loadCategories();
      }
    } catch (error) {
      console.error('删除分类失败:', error);
    }
  }
};

// 处理提交
const handleSubmit = async () => {
  if (!formRef.value) return;
  
  try {
    await formRef.value.validate();
    
    const url = form.id ? `/api/sys/permission_category/edit` : `/api/sys/permission_category/add`;
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
      loadCategories();
    }
  } catch (error) {
    console.error('提交失败:', error);
  }
};

// 处理搜索
const handleSearch = () => {
  currentPage.value = 1;
  loadCategories();
};

// 重置搜索
const resetSearch = () => {
  Object.assign(searchForm, {
    name: '',
    code: ''
  });
  currentPage.value = 1;
  loadCategories();
};

// 处理分页
const handleSizeChange = (size: number) => {
  pageSize.value = size;
  loadCategories();
};

const handleCurrentChange = (current: number) => {
  currentPage.value = current;
  loadCategories();
};

// 初始化
onMounted(() => {
  loadCategories();
});
</script>

<style scoped>
.permission-category-list {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
}

.page-title {
  font-size: var(--font-size-xxl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.actions {
  display: flex;
  gap: var(--spacing-md);
}

.action-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
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

.search-form {
  margin-bottom: 0;
}

.search-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
}

.search-button:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow);
}

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

.data-table {
  border-radius: var(--radius);
  overflow: hidden;
}

.table-action-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  transition: all var(--transition-fast);
}

.table-action-button:hover {
  transform: translateY(-1px);
}

.pagination {
  margin-top: var(--spacing-lg);
  display: flex;
  justify-content: flex-end;
}

.custom-dialog {
  border-radius: var(--radius-md);
}

.dialog-form {
  padding-top: var(--spacing-md);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
}

.dialog-button {
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.dialog-button:hover {
  transform: translateY(-1px);
}

.dialog-button.primary {
  box-shadow: var(--shadow-sm);
}

@media screen and (max-width: 768px) {
  .permission-category-list {
    padding: var(--spacing-md);
  }
  
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-md);
  }
  
  .actions {
    width: 100%;
  }
  
  .action-button {
    flex: 1;
    justify-content: center;
  }
  
  .search-card,
  .table-card {
    padding: var(--spacing-md);
  }
  
  .search-form {
    display: flex;
    flex-direction: column;
  }
  
  .search-form .el-form-item {
    width: 100%;
    margin-right: 0;
    margin-bottom: var(--spacing-md);
  }
  
  .search-form .el-form-item:last-child {
    margin-bottom: 0;
  }
  
  .search-form .el-form-item .el-button {
    flex: 1;
  }
  
  .pagination {
    justify-content: center;
  }
}
</style>