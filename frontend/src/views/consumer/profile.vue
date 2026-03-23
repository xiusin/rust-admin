<template>
  <div class="user-container">
    <a-card>
      <template #title>用户管理</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.keyword" placeholder="手机号/昵称" allow-clear style="width: 180px" />
          <a-select v-model="searchForm.status" placeholder="状态" allow-clear style="width: 100px">
            <a-option value="0">正常</a-option>
            <a-option value="1">禁用</a-option>
          </a-select>
          <a-select v-model="searchForm.level" placeholder="等级" allow-clear style="width: 100px">
            <a-option :value="1">普通会员</a-option>
            <a-option :value="2">铜牌会员</a-option>
            <a-option :value="3">银牌会员</a-option>
            <a-option :value="4">金牌会员</a-option>
            <a-option :value="5">钻石会员</a-option>
            <a-option :value="6">至尊会员</a-option>
          </a-select>
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            搜索
          </a-button>
          <a-button @click="handleReset">重置</a-button>
        </a-space>
      </template>

      <a-table :data="userList" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="ID" data-index="id" :width="80" />
          <a-table-column title="用户信息" :width="240">
            <template #cell="{ record }">
              <div class="user-info">
                <a-avatar :size="40" :image-url="record.avatar">
                  <icon-user v-if="!record.avatar" />
                </a-avatar>
                <div class="user-detail">
                  <div class="nickname">
                    {{ record.nickname || '未设置' }}
                    <a-tag v-if="record.level_name" :color="getLevelColor(record.level)" size="small">
                      {{ record.level_name }}
                    </a-tag>
                  </div>
                  <div class="phone">{{ record.phone }}</div>
                </div>
              </div>
            </template>
          </a-table-column>
          <a-table-column title="邮箱" :width="180" ellipsis>
            <template #cell="{ record }">{{ record.email || '-' }}</template>
          </a-table-column>
          <a-table-column title="第三方绑定" :width="160">
            <template #cell="{ record }">
              <a-space>
                <a-tooltip v-if="record.oauth_types?.includes('wechat')" content="微信已绑定">
                  <icon-wechat style="color: #07c160; font-size: 20px" />
                </a-tooltip>
                <a-tooltip v-if="record.oauth_types?.includes('github')" content="GitHub已绑定">
                  <icon-github style="color: #333; font-size: 20px" />
                </a-tooltip>
                <a-tooltip v-if="record.oauth_types?.includes('apple')" content="Apple已绑定">
                  <icon-apple style="color: #000; font-size: 20px" />
                </a-tooltip>
                <a-tooltip v-if="record.oauth_types?.includes('google')" content="Google已绑定">
                  <icon-google style="color: #4285f4; font-size: 20px" />
                </a-tooltip>
                <span v-if="!record.oauth_types?.length" class="text-muted">-</span>
              </a-space>
            </template>
          </a-table-column>
          <a-table-column title="标签" :width="180">
            <template #cell="{ record }">
              <template v-if="record.tags?.length">
                <a-tag v-for="tag in record.tags.slice(0, 2)" :key="tag.id" :color="tag.color" size="small" style="margin: 2px">
                  {{ tag.name }}
                </a-tag>
                <span v-if="record.tags.length > 2" class="text-muted">+{{ record.tags.length - 2 }}</span>
              </template>
              <span v-else class="text-muted">-</span>
            </template>
          </a-table-column>
          <a-table-column title="消费金额" :width="120">
            <template #cell="{ record }">
              <span class="text-danger">¥{{ record.total_consumption || '0.00' }}</span>
            </template>
          </a-table-column>
          <a-table-column title="订单数" :width="80">
            <template #cell="{ record }">{{ record.order_count || 0 }}笔</template>
          </a-table-column>
          <a-table-column title="风险评分" :width="100">
            <template #cell="{ record }">
              <a-progress :percent="record.risk_score || 0" :status="getRiskStatus(record.risk_score)" size="small" />
            </template>
          </a-table-column>
          <a-table-column title="状态" :width="80">
            <template #cell="{ record }">
              <a-badge :status="record.status === '0' ? 'success' : 'danger'" :text="record.status === '0' ? '正常' : '禁用'" />
            </template>
          </a-table-column>
          <a-table-column title="注册时间" data-index="created_at" :width="160" />
          <a-table-column title="操作" :width="200" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
                <a-dropdown>
                  <a-button type="text" size="small">更多<icon-down /></a-button>
                  <template #content>
                    <a-doption @click="handleTags(record)">管理标签</a-doption>
                    <a-doption @click="handleOAuth(record)">绑定信息</a-doption>
                    <a-doption @click="handleLevel(record)">调整等级</a-doption>
                    <a-divider style="margin: 4px 0" />
                    <a-doption v-if="record.status === '0'" @click="handleBan(record)" status="danger">禁用用户</a-doption>
                    <a-doption v-else @click="handleUnban(record)" status="success">解禁用户</a-doption>
                  </template>
                </a-dropdown>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="用户详情" :width="800" :footer="false">
      <a-tabs>
        <a-tab-pane key="basic" title="基本信息">
          <a-descriptions :column="2" bordered>
            <a-descriptions-item label="用户ID">{{ currentUser.id }}</a-descriptions-item>
            <a-descriptions-item label="手机号">{{ currentUser.phone }}</a-descriptions-item>
            <a-descriptions-item label="昵称">{{ currentUser.nickname || '-' }}</a-descriptions-item>
            <a-descriptions-item label="邮箱">{{ currentUser.email || '-' }}</a-descriptions-item>
            <a-descriptions-item label="会员等级">
              <a-tag :color="getLevelColor(currentUser.level)">{{ currentUser.level_name || '普通会员' }}</a-tag>
            </a-descriptions-item>
            <a-descriptions-item label="经验值">{{ currentUser.total_exp || 0 }}</a-descriptions-item>
            <a-descriptions-item label="账户状态">
              <a-badge :status="currentUser.status === '0' ? 'success' : 'danger'" :text="currentUser.status === '0' ? '正常' : '禁用'" />
            </a-descriptions-item>
            <a-descriptions-item label="风险评分">
              <a-progress :percent="currentUser.risk_score || 0" :status="getRiskStatus(currentUser.risk_score)" size="small" style="width: 100px" />
            </a-descriptions-item>
            <a-descriptions-item label="注册时间">{{ currentUser.created_at }}</a-descriptions-item>
            <a-descriptions-item label="最后登录">{{ currentUser.last_login_at || '-' }}</a-descriptions-item>
            <a-descriptions-item label="最后登录IP">{{ currentUser.last_login_ip || '-' }}</a-descriptions-item>
            <a-descriptions-item label="消费金额">
              <span class="text-danger">¥{{ currentUser.total_consumption || '0.00' }}</span>
            </a-descriptions-item>
          </a-descriptions>
        </a-tab-pane>
        <a-tab-pane key="oauth" title="第三方绑定">
          <a-list :bordered="false">
            <a-list-item v-for="oauth in currentUserOauths" :key="oauth.id">
              <a-list-item-meta :title="getOAuthTypeName(oauth.oauth_type)" :description="oauth.oauth_name || '-'">
                <template #avatar>
                  <a-avatar :style="{ backgroundColor: getOAuthColor(oauth.oauth_type) }">
                    <icon-wechat v-if="oauth.oauth_type === 'wechat'" />
                    <icon-github v-if="oauth.oauth_type === 'github'" />
                    <icon-apple v-if="oauth.oauth_type === 'apple'" />
                    <icon-google v-if="oauth.oauth_type === 'google'" />
                  </a-avatar>
                </template>
              </a-list-item-meta>
              <template #actions>
                <a-tag v-if="oauth.is_primary" color="blue">主绑定</a-tag>
                <a-button type="text" size="small" status="danger" @click="handleUnbindOAuth(oauth)">解绑</a-button>
              </template>
              <template #extra>
                <div class="oauth-extra">
                  <div>绑定时间: {{ oauth.bind_at || oauth.created_at }}</div>
                </div>
              </template>
            </a-list-item>
            <a-list-item v-if="!currentUserOauths.length">
              <a-empty description="暂无第三方绑定" />
            </a-list-item>
          </a-list>
        </a-tab-pane>
        <a-tab-pane key="tags" title="用户标签">
          <div class="tags-container">
            <a-tag v-for="tag in (currentUser.tags || [])" :key="tag.id" :color="tag.color" closable @close="removeTag(tag.id)" style="margin: 4px">
              {{ tag.name }}
            </a-tag>
            <a-button type="dashed" size="small" @click="handleTags(currentUser)">
              <template #icon><icon-plus /></template>
              添加标签
            </a-button>
          </div>
        </a-tab-pane>
        <a-tab-pane key="statistics" title="消费统计">
          <a-row :gutter="16">
            <a-col :span="8">
              <a-statistic title="订单总数" :value="currentUser.order_count || 0" suffix="笔" />
            </a-col>
            <a-col :span="8">
              <a-statistic title="消费总额" :value="currentUser.total_consumption || '0'" prefix="¥" />
            </a-col>
            <a-col :span="8">
              <a-statistic title="经验值" :value="currentUser.total_exp || 0" />
            </a-col>
          </a-row>
        </a-tab-pane>
      </a-tabs>
    </a-modal>

    <a-modal v-model:visible="tagsVisible" title="管理标签" @ok="submitTags" @cancel="tagsVisible = false" :width="500">
      <a-form layout="vertical">
        <a-form-item label="当前标签">
          <div>
            <a-tag v-for="tag in (currentUser.tags || [])" :key="tag.id" :color="tag.color" closable @close="removeTagInline(tag.id)" style="margin: 4px">
              {{ tag.name }}
            </a-tag>
            <span v-if="!(currentUser.tags || []).length" class="text-muted">暂无标签</span>
          </div>
        </a-form-item>
        <a-form-item label="添加标签">
          <a-select v-model="selectedTagIds" multiple placeholder="选择要添加的标签" style="width: 100%">
            <a-option v-for="tag in availableTags" :key="tag.id" :value="tag.id">
              <a-tag :color="tag.color" size="small">{{ tag.name }}</a-tag>
              <span style="margin-left: 8px; color: #999">{{ tag.category }}</span>
            </a-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="oauthVisible" title="第三方绑定管理" :width="600" :footer="false">
      <a-list :bordered="false">
        <a-list-item v-for="oauth in currentUserOauths" :key="oauth.id">
          <a-list-item-meta :title="getOAuthTypeName(oauth.oauth_type)" :description="`账号: ${oauth.oauth_name || '-'}`">
            <template #avatar>
              <a-avatar :size="40" :style="{ backgroundColor: getOAuthColor(oauth.oauth_type) }">
                <icon-wechat v-if="oauth.oauth_type === 'wechat'" />
                <icon-github v-if="oauth.oauth_type === 'github'" />
                <icon-apple v-if="oauth.oauth_type === 'apple'" />
                <icon-google v-if="oauth.oauth_type === 'google'" />
              </a-avatar>
            </template>
          </a-list-item-meta>
          <template #actions>
            <a-space direction="vertical">
              <a-tag v-if="oauth.is_primary" color="blue">主绑定</a-tag>
              <a-button v-else type="text" size="small" @click="setPrimaryOAuth(oauth)">设为主绑定</a-button>
              <a-button type="text" size="small" status="danger" @click="handleUnbindOAuth(oauth)">解绑</a-button>
            </a-space>
          </template>
          <template #extra>
            <div class="oauth-detail">
              <div><icon-clock-circle /> 绑定: {{ oauth.bind_at || oauth.created_at }}</div>
            </div>
          </template>
        </a-list-item>
        <a-list-item v-if="!currentUserOauths.length">
          <a-empty description="暂无第三方账号绑定" />
        </a-list-item>
      </a-list>
    </a-modal>

    <a-modal v-model:visible="levelVisible" title="调整用户等级" @ok="submitLevel" @cancel="levelVisible = false">
      <a-form :model="levelForm" layout="vertical">
        <a-form-item label="当前等级">
          <a-tag :color="getLevelColor(currentUser.level)">{{ currentUser.level_name || '普通会员' }}</a-tag>
          <span style="margin-left: 8px">经验值: {{ currentUser.total_exp || 0 }}</span>
        </a-form-item>
        <a-form-item label="调整方式">
          <a-radio-group v-model="levelForm.adjust_type">
            <a-radio value="exp">增减经验值</a-radio>
            <a-radio value="level">直接设置等级</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item v-if="levelForm.adjust_type === 'exp'" label="经验值变化">
          <a-input-number v-model="levelForm.exp_change" :min="-10000" :max="10000" style="width: 100%" />
          <div class="form-tip">正数为增加经验，负数为减少经验</div>
        </a-form-item>
        <a-form-item v-else label="目标等级">
          <a-select v-model="levelForm.target_level" style="width: 100%">
            <a-option :value="1">普通会员</a-option>
            <a-option :value="2">铜牌会员</a-option>
            <a-option :value="3">银牌会员</a-option>
            <a-option :value="4">金牌会员</a-option>
            <a-option :value="5">钻石会员</a-option>
            <a-option :value="6">至尊会员</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="调整原因">
          <a-textarea v-model="levelForm.remark" placeholder="请输入调整原因" :max-length="200" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="banVisible" title="禁用用户" @ok="submitBan" @cancel="banVisible = false">
      <a-form :model="banForm" layout="vertical">
        <a-form-item label="禁用类型">
          <a-radio-group v-model="banForm.ban_type">
            <a-radio value="temporary">临时禁用</a-radio>
            <a-radio value="permanent">永久禁用</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item v-if="banForm.ban_type === 'temporary'" label="到期时间">
          <a-date-picker v-model="banForm.end_at" show-time format="YYYY-MM-DD HH:mm:ss" style="width: 100%" />
        </a-form-item>
        <a-form-item label="禁用原因" required>
          <a-textarea v-model="banForm.reason" placeholder="请输入禁用原因" :max-length="500" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { consumerApi, ConsumerListItem } from '@/api/modules/consumer/index';
import { userExtensionApi, UserTagModel, UserOauthModel } from '@/api/modules/consumer/userExtension';

const loading = ref(false);
const userList = ref<ConsumerListItem[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const searchForm = reactive({
  keyword: '',
  status: '',
  level: undefined as number | undefined,
});

const detailVisible = ref(false);
const tagsVisible = ref(false);
const oauthVisible = ref(false);
const levelVisible = ref(false);
const banVisible = ref(false);

const currentUser = ref<ConsumerListItem>({} as ConsumerListItem);
const currentUserOauths = ref<UserOauthModel[]>([]);

const allTags = ref<UserTagModel[]>([]);
const selectedTagIds = ref<number[]>([]);

const availableTags = computed(() => {
  const currentTagIds = (currentUser.value.tags || []).map(t => t.id);
  return allTags.value.filter(t => !currentTagIds.includes(t.id));
});

const levelForm = reactive({
  adjust_type: 'exp',
  exp_change: 0,
  target_level: 1,
  remark: '',
});

const banForm = reactive({
  consumer_id: 0,
  ban_type: 'temporary',
  reason: '',
  end_at: '',
});

const getLevelColor = (level?: number) => {
  const colors: Record<number, string> = {
    1: '#999999',
    2: '#cd7f32',
    3: '#c0c0c0',
    4: '#ffd700',
    5: '#b9f2ff',
    6: '#ff6b6b',
  };
  return colors[level || 1] || '#999999';
};

const getRiskStatus = (score?: number) => {
  if (!score || score < 30) return 'success';
  if (score < 60) return 'warning';
  return 'danger';
};

const getOAuthTypeName = (type: string) => {
  const names: Record<string, string> = {
    wechat: '微信',
    github: 'GitHub',
    apple: 'Apple',
    google: 'Google',
  };
  return names[type] || type;
};

const getOAuthColor = (type: string) => {
  const colors: Record<string, string> = {
    wechat: '#07c160',
    github: '#333333',
    apple: '#000000',
    google: '#4285f4',
  };
  return colors[type] || '#999999';
};

const loadData = async () => {
  loading.value = true;
  try {
    const res = await consumerApi.list({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
      phone: searchForm.keyword || undefined,
      status: searchForm.status || undefined,
    });
    userList.value = res.data?.list || [];
    pagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const loadAllTags = async () => {
  try {
    const res = await userExtensionApi.listTags({ page_size: 100 });
    allTags.value = res.data?.list || [];
  } catch (error) {
    console.error(error);
  }
};

const loadUserOauths = async (consumerId: number) => {
  try {
    const res = await userExtensionApi.listOauth(consumerId);
    currentUserOauths.value = res.data || [];
  } catch (error) {
    console.error(error);
    currentUserOauths.value = [];
  }
};

const handleSearch = () => {
  pagination.value.current = 1;
  loadData();
};

const handleReset = () => {
  searchForm.keyword = '';
  searchForm.status = '';
  searchForm.level = undefined;
  pagination.value.current = 1;
  loadData();
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleDetail = async (record: ConsumerListItem) => {
  currentUser.value = record;
  await loadUserOauths(record.id);
  detailVisible.value = true;
};

const handleTags = async (record: ConsumerListItem) => {
  currentUser.value = record;
  selectedTagIds.value = [];
  await loadAllTags();
  tagsVisible.value = true;
};

const submitTags = async () => {
  try {
    if (selectedTagIds.value.length > 0) {
      await userExtensionApi.addUserTags({
        consumer_id: currentUser.value.id,
        tag_ids: selectedTagIds.value,
      });
    }
    Message.success('标签更新成功');
    tagsVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const removeTag = async (tagId: number) => {
  try {
    await userExtensionApi.removeUserTags({
      consumer_id: currentUser.value.id,
      tag_ids: [tagId],
    });
    Message.success('移除成功');
    currentUser.value.tags = (currentUser.value.tags || []).filter(t => t.id !== tagId);
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const removeTagInline = (tagId: number) => {
  currentUser.value.tags = (currentUser.value.tags || []).filter(t => t.id !== tagId);
};

const handleOAuth = async (record: ConsumerListItem) => {
  currentUser.value = record;
  await loadUserOauths(record.id);
  oauthVisible.value = true;
};

const handleUnbindOAuth = async (oauth: UserOauthModel) => {
  try {
    await userExtensionApi.unbindOauth({
      consumer_id: currentUser.value.id,
      oauth_type: oauth.oauth_type,
    });
    Message.success('解绑成功');
    await loadUserOauths(currentUser.value.id);
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const setPrimaryOAuth = async (oauth: UserOauthModel) => {
  try {
    await userExtensionApi.setPrimaryBind({
      consumer_id: currentUser.value.id,
      oauth_id: oauth.id,
    });
    Message.success('设置成功');
    await loadUserOauths(currentUser.value.id);
  } catch (error) {
    console.error(error);
  }
};

const handleLevel = (record: ConsumerListItem) => {
  currentUser.value = record;
  levelForm.adjust_type = 'exp';
  levelForm.exp_change = 0;
  levelForm.target_level = record.level || 1;
  levelForm.remark = '';
  levelVisible.value = true;
};

const submitLevel = async () => {
  try {
    if (levelForm.adjust_type === 'exp') {
      await userExtensionApi.addExp({
        consumer_id: currentUser.value.id,
        exp: levelForm.exp_change,
        source: 'admin_adjust',
        remark: levelForm.remark,
      });
    } else {
      Message.warning('暂不支持直接设置等级');
      return;
    }
    Message.success('等级调整成功');
    levelVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleBan = (record: ConsumerListItem) => {
  banForm.consumer_id = record.id;
  banForm.ban_type = 'temporary';
  banForm.reason = '';
  banForm.end_at = '';
  banVisible.value = true;
};

const submitBan = async () => {
  if (!banForm.reason) {
    Message.warning('请输入禁用原因');
    return;
  }
  try {
    await userExtensionApi.banUser({
      consumer_id: banForm.consumer_id,
      ban_type: banForm.ban_type,
      reason: banForm.reason,
      end_at: banForm.end_at || undefined,
    });
    Message.success('禁用成功');
    banVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleUnban = async (record: ConsumerListItem) => {
  try {
    await userExtensionApi.unbanUser({
      consumer_id: record.id,
      unban_reason: '管理员解禁',
    });
    Message.success('解禁成功');
    loadData();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.user-container {
  padding: 20px;
}
.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}
.user-detail {
  display: flex;
  flex-direction: column;
}
.nickname {
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
}
.phone {
  color: #86909c;
  font-size: 12px;
}
.text-muted {
  color: #86909c;
}
.text-danger {
  color: #f53f3f;
  font-weight: 500;
}
.tags-container {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 4px;
}
.oauth-extra,
.oauth-detail {
  font-size: 12px;
  color: #86909c;
}
.oauth-detail > div {
  margin-bottom: 4px;
}
.form-tip {
  font-size: 12px;
  color: #86909c;
  margin-top: 4px;
}
</style>
