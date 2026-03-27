<template>
  <div class="gen-config">
    <a-form layout="inline" :model="configData">
      <a-form-item label="输出目录">
        <a-input v-model="configData.outputDir" placeholder="/src" style="width: 200px" />
      </a-form-item>
      
      <a-form-item label="覆盖已存在">
        <a-switch v-model="configData.overwrite" />
      </a-form-item>
      
      <a-form-item label="生成测试">
        <a-switch v-model="configData.generateTests" />
      </a-form-item>
      
      <a-form-item label="生成文档">
        <a-switch v-model="configData.generateDocs" />
      </a-form-item>
      
      <a-form-item>
        <a-button type="primary" @click="handleGenerate">
          <template #icon><icon-thunderbolt /></template>
          生成代码
        </a-button>
      </a-form-item>
    </a-form>
    
    <a-modal
      v-model:visible="configModalVisible"
      title="高级配置"
      :width="600"
      @ok="handleConfigSave"
    >
      <a-form :model="configData" layout="vertical">
        <a-divider orientation="left">后端配置</a-divider>
        
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="模块名称">
              <a-input v-model="configData.moduleName" placeholder="sys" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="包路径">
              <a-input v-model="configData.packagePath" placeholder="com.example" />
            </a-form-item>
          </a-col>
        </a-row>
        
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="作者">
              <a-input v-model="configData.author" placeholder="作者名称" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="版本">
              <a-input v-model="configData.version" placeholder="1.0.0" />
            </a-form-item>
          </a-col>
        </a-row>
        
        <a-divider orientation="left">前端配置</a-divider>
        
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="API路径">
              <a-input v-model="configData.apiPath" placeholder="/api" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="视图路径">
              <a-input v-model="configData.viewPath" placeholder="/views" />
            </a-form-item>
          </a-col>
        </a-row>
        
        <a-divider orientation="left">生成选项</a-divider>
        
        <a-row :gutter="16">
          <a-col :span="8">
            <a-form-item label="生成Entity">
              <a-switch v-model="configData.genEntity" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="生成Service">
              <a-switch v-model="configData.genService" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="生成Controller">
              <a-switch v-model="configData.genController" />
            </a-form-item>
          </a-col>
        </a-row>
        
        <a-row :gutter="16">
          <a-col :span="8">
            <a-form-item label="生成Vue页面">
              <a-switch v-model="configData.genVue" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="生成API接口">
              <a-switch v-model="configData.genApi" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="生成路由">
              <a-switch v-model="configData.genRouter" />
            </a-form-item>
          </a-col>
        </a-row>
        
        <a-form-item label="模板选择">
          <a-select v-model="configData.template" placeholder="选择代码模板">
            <a-option value="default">默认模板</a-option>
            <a-option value="simple">简洁模板</a-option>
            <a-option value="full">完整模板</a-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue'

interface GenConfigData {
  outputDir: string
  overwrite: boolean
  generateTests: boolean
  generateDocs: boolean
  moduleName?: string
  packagePath?: string
  author?: string
  version?: string
  apiPath?: string
  viewPath?: string
  genEntity?: boolean
  genService?: boolean
  genController?: boolean
  genVue?: boolean
  genApi?: boolean
  genRouter?: boolean
  template?: string
}

interface Props {
  modelValue: GenConfigData
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({
    outputDir: '/src',
    overwrite: false,
    generateTests: false,
    generateDocs: false,
    moduleName: '',
    packagePath: '',
    author: '',
    version: '1.0.0',
    apiPath: '/api',
    viewPath: '/views',
    genEntity: true,
    genService: true,
    genController: true,
    genVue: true,
    genApi: true,
    genRouter: true,
    template: 'default'
  })
})

const emit = defineEmits<{
  'update:modelValue': [value: GenConfigData]
  'generate': [value: GenConfigData]
}>()

const configData = ref<GenConfigData>({ ...props.modelValue })
const configModalVisible = ref(false)

watch(
  () => props.modelValue,
  (val) => {
    configData.value = { ...val }
  },
  { deep: true }
)

watch(
  configData,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)

const handleGenerate = () => {
  emit('generate', configData.value)
  Message.success('代码生成成功')
}

const handleConfigSave = () => {
  configModalVisible.value = false
  Message.success('配置已保存')
}
</script>

<style lang="scss" scoped>
.gen-config {
  :deep(.arco-form-item) {
    margin-bottom: 0;
  }
}
</style>
