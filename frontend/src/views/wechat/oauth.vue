<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="keyword" label="用户信息">
              <a-input v-model="formData.form.keyword" placeholder="用户昵称/OpenID" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="oauth_type" label="授权类型">
              <a-select v-model="formData.form.oauth_type" placeholder="请选择授权类型" allow-clear>
                <a-option value="wechat">微信公众号</a-option>
                <a-option value="mini">微信小程序</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="绑定状态">
              <a-select v-model="formData.form.status" placeholder="请选择状态" allow-clear>
                <a-option value="active">已绑定</a-option>
                <a-option value="inactive">已解绑</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" @click="loadData">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button @click="onReset">
                <template #icon><icon-refresh /></template>
                <template #default>重置</template>
              </a-button>
            </a-space>
          </a-col>
        </a-row>
      </a-form>
      <a-divider :margin="0" />
      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" status="danger" @click="handleBatchUnbind" :disabled="selectedKeys.length === 0">
              <template #icon><icon-close /></template>
              批量解绑
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="loadData"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-dropdown @select="onDensity">
              <a-tooltip content="密度">
                <div class="action-icon"><icon-line-height size="18" /></div>
              </a-tooltip>
              <template #content>
                <a-doption v-for="item in densityType" :value="item.value" :key="item.value" :disabled="item.value === density">{{ item.label }}</a-doption>
              </template>
            </a-dropdown>
            <a-tooltip content="列设置">
              <a-popover trigger="click" position="br" @popup-visible-change="popupVisibleChange">
                <div class="action-icon"><icon-settings size="18" /></div>
                <template #content>
                  <div id="tableSetting">
                    <div v-for="(item, index) in columns" :key="item.dataIndex" class="setting">
                      <div class="setting-box-icon"><icon-drag-arrow /></div>
                      <div>
                        <a-checkbox v-model="item.checked" @change="onCheckbox($event, item, index)"></a-checkbox>
                      </div>
                      <div class="title">{{ item.title }}</div>
                    </div>
                  </div>
                </template>
              </a-popover>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>
      <a-table
        row-key="id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columnsShow"
        :data="tableData"
        :row-selection="rowSelection"
        v-model:selectedKeys="selectedKeys"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #userInfo="{ record }">
          <div class="user-info">
            <a-avatar :size="36" :image-url="record.avatar" />
            <div class="user-detail">
              <div>{{ record.nickname }}</div>
              <div class="text-muted">ID: {{ record.consumer_id }}</div>
            </div>
          </div>
        </template>
        <template #oauthType="{ record }">
          <a-tag :color="record.oauth_type === 'wechat' ? 'green' : 'blue'">
            {{ record.oauth_type === 'wechat' ? '公众号' : '小程序' }}
          </a-tag>
        </template>
        <template #status="{ record }">
          <a-badge :status="record.status === 'active' ? 'success' : 'default'" :text="record.status === 'active' ? '已绑定' : '已解绑'" />
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button v-if="record.status === 'active'" type="text" size="small" status="danger" @click="handleUnbind(record)">解绑</a-button>
            <span v-else class="text-muted">-</span>
          </a-space>
        </template>
      </a-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import Sortable from 'sortablejs';
import { deepClone } from '@/utils';

interface OAuthRecord {
  id: number;
  consumer_id: number;
  nickname: string;
  avatar: string;
  oauth_type: string;
  openid: string;
  unionid: string;
  status: string;
  created_at: string;
}

const formData = reactive({
  form: {
    keyword: '',
    oauth_type: null as string | null,
    status: null as string | null,
  },
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<OAuthRecord[]>([]);
const selectedKeys = ref<number[]>([]);
const rowSelection = reactive({
  type: 'checkbox' as const,
  showCheckedAll: true,
  onlyCurrent: false,
});

const pagination = ref({ showPageSize: true, showTotal: true, current: 1, pageSize: 10, total: 0 });
const pageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  loadData();
};

interface Column {
  title: string;
  dataIndex: string;
  checked: boolean;
  slotName?: string;
  align?: string;
  width?: number;
  ellipsis?: boolean;
}

const columnsShow = ref<Column[]>([]);
const columns = ref<Column[]>([
  { title: '用户信息', dataIndex: 'userInfo', checked: true, slotName: 'userInfo', width: 200 },
  { title: '授权类型', dataIndex: 'oauth_type', checked: true, slotName: 'oauthType', width: 120 },
  { title: 'OpenID', dataIndex: 'openid', checked: true, width: 220, ellipsis: true },
  { title: 'UnionID', dataIndex: 'unionid', checked: true, width: 220, ellipsis: true },
  { title: '绑定状态', dataIndex: 'status', checked: true, slotName: 'status', width: 100 },
  { title: '授权时间', dataIndex: 'created_at', checked: true, width: 160 },
  { title: '操作', slotName: 'optional', align: 'center', checked: true, width: 120 },
]);

const deepColumns = () => {
  columnsShow.value = deepClone(columns.value);
};
deepColumns();

const densityType = ref([
  { value: 'mini', label: '迷你' },
  { value: 'small', label: '偏小' },
  { value: 'medium', label: '中等' },
  { value: 'large', label: '偏大' },
]);

const density = ref('small');
const onDensity = (e: string) => {
  density.value = e;
};

const onCheckbox = (checked: any, row: any, index: any) => {
  if (!checked) {
    columnsShow.value = columnsShow.value.filter((item: any) => item.dataIndex != row.dataIndex);
  } else {
    columnsShow.value.splice(index, 0, row);
  }
};

const popupVisibleChange = (visible: boolean) => {
  if (visible) {
    nextTick(() => {
      const el = document.getElementById('tableSetting') as HTMLElement;
      new Sortable(el, {
        onEnd(e: any) {
          const { oldIndex, newIndex } = e;
          exchangeArray(columns.value, oldIndex, newIndex);
          exchangeArray(columnsShow.value, oldIndex, newIndex);
        },
      });
    });
  }
};

const exchangeArray = (cols: Array<any>, oldIndex: number, newIndex: number) => {
  let temp = cols[newIndex];
  cols[newIndex] = cols[oldIndex];
  cols[oldIndex] = temp;
};

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    tableData.value = [
      { id: 1, consumer_id: 1001, nickname: '张三', avatar: 'https://thirdwx.qlogo.cn/mmopen/vi_32/xxx', oauth_type: 'wechat', openid: 'oXXXXxXXXXXXXXXXXXX', unionid: 'o6_bmasdasdsad6_2sgVt7hMZOPfL', status: 'active', created_at: '2026-03-22 10:00:00' },
      { id: 2, consumer_id: 1002, nickname: '李四', avatar: 'https://thirdwx.qlogo.cn/mmopen/vi_32/yyy', oauth_type: 'mini', openid: 'oYYYYyYYYYYYYYYYYYY', unionid: '', status: 'active', created_at: '2026-03-21 15:30:00' },
    ];
    pagination.value.total = 2;
  } finally {
    loading.value = false;
  }
};

const handleUnbind = (record: OAuthRecord) => {
  Modal.confirm({
    title: '确认解绑',
    content: `确定要解绑用户"${record.nickname}"的微信授权吗？`,
    onOk: () => {
      Message.success('解绑成功');
      loadData();
    },
  });
};

const handleBatchUnbind = () => {
  Modal.confirm({
    title: '确认批量解绑',
    content: `确定要解绑选中的 ${selectedKeys.value.length} 个授权吗？`,
    onOk: () => {
      Message.success('批量解绑成功');
      selectedKeys.value = [];
      loadData();
    },
  });
};

onMounted(() => {
  loadData();
});
</script>

<style lang="scss" scoped>
.search-btn {
  margin-bottom: 20px;
}
.setting {
  display: flex;
  align-items: center;
  width: 200px;
  .setting-box-icon {
    margin-right: 4px;
    cursor: move;
  }
  .title {
    margin-left: 8px;
  }
}
.action-icon {
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  &:hover {
    background-color: var(--color-fill-2);
  }
}
.user-info {
  display: flex;
  align-items: center;
  gap: 10px;
}
.user-detail {
  display: flex;
  flex-direction: column;
}
.text-muted {
  color: #86909c;
  font-size: 12px;
}
</style>
