<template>
  <div class="filter-config">
    <div class="config-header">
      <span>筛选配置</span>
      <a-button type="primary" size="small" @click="handleAddFilter">
        <template #icon><icon-plus /></template>
        添加筛选
      </a-button>
    </div>
    
    <div v-if="filters.length === 0" class="empty-filters">
      <a-empty description="暂无筛选条件" />
    </div>
    
    <div v-else class="filters-list">
      <div v-for="(filter, index) in filters" :key="filter.id" class="filter-item">
        <div class="filter-header">
          <a-input v-model="filter.label" placeholder="筛选名称" style="width: 150px" />
          <a-select v-model="filter.type" placeholder="筛选类型" style="width: 120px">
            <a-option value="select">下拉选择</a-option>
            <a-option value="checkbox">多选框</a-option>
            <a-option value="radio">单选框</a-option>
            <a-option value="date">日期</a-option>
            <a-option value="number">数字</a-option>
          </a-select>
          <a-button type="text" status="danger" size="small" @click="handleDelete(index)">
            <icon-delete />
          </a-button>
        </div>
        
        <div class="filter-content">
          <a-form layout="vertical">
            <a-row :gutter="12">
              <a-col :span="12">
                <a-form-item label="绑定字段">
                  <a-input v-model="filter.field" placeholder="字段名" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item label="默认值">
                  <a-input v-model="filter.defaultValue" placeholder="默认值" />
                </a-form-item>
              </a-col>
            </a-row>
            
            <a-form-item v-if="['select', 'checkbox', 'radio'].includes(filter.type)" label="选项配置">
              <div class="options-config">
                <div v-for="(option, optIndex) in filter.options" :key="optIndex" class="option-row">
                  <a-input v-model="option.label" placeholder="标签" style="width: 120px" />
                  <a-input v-model="option.value" placeholder="值" style="width: 120px" />
                  <a-button type="text" size="small" @click="handleDeleteOption(filter, optIndex)">
                    <icon-minus />
                  </a-button>
                </div>
                <a-button type="dashed" long size="small" @click="handleAddOption(filter)">
                  <icon-plus /> 添加选项
                </a-button>
              </div>
            </a-form-item>
            
            <a-row :gutter="12">
              <a-col :span="12">
                <a-form-item label="是否多选">
                  <a-switch v-model="filter.multiple" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item label="是否必选">
                  <a-switch v-model="filter.required" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-form>
        </div>
      </div>
    </div>
    
    <a-divider orientation="left">快捷筛选模板</a-divider>
    
    <div class="filter-templates">
      <a-tag
        v-for="template in filterTemplates"
        :key="template.type"
        color="arcoblue"
        style="cursor: pointer"
        @click="handleApplyTemplate(template)"
      >
        {{ template.label }}
      </a-tag>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue'

interface FilterOption {
  label: string
  value: string
}

interface Filter {
  id: string
  field: string
  label: string
  type: string
  defaultValue?: string
  options?: FilterOption[]
  multiple?: boolean
  required?: boolean
}

interface Props {
  modelValue: Filter[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
})

const emit = defineEmits<{
  'update:modelValue': [value: Filter[]]
}>()

const filters = ref<Filter[]>([...props.modelValue])

watch(
  () => props.modelValue,
  (val) => {
    filters.value = [...val]
  },
  { deep: true }
)

watch(
  filters,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)

const handleAddFilter = () => {
  filters.value.push({
    id: `filter_${Date.now()}`,
    field: '',
    label: '',
    type: 'select',
    options: []
  })
}

const handleDelete = (index: number) => {
  filters.value.splice(index, 1)
  Message.success('删除成功')
}

const handleAddOption = (filter: Filter) => {
  if (!filter.options) {
    filter.options = []
  }
  filter.options.push({ label: '', value: '' })
}

const handleDeleteOption = (filter: Filter, index: number) => {
  filter.options?.splice(index, 1)
}

const filterTemplates = [
  {
    type: 'status',
    label: '状态筛选',
    filter: {
      field: 'status',
      label: '状态',
      type: 'select',
      options: [
        { label: '全部', value: '' },
        { label: '正常', value: '1' },
        { label: '禁用', value: '0' }
      ]
    }
  },
  {
    type: 'date',
    label: '日期筛选',
    filter: {
      field: 'created_at',
      label: '创建时间',
      type: 'date'
    }
  },
  {
    type: 'keyword',
    label: '关键词筛选',
    filter: {
      field: 'keyword',
      label: '关键词',
      type: 'select',
      multiple: true
    }
  }
]

const handleApplyTemplate = (template: { filter: Partial<Filter> }) => {
  filters.value.push({
    id: `filter_${Date.now()}`,
    ...template.filter,
    options: template.filter.options ? [...template.filter.options] : []
  } as Filter)
}
</script>

<style lang="scss" scoped>
.filter-config {
  .config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    font-weight: 500;
  }

  .empty-filters {
    padding: 20px 0;
  }

  .filters-list {
    .filter-item {
      margin-bottom: 16px;
      padding: 16px;
      background: var(--color-fill-1);
      border-radius: 4px;

      .filter-header {
        display: flex;
        gap: 12px;
        align-items: center;
        margin-bottom: 12px;
      }

      .filter-content {
        .options-config {
          .option-row {
            display: flex;
            gap: 8px;
            align-items: center;
            margin-bottom: 8px;
          }
        }
      }
    }
  }

  .filter-templates {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
}
</style>
