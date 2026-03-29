<template>
  <div class="ban-container">
    <a-card>
      <template #title>禁用管理</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.consumer_id" placeholder="用户ID" allow-clear style="width: 120px" />
          <a-select v-model="searchForm.status" placeholder="状态" allow-clear style="width: 120px">
            <a-option value="active">禁用中</a-option>
            <a-option value="expired">已过期</a-option>
            <a-option value="unbanned">已解禁</a-option>
          </a-select>
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            搜索
          </a-button>
        </a-space>
      </template>

      <a-table :data="banList" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="ID" data-index="id" :width="80" />
          <a-table-column title="用户ID" data-index="consumer_id" :width="100" />
          <a-table-column title="禁用类型" data-index="ban_type" :width="100">
            <template #cell="{ record }">
              <a-tag :color="record.ban_type === 'permanent' ? 'red' : 'orange'">
                {{ record.ban_type === "permanent" ? "永久" : "临时" }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="禁用原因" data-index="reason" ellipsis />
          <a-table-column title="状态" data-index="status" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getStatusColor(record.status)">
                {{ getStatusName(record.status) }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="禁用时间" data-index="start_at" :width="180" />
          <a-table-column title="到期时间" data-index="end_at" :width="180">
            <template #cell="{ record }">
              {{ record.end_at || "永久" }}
            </template>
          </a-table-column>
          <a-table-column title="操作" :width="100" fixed="right">
            <template #cell="{ record }">
              <a-button v-if="record.status === 'active'" type="text" size="small" status="success" @click="handleUnban(record)">
                解禁
              </a-button>
              <span v-else class="text-muted">-</span>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="banVisible" title="禁用用户" @ok="submitBan" @cancel="banVisible = false">
      <a-form :model="banForm" layout="vertical">
        <a-form-item label="用户ID" required>
          <a-input-number v-model="banForm.consumer_id" placeholder="请输入用户ID" style="width: 100%" />
        </a-form-item>
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

    <a-modal v-model:visible="unbanVisible" title="解禁用户" @ok="submitUnban" @cancel="unbanVisible = false">
      <a-form :model="unbanForm" layout="vertical">
        <a-form-item label="解禁原因">
          <a-textarea v-model="unbanForm.unban_reason" placeholder="请输入解禁原因" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import { userExtensionApi, UserBanModel } from "@/api/modules/consumer/userExtension";

const loading = ref(false);
const banList = ref<UserBanModel[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const searchForm = reactive({
  consumer_id: "",
  status: ""
});

const banVisible = ref(false);
const banForm = reactive({
  consumer_id: 0,
  ban_type: "temporary",
  reason: "",
  end_at: ""
});

const unbanVisible = ref(false);
const unbanForm = reactive({
  consumer_id: 0,
  unban_reason: ""
});

const getStatusColor = (status: string) => {
  const map: Record<string, string> = {
    active: "red",
    expired: "gray",
    unbanned: "green"
  };
  return map[status] || "gray";
};

const getStatusName = (status: string) => {
  const map: Record<string, string> = {
    active: "禁用中",
    expired: "已过期",
    unbanned: "已解禁"
  };
  return map[status] || status;
};

const loadData = async () => {
  loading.value = true;
  try {
    const res = await userExtensionApi.getBanHistory({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
      consumer_id: searchForm.consumer_id ? parseInt(searchForm.consumer_id) : undefined,
      status: searchForm.status || undefined
    });
    banList.value = res.data?.list || [];
    pagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handleSearch = () => {
  pagination.value.current = 1;
  loadData();
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleBan = () => {
  Object.assign(banForm, {
    consumer_id: 0,
    ban_type: "temporary",
    reason: "",
    end_at: ""
  });
  banVisible.value = true;
};

const submitBan = async () => {
  if (!banForm.consumer_id) {
    Message.warning("请输入用户ID");
    return;
  }
  if (!banForm.reason) {
    Message.warning("请输入禁用原因");
    return;
  }
  try {
    await userExtensionApi.banUser({
      consumer_id: banForm.consumer_id,
      ban_type: banForm.ban_type,
      reason: banForm.reason,
      end_at: banForm.end_at || undefined
    });
    Message.success("禁用成功");
    banVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleUnban = (record: UserBanModel) => {
  unbanForm.consumer_id = record.consumer_id;
  unbanForm.unban_reason = "";
  unbanVisible.value = true;
};

const submitUnban = async () => {
  try {
    await userExtensionApi.unbanUser({
      consumer_id: unbanForm.consumer_id,
      unban_reason: unbanForm.unban_reason
    });
    Message.success("解禁成功");
    unbanVisible.value = false;
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
.ban-container {
  padding: 20px;
}
.text-muted {
  color: #999;
}
</style>
