<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="渠道名称">
              <a-input v-model="formData.form.name" placeholder="请输入渠道名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="channel_type" label="渠道类型">
              <a-select v-model="formData.form.channel_type" placeholder="请选择渠道类型" allow-clear>
                <a-option value="wechat">微信支付</a-option>
                <a-option value="alipay">支付宝</a-option>
                <a-option value="unionpay">银联支付</a-option>
                <a-option value="balance">余额支付</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="is_active" label="状态">
              <a-select v-model="formData.form.is_active" placeholder="请选择状态" allow-clear>
                <a-option :value="1">启用</a-option>
                <a-option :value="0">停用</a-option>
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
            <a-button type="primary" @click="handleAdd">
              <template #icon><icon-plus /></template>
              新增渠道
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
        :pagination="false"
      >
        <template #channelType="{ record }">
          <a-space>
            <icon-wechat v-if="record.channel_type === 'wechat'" :style="{ color: '#07c160' }" />
            <icon-alipay-circle v-else-if="record.channel_type === 'alipay'" :style="{ color: '#1677ff' }" />
            <icon-credit-card v-else-if="record.channel_type === 'unionpay'" :style="{ color: '#e6a23c' }" />
            <icon-wallet v-else :style="{ color: '#909399' }" />
            <span>{{ getChannelTypeName(record.channel_type) }}</span>
          </a-space>
        </template>
        <template #scene="{ record }">
          <a-space wrap>
            <a-tag v-for="s in record.scenes" :key="s" size="small">{{ getSceneName(s) }}</a-tag>
          </a-space>
        </template>
        <template #status="{ record }">
          <a-switch v-model="record.is_active" @change="handleStatusChange(record)" />
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
            <a-button type="text" size="small" @click="handleConfig(record)">配置</a-button>
            <a-button type="text" size="small" status="danger" @click="handleDelete(record)">删除</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false" :width="600">
      <a-form :model="form" layout="vertical">
        <a-form-item label="渠道名称" required>
          <a-input v-model="form.name" placeholder="请输入渠道名称，如：微信支付-小程序" />
        </a-form-item>
        <a-form-item label="渠道类型" required>
          <a-select v-model="form.channel_type" placeholder="请选择渠道类型" @change="onChannelTypeChange">
            <a-option value="wechat">微信支付</a-option>
            <a-option value="alipay">支付宝</a-option>
            <a-option value="unionpay">银联支付</a-option>
            <a-option value="balance">余额支付</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="渠道标识" required>
          <a-input v-model="form.code" placeholder="如：wechat_miniapp、alipay_app" />
        </a-form-item>
        <a-form-item label="适用场景" required>
          <a-checkbox-group v-model="form.scenes">
            <a-checkbox value="h5">H5</a-checkbox>
            <a-checkbox value="app">APP</a-checkbox>
            <a-checkbox value="miniapp">小程序</a-checkbox>
            <a-checkbox value="pc">PC</a-checkbox>
          </a-checkbox-group>
        </a-form-item>
        <a-form-item label="排序">
          <a-input-number v-model="form.sort" :min="0" :max="999" placeholder="数字越小越靠前" />
        </a-form-item>
        <a-form-item label="是否启用">
          <a-switch v-model="form.is_active" />
        </a-form-item>
        <a-form-item label="备注">
          <a-textarea v-model="form.remark" placeholder="请输入备注" :auto-size="{ minRows: 2, maxRows: 4 }" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="configVisible" :title="configTitle" @ok="submitConfig" @cancel="configVisible = false" :width="700">
      <a-form :model="configForm" layout="vertical">
        <template v-if="currentChannel === 'wechat'">
          <a-form-item label="AppID" required>
            <a-input v-model="configForm.app_id" placeholder="微信支付AppID" />
          </a-form-item>
          <a-form-item label="MchID" required>
            <a-input v-model="configForm.mch_id" placeholder="商户号" />
          </a-form-item>
          <a-form-item label="API密钥" required>
            <a-input-password v-model="configForm.api_key" placeholder="APIv2密钥" />
          </a-form-item>
          <a-form-item label="APIv3密钥">
            <a-input-password v-model="configForm.api_v3_key" placeholder="APIv3密钥" />
          </a-form-item>
          <a-form-item label="证书序列号">
            <a-input v-model="configForm.cert_serial" placeholder="商户证书序列号" />
          </a-form-item>
          <a-form-item label="回调地址">
            <a-input v-model="configForm.notify_url" placeholder="支付结果回调地址" />
          </a-form-item>
        </template>
        <template v-else-if="currentChannel === 'alipay'">
          <a-form-item label="AppID" required>
            <a-input v-model="configForm.app_id" placeholder="支付宝AppID" />
          </a-form-item>
          <a-form-item label="应用私钥" required>
            <a-textarea v-model="configForm.private_key" placeholder="应用私钥内容" :auto-size="{ minRows: 4, maxRows: 8 }" />
          </a-form-item>
          <a-form-item label="支付宝公钥" required>
            <a-textarea v-model="configForm.alipay_public_key" placeholder="支付宝公钥内容" :auto-size="{ minRows: 4, maxRows: 8 }" />
          </a-form-item>
          <a-form-item label="回调地址">
            <a-input v-model="configForm.notify_url" placeholder="支付结果回调地址" />
          </a-form-item>
        </template>
        <template v-else-if="currentChannel === 'unionpay'">
          <a-form-item label="商户号" required>
            <a-input v-model="configForm.mer_id" placeholder="银联商户号" />
          </a-form-item>
          <a-form-item label="终端号">
            <a-input v-model="configForm.terminal_id" placeholder="终端号" />
          </a-form-item>
          <a-form-item label="回调地址">
            <a-input v-model="configForm.notify_url" placeholder="支付结果回调地址" />
          </a-form-item>
        </template>
        <template v-else>
          <a-empty description="余额支付无需额外配置" />
        </template>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import Sortable from 'sortablejs';
import { deepClone } from '@/utils';
import axios from '@/api';

interface ChannelRecord {
  id: number;
  name: string;
  code: string;
  channel_type: string;
  scenes: string[];
  config: any;
  sort: number;
  is_active: boolean;
  remark?: string;
  created_at: string;
}

const formData = reactive({
  form: {
    name: '',
    channel_type: null as string | null,
    is_active: null as number | null,
  },
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<ChannelRecord[]>([]);

interface Column {
  title: string;
  dataIndex: string;
  checked: boolean;
  slotName?: string;
  align?: string;
  width?: number;
}

const columnsShow = ref<Column[]>([]);
const columns = ref<Column[]>([
  { title: '渠道名称', dataIndex: 'name', checked: true, width: 180 },
  { title: '渠道类型', dataIndex: 'channel_type', checked: true, slotName: 'channelType', width: 150 },
  { title: '渠道标识', dataIndex: 'code', checked: true, width: 150 },
  { title: '适用场景', dataIndex: 'scene', checked: true, slotName: 'scene', width: 200 },
  { title: '排序', dataIndex: 'sort', checked: true, width: 80, align: 'center' },
  { title: '状态', dataIndex: 'status', checked: true, slotName: 'status', width: 100 },
  { title: '创建时间', dataIndex: 'created_at', checked: true, width: 160 },
  { title: '操作', slotName: 'optional', align: 'center', checked: true, width: 200 },
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

const getChannelTypeName = (type: string) => {
  const names: Record<string, string> = {
    wechat: '微信支付',
    alipay: '支付宝',
    unionpay: '银联支付',
    balance: '余额支付',
  };
  return names[type] || type;
};

const getSceneName = (scene: string) => {
  const names: Record<string, string> = {
    h5: 'H5',
    app: 'APP',
    miniapp: '小程序',
    pc: 'PC',
  };
  return names[scene] || scene;
};

const modalVisible = ref(false);
const modalTitle = ref('新增渠道');
const form = reactive({
  id: 0,
  name: '',
  code: '',
  channel_type: 'wechat',
  scenes: [] as string[],
  sort: 0,
  is_active: true,
  remark: '',
});

const configVisible = ref(false);
const configTitle = ref('渠道配置');
const currentChannel = ref('');
const currentRecordId = ref(0);
const configForm = reactive({
  app_id: '',
  mch_id: '',
  api_key: '',
  api_v3_key: '',
  cert_serial: '',
  notify_url: '',
  private_key: '',
  alipay_public_key: '',
  mer_id: '',
  terminal_id: '',
});

const loadData = async () => {
  loading.value = true;
  try {
    const params: any = {};
    if (formData.form.name) params.name = formData.form.name;
    if (formData.form.channel_type) params.channel_type = formData.form.channel_type;
    if (formData.form.is_active !== null) params.is_active = formData.form.is_active;
    
    const { data } = await axios.get('/pay_channel/list', { params });
    if (data.message === 'success') {
      tableData.value = (data.data || []).map((item: any) => ({
        ...item,
        is_active: item.is_active === 1 || item.is_active === true,
      }));
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleAdd = () => {
  modalTitle.value = '新增渠道';
  Object.assign(form, { id: 0, name: '', code: '', channel_type: 'wechat', scenes: [], sort: 0, is_active: true, remark: '' });
  modalVisible.value = true;
};

const handleEdit = (record: ChannelRecord) => {
  modalTitle.value = '编辑渠道';
  Object.assign(form, {
    ...record,
    is_active: record.is_active === true || record.is_active === 1,
  });
  modalVisible.value = true;
};

const handleConfig = (record: ChannelRecord) => {
  configTitle.value = `${record.name} - 渠道配置`;
  currentChannel.value = record.channel_type;
  currentRecordId.value = record.id;
  
  const config = record.config || {};
  Object.assign(configForm, {
    app_id: config.app_id || '',
    mch_id: config.mch_id || '',
    api_key: config.api_key || '',
    api_v3_key: config.api_v3_key || '',
    cert_serial: config.cert_serial || '',
    notify_url: config.notify_url || '',
    private_key: config.private_key || '',
    alipay_public_key: config.alipay_public_key || '',
    mer_id: config.mer_id || '',
    terminal_id: config.terminal_id || '',
  });
  configVisible.value = true;
};

const handleDelete = (record: ChannelRecord) => {
  Modal.confirm({
    title: '确认删除',
    content: `确定要删除支付渠道"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete('/pay_channel/del', { params: { id: record.id } });
        Message.success('删除成功');
        loadData();
      } catch (e) {
        console.error(e);
      }
    },
  });
};

const handleStatusChange = async (record: ChannelRecord) => {
  try {
    await axios.put('/pay_channel/toggle', null, { params: { id: record.id } });
    Message.success(`${record.name} ${record.is_active ? '已启用' : '已停用'}`);
  } catch (e) {
    console.error(e);
    record.is_active = !record.is_active;
  }
};

const onChannelTypeChange = () => {
  form.scenes = [];
};

const handleSubmit = async () => {
  if (!form.name || !form.code || !form.channel_type || form.scenes.length === 0) {
    Message.warning('请填写完整信息');
    return;
  }
  try {
    const submitData = {
      ...form,
      is_active: form.is_active ? 1 : 0,
    };
    
    if (form.id) {
      await axios.put('/pay_channel/edit', submitData);
      Message.success('编辑成功');
    } else {
      await axios.post('/pay_channel/add', submitData);
      Message.success('新增成功');
    }
    modalVisible.value = false;
    loadData();
  } catch (e) {
    console.error(e);
  }
};

const submitConfig = async () => {
  try {
    await axios.put('/pay_channel/config', {
      id: currentRecordId.value,
      config: configForm,
    });
    Message.success('配置保存成功');
    configVisible.value = false;
    loadData();
  } catch (e) {
    console.error(e);
  }
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
</style>
