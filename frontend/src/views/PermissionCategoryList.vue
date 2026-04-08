<template>
  <div class="permission-category-list">
    <h1>权限分类管理</h1>
    <div class="actions">
      <el-button type="primary" @click="handleAdd">新增分类</el-button>
    </div>
    
    <el-card>
      <el-form :inline="true" :model="searchForm" class="search-form">
        <el-form-item label="分类名称">
          <el-input v-model="searchForm.name" placeholder="请输入分类名称" />
        </el-form-item>
        <el-form-item label="分类编码">
          <el-input v-model="searchForm.code" placeholder="请输入分类编码" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">查询</el-button>
          <el-button @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
      
      <el-table :data="categories" style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="分类名称" min-width="150" />
        <el-table-column prop="code" label="分类编码" min-width="150" />
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
        <el-form-item label="分类名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入分类名称" />
        </el-form-item>
        <el-form-item label="分类编码" prop="code">
          <el-input v-model="form.code" placeholder="请输入分类编码" />
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