<template>
  <div class="job-list-container">
    <div class="page-header">
      <h1>定时任务管理</h1>
      <el-button type="primary" @click="handleAdd">新增任务</el-button>
    </div>
    
    <el-card>
      <el-form :model="searchForm" inline @submit.prevent>
        <el-form-item label="任务名称">
          <el-input v-model="searchForm.job_name" placeholder="请输入任务名称" />
        </el-form-item>
        <el-form-item label="任务分组">
          <el-input v-model="searchForm.job_group" placeholder="请输入任务分组" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">查询</el-button>
          <el-button @click="resetForm">重置</el-button>
        </el-form-item>
      </el-form>
      
      <el-table :data="jobList" style="width: 100%">
        <el-table-column prop="job_id" label="任务ID" width="80" />
        <el-table-column prop="job_name" label="任务名称" />
        <el-table-column prop="job_group" label="任务分组" />
        <el-table-column prop="cron_expression" label="Cron表达式" />
        <el-table-column prop="task_type" label="任务类型" />
        <el-table-column prop="job_params" label="任务参数" show-overflow-tooltip />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="scope">
            <el-tag :type="scope.row.status === 1 ? 'success' : 'danger'">
              {{ scope.row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="180" />
        <el-table-column prop="update_time" label="更新时间" width="180" />
        <el-table-column label="操作" width="200">
          <template #default="scope">
            <el-button size="small" @click="handleEdit(scope.row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(scope.row)">删除</el-button>
            <el-button size="small" type="primary" @click="handleExecute(scope.row)">执行</el-button>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="pagination.currentPage"
          v-model:page-size="pagination.pageSize"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          :total="pagination.total"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </el-card>
    
    <!-- 新增/编辑任务对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="500px"
    >
      <el-form :model="form" :rules="rules" ref="formRef" label-width="100px">
        <el-form-item label="任务名称" prop="job_name">
          <el-input v-model="form.job_name" placeholder="请输入任务名称" />
        </el-form-item>
        <el-form-item label="任务分组" prop="job_group">
          <el-input v-model="form.job_group" placeholder="请输入任务分组" />
        </el-form-item>
        <el-form-item label="Cron表达式" prop="cron_expression">
          <el-input v-model="form.cron_expression" placeholder="请输入Cron表达式" />
          <el-button type="text" @click="validateCron" style="margin-top: 10px">验证Cron表达式</el-button>
          <div v-if="cronValidationResult" class="cron-validation-result">
            <el-tag :type="cronValidationResult.validate ? 'success' : 'danger'">
              {{ cronValidationResult.validate ? '验证通过' : '验证失败' }}
            </el-tag>
            <div v-if="cronValidationResult.validate && cronValidationResult.next_ten" class="cron-next-executions">
              <p>下次执行时间：</p>
              <ul>
                <li v-for="(time, index) in cronValidationResult.next_ten" :key="index">
                  {{ time }}
                </li>
              </ul>
            </div>
          </div>
        </el-form-item>
        <el-form-item label="任务类型" prop="task_type">
          <el-select v-model="form.task_type" placeholder="请选择任务类型">
            <el-option label="HTTP请求" value="geturl" />
            <el-option label="函数调用" value="invokefunction" />
          </el-select>
        </el-form-item>
        <el-form-item label="任务参数" prop="job_params">
          <el-input
            v-model="form.job_params"
            type="textarea"
            :rows="4"
            placeholder="请输入任务参数"
          />
          <div class="param-hint">
            <p v-if="form.task_type === 'geturl'">HTTP请求：输入完整的URL地址</p>
            <p v-else-if="form.task_type === 'invokefunction'">函数调用：{"callfun": "函数名", "parmets": "参数"}</p>
          </div>
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-switch v-model="form.status" active-value="1" inactive-value="0" />
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
import { ElMessage, ElMessageBox } from 'element-plus';
import { request } from '../api/request';

// 搜索表单
const searchForm = reactive({
  job_name: '',
  job_group: ''
});

// 任务列表
const jobList = ref<any[]>([]);

// 分页信息
const pagination = reactive({
  currentPage: 1,
  pageSize: 10,
  total: 0
});

// 对话框状态
const dialogVisible = ref(false);
const dialogTitle = ref('新增任务');

// 表单数据
const form = reactive({
  job_id: 0,
  job_name: '',
  job_group: '',
  cron_expression: '',
  task_type: 'geturl',
  job_params: '',
  status: 1
});

// 表单验证规则
const rules = reactive({
  job_name: [{ required: true, message: '请输入任务名称', trigger: 'blur' }],
  job_group: [{ required: true, message: '请输入任务分组', trigger: 'blur' }],
  cron_expression: [{ required: true, message: '请输入Cron表达式', trigger: 'blur' }],
  task_type: [{ required: true, message: '请选择任务类型', trigger: 'change' }],
  job_params: [{ required: true, message: '请输入任务参数', trigger: 'blur' }]
});

// 表单引用
const formRef = ref<any>(null);

// Cron验证结果
const cronValidationResult = ref<any>(null);

// 获取任务列表
const getJobList = async () => {
  try {
    const response = await request.post('/api/sys/job/list', {
      page: pagination.currentPage,
      page_size: pagination.pageSize,
      ...searchForm
    });
    if (response.code === 200) {
      jobList.value = response.data.list;
      pagination.total = response.data.total;
    } else {
      ElMessage.error('获取任务列表失败');
    }
  } catch (error) {
    ElMessage.error('网络错误');
  }
};

// 搜索
const handleSearch = () => {
  pagination.currentPage = 1;
  getJobList();
};

// 重置
const resetForm = () => {
  searchForm.job_name = '';
  searchForm.job_group = '';
  pagination.currentPage = 1;
  getJobList();
};

// 分页大小变化
const handleSizeChange = (size: number) => {
  pagination.pageSize = size;
  getJobList();
};

// 当前页码变化
const handleCurrentChange = (current: number) => {
  pagination.currentPage = current;
  getJobList();
};

// 新增任务
const handleAdd = () => {
  dialogTitle.value = '新增任务';
  form.job_id = 0;
  form.job_name = '';
  form.job_group = '';
  form.cron_expression = '';
  form.task_type = 'geturl';
  form.job_params = '';
  form.status = 1;
  cronValidationResult.value = null;
  dialogVisible.value = true;
};

// 编辑任务
const handleEdit = (row: any) => {
  dialogTitle.value = '编辑任务';
  form.job_id = row.job_id;
  form.job_name = row.job_name;
  form.job_group = row.job_group;
  form.cron_expression = row.cron_expression;
  form.task_type = row.task_type;
  form.job_params = row.job_params || '';
  form.status = row.status;
  cronValidationResult.value = null;
  dialogVisible.value = true;
};

// 删除任务
const handleDelete = (row: any) => {
  ElMessageBox.confirm('确定要删除这个任务吗？', '警告', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      const response = await request.post('/api/sys/job/delete', {
        job_ids: [row.job_id]
      });
      if (response.code === 200) {
        ElMessage.success('删除成功');
        getJobList();
      } else {
        ElMessage.error('删除失败');
      }
    } catch (error) {
      ElMessage.error('网络错误');
    }
  }).catch(() => {
    // 取消删除
  });
};

// 执行任务
const handleExecute = (row: any) => {
  ElMessageBox.confirm('确定要立即执行这个任务吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'info'
  }).then(async () => {
    try {
      const response = await request.post('/api/sys/job/hand_execute_job', {
        job_id: row.job_id
      });
      if (response.code === 200) {
        ElMessage.success('执行成功');
      } else {
        ElMessage.error('执行失败');
      }
    } catch (error) {
      ElMessage.error('网络错误');
    }
  }).catch(() => {
    // 取消执行
  });
};

// 验证Cron表达式
const validateCron = async () => {
  if (!form.cron_expression) {
    ElMessage.warning('请输入Cron表达式');
    return;
  }
  try {
    const response = await request.post('/api/sys/job/validate_cron', {
      cron_expression: form.cron_expression
    });
    if (response.code === 200) {
      cronValidationResult.value = response.data;
    }
  } catch (error) {
    ElMessage.error('网络错误');
  }
};

// 提交表单
const handleSubmit = async () => {
  if (!formRef.value) return;
  await formRef.value.validate(async (valid: boolean) => {
    if (valid) {
      try {
        let response;
        if (form.job_id === 0) {
          // 新增
          response = await request.post('/api/sys/job/add', {
            job_name: form.job_name,
            job_group: form.job_group,
            cron_expression: form.cron_expression,
            task_type: form.task_type,
            job_params: form.job_params,
            status: form.status
          });
        } else {
          // 编辑
          response = await request.post('/api/sys/job/edit', {
            job_id: form.job_id,
            job_name: form.job_name,
            job_group: form.job_group,
            cron_expression: form.cron_expression,
            task_type: form.task_type,
            job_params: form.job_params,
            status: form.status
          });
        }
        if (response.code === 200) {
          ElMessage.success(form.job_id === 0 ? '新增成功' : '编辑成功');
          dialogVisible.value = false;
          getJobList();
        } else {
          ElMessage.error(form.job_id === 0 ? '新增失败' : '编辑失败');
        }
      } catch (error) {
        ElMessage.error('网络错误');
      }
    }
  });
};

// 初始化
onMounted(() => {
  getJobList();
});
</script>

<style scoped>
.job-list-container {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}

.cron-validation-result {
  margin-top: 10px;
}

.cron-next-executions {
  margin-top: 10px;
  font-size: 12px;
  color: #606266;
}

.cron-next-executions ul {
  margin-top: 5px;
  padding-left: 20px;
}

.param-hint {
  margin-top: 10px;
  font-size: 12px;
  color: #909399;
}
</style>