<template>
  <div class="api-permission-list">
    <h1>API权限管理</h1>
    <div class="actions">
      <el-button type="primary" @click="handleAdd">新增权限</el-button>
      <el-button type="info" @click="handleRefresh">刷新权限</el-button>
    </div>
    
    <el-card>
      <el-form :inline="true" :model="searchForm" class="search-form">
        <el-form-item label="API路径">
          <el-input v-model="searchForm.api" placeholder="请输入API路径" />
        </el-form-item>
        <el-form-item label="方法">
          <el-select v-model="searchForm.method" placeholder="请选择方法">
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
          </el-select>
        </el-form-item>
        <el-form-item label="权限名称">
          <el-input v-model="searchForm.apiname" placeholder="请输入权限名称" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">查询</el-button>
          <el-button @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
      
      <el-table :data="apiPermissions" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="api" label="API路径" min-width="200" />
        <el-table-column prop="method" label="方法" width="100" />
        <el-table-column prop="apiname" label="权限名称" min-width="150" />
        <el-table-column prop="sort" label="排序" width="80" />
        <el-table-column prop="remark" label="备注" min-width="150" />
        <el-table-column prop="created_at" label="创建时间" width="180" />
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button type="danger" size="small" @click="handleDelete(row.id)">删除</el-button>
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
    </el-card>
    
    <!-- 新增/编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
    >
      <el-form :model="form" :rules="rules" ref="formRef">
        <el-form-item label="API路径" prop="api">
          <el-input v-model="form.api" placeholder="请输入API路径" />
        </el-form-item>
        <el-form-item label="方法" prop="method">
          <el-select v-model="form.method" placeholder="请选择方法">
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
          </el-select>
        </el-form-item>
        <el-form-item label="权限名称" prop="apiname">
          <el-input v-model="form.apiname" placeholder="请输入权限名称" />
        </el-form-item>
        <el-form-item label="排序" prop="sort">
          <el-input-number v-model="form.sort" :min="1" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入备注" />
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
import { ref, reactive, onMounted } from 'vue';
import type { FormInstance } from 'element-plus';

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
  padding: 20px;
}

.actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.search-form {
  margin-bottom: 20px;
}

.pagination {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>