<template>
  <div class="platform-create">
    <div class="page-header">
      <h1 class="page-title">新增平台</h1>
      <a-button @click="goBack">
        <template #icon><icon-left /></template>
        返回
      </a-button>
    </div>
    
    <div class="panel">
      <div class="panel-content">
        <a-form
          :model="form"
          :rules="rules"
          ref="formRef"
          layout="vertical"
        >
          <div class="form-grid">
            <a-form-item label="平台类型" field="platformType" class="form-item">
              <a-select v-model:value="form.platformType" placeholder="请选择平台类型" style="width: 100%">
                <a-option value="taobao">淘宝</a-option>
                <a-option value="pdd">拼多多</a-option>
                <a-option value="douyin">抖音</a-option>
                <a-option value="xianyu">闲鱼</a-option>
                <a-option value="amazon">亚马逊</a-option>
                <a-option value="wechat">微信</a-option>
              </a-select>
            </a-form-item>
            
            <a-form-item label="平台名称" field="name" class="form-item">
              <a-input v-model:value="form.name" placeholder="请输入平台名称" />
            </a-form-item>
            
            <a-form-item label="App Key" field="appKey" class="form-item">
              <a-input v-model:value="form.appKey" placeholder="请输入App Key" />
            </a-form-item>
            
            <a-form-item label="App Secret" field="appSecret" class="form-item">
              <a-input-password v-model:value="form.appSecret" placeholder="请输入App Secret" />
            </a-form-item>
            
            <a-form-item label="Access Token" field="accessToken" class="form-item">
              <a-input v-model:value="form.accessToken" placeholder="请输入Access Token（可选）" />
            </a-form-item>
            
            <a-form-item label="Refresh Token" field="refreshToken" class="form-item">
              <a-input v-model:value="form.refreshToken" placeholder="请输入Refresh Token（可选）" />
            </a-form-item>
            
            <a-form-item label="状态" field="status" class="form-item status-item">
              <a-switch v-model:checked="form.status" checked-children="启用" un-checked-children="禁用" />
            </a-form-item>
            
            <a-form-item label="备注" field="remark" class="form-item full-width">
              <a-textarea v-model:value="form.remark" placeholder="请输入备注信息（可选）" :rows="3" />
            </a-form-item>
          </div>
          
          <div class="form-actions">
            <a-button @click="goBack">取消</a-button>
            <a-button type="primary" @click="submitForm">
              保存
            </a-button>
            <a-button type="primary" @click="saveAndTest">
              保存并测试连接
            </a-button>
          </div>
        </a-form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const formRef = ref()

// 表单数据
const form = reactive({
  platformType: '',
  name: '',
  appKey: '',
  appSecret: '',
  accessToken: '',
  refreshToken: '',
  status: true,
  remark: ''
})

// 表单验证规则
const rules = {
  platformType: [
    { required: true, message: '请选择平台类型' }
  ],
  name: [
    { required: true, message: '请输入平台名称' },
    { min: 2, max: 50, message: '平台名称长度应在2-50个字符之间' }
  ],
  appKey: [
    { required: true, message: '请输入App Key' }
  ],
  appSecret: [
    { required: true, message: '请输入App Secret' }
  ]
}

// 提交表单
const submitForm = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    // 实现保存逻辑
    console.log('保存平台:', form)
    router.push('/ecommerce/platforms/list')
  } catch (error) {
    console.error('表单验证失败:', error)
  }
}

// 保存并测试连接
const saveAndTest = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    // 实现保存和测试连接逻辑
    console.log('保存并测试连接:', form)
    // 测试连接
    testConnection()
  } catch (error) {
    console.error('表单验证失败:', error)
  }
}

// 测试连接
const testConnection = () => {
  // 实现测试连接逻辑
  console.log('测试连接:', form.platformType)
  // 模拟测试成功
  setTimeout(() => {
    alert('连接测试成功！')
    router.push('/ecommerce/platforms/list')
  }, 1000)
}

// 返回
const goBack = () => {
  router.push('/ecommerce/platforms/list')
}
</script>

<style lang="scss" scoped>
.platform-create {
  padding: 24px;
  background: $color-bg-1;
  min-height: 100vh;

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;

    .page-title {
      font-size: 28px;
      font-weight: 600;
      color: $color-text-1;
      margin: 0;
    }
  }

  .panel {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
    overflow: hidden;

    .panel-content {
      padding: 32px;

      .form-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 24px;

        .form-item {
          &.full-width {
            grid-column: span 2;
          }

          &.status-item {
            display: flex;
            align-items: center;

            :deep(.arco-form-item-label) {
              margin-bottom: 0;
              margin-right: 24px;
            }
          }
        }
      }

      .form-actions {
        display: flex;
        gap: 12px;
        margin-top: 32px;
        justify-content: flex-end;
      }
    }
  }
}

@media (max-width: 768px) {
  .platform-create {
    padding: 16px;

    .page-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;
    }

    .panel {
      .panel-content {
        padding: 20px;

        .form-grid {
          grid-template-columns: 1fr;

          .form-item {
            &.full-width {
              grid-column: span 1;
            }
          }
        }

        .form-actions {
          flex-direction: column;

          :deep(.arco-button) {
            width: 100%;
          }
        }
      }
    }
  }
}
</style>
