<template>
  <div class="level-container">
    <a-tabs v-model:active-key="activeTab">
      <a-tab-pane key="level" title="用户等级">
        <a-card>
          <template #extra>
            <a-button type="primary" @click="handleCreateConfig">
              <template #icon><icon-plus /></template>
              新增等级
            </a-button>
          </template>
          <a-table :data="levelConfigs" :loading="levelLoading" :pagination="false">
            <template #columns>
              <a-table-column title="等级" data-index="level" :width="80" />
              <a-table-column title="等级名称" data-index="level_name" :width="150" />
              <a-table-column title="经验范围" :width="150">
                <template #cell="{ record }"> {{ record.min_exp }} - {{ record.max_exp || "∞" }} </template>
              </a-table-column>
              <a-table-column title="折扣率" data-index="discount_rate" :width="100">
                <template #cell="{ record }"> {{ (record.discount_rate * 100).toFixed(0) }}% </template>
              </a-table-column>
              <a-table-column title="图标" data-index="icon" :width="80">
                <template #cell="{ record }">
                  <icon-font v-if="record.icon" :type="record.icon" :style="{ color: record.color }" />
                  <span v-else>-</span>
                </template>
              </a-table-column>
              <a-table-column title="颜色" data-index="color" :width="80">
                <template #cell="{ record }">
                  <div :style="{ width: '20px', height: '20px', backgroundColor: record.color, borderRadius: '4px' }" />
                </template>
              </a-table-column>
              <a-table-column title="操作" :width="120">
                <template #cell="{ record }">
                  <a-button type="text" size="small" @click="handleEditConfig(record)">编辑</a-button>
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-tab-pane>

      <a-tab-pane key="tag" title="用户标签">
        <a-card>
          <template #extra>
            <a-button type="primary" @click="handleCreateTag">
              <template #icon><icon-plus /></template>
              新增标签
            </a-button>
          </template>
          <a-table :data="tagList" :loading="tagLoading" :pagination="tagPagination" @page-change="handleTagPageChange">
            <template #columns>
              <a-table-column title="标签名称" data-index="name" :width="150" />
              <a-table-column title="类型" data-index="tag_type" :width="120">
                <template #cell="{ record }">
                  <a-tag :color="record.tag_type === 'system' ? 'blue' : 'green'">
                    {{ record.tag_type === "system" ? "系统" : "自定义" }}
                  </a-tag>
                </template>
              </a-table-column>
              <a-table-column title="分类" data-index="category" :width="120" />
              <a-table-column title="颜色" data-index="color" :width="80">
                <template #cell="{ record }">
                  <div :style="{ width: '20px', height: '20px', backgroundColor: record.color, borderRadius: '4px' }" />
                </template>
              </a-table-column>
              <a-table-column title="描述" data-index="description" ellipsis />
              <a-table-column title="操作" :width="150">
                <template #cell="{ record }">
                  <a-space>
                    <a-button type="text" size="small" @click="handleEditTag(record)">编辑</a-button>
                    <a-popconfirm content="确定删除该标签吗？" @ok="handleDeleteTag(record)">
                      <a-button type="text" size="small" status="danger">删除</a-button>
                    </a-popconfirm>
                  </a-space>
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-tab-pane>

      <a-tab-pane key="ban" title="禁用管理">
        <a-card>
          <template #extra>
            <a-space>
              <a-input v-model="banSearch.consumer_id" placeholder="用户ID" allow-clear style="width: 150px" />
              <a-select v-model="banSearch.status" placeholder="状态" allow-clear style="width: 120px">
                <a-option value="active">禁用中</a-option>
                <a-option value="expired">已过期</a-option>
                <a-option value="unbanned">已解禁</a-option>
              </a-select>
              <a-button type="primary" @click="loadBanList">搜索</a-button>
            </a-space>
          </template>
          <a-table :data="banList" :loading="banLoading" :pagination="banPagination" @page-change="handleBanPageChange">
            <template #columns>
              <a-table-column title="用户ID" data-index="consumer_id" :width="100" />
              <a-table-column title="禁用类型" data-index="ban_type" :width="120">
                <template #cell="{ record }">
                  <a-tag :color="record.ban_type === 'permanent' ? 'red' : 'orange'">
                    {{ record.ban_type === "permanent" ? "永久" : "临时" }}
                  </a-tag>
                </template>
              </a-table-column>
              <a-table-column title="原因" data-index="reason" ellipsis />
              <a-table-column title="状态" data-index="status" :width="100">
                <template #cell="{ record }">
                  <a-tag :color="record.status === 'active' ? 'red' : 'green'">
                    {{ record.status === "active" ? "禁用中" : record.status === "expired" ? "已过期" : "已解禁" }}
                  </a-tag>
                </template>
              </a-table-column>
              <a-table-column title="禁用时间" data-index="start_at" :width="180" />
              <a-table-column title="解禁时间" data-index="end_at" :width="180" />
              <a-table-column title="操作" :width="120">
                <template #cell="{ record }">
                  <a-button
                    v-if="record.status === 'active'"
                    type="text"
                    size="small"
                    status="success"
                    @click="handleUnban(record)"
                    >解禁</a-button
                  >
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-tab-pane>

      <a-tab-pane key="statistics" title="消费统计">
        <a-card>
          <template #extra>
            <a-space>
              <a-input-number v-model="statisticsSearch.consumer_id" placeholder="用户ID" :min="1" style="width: 150px" />
              <a-select v-model="statisticsSearch.year" placeholder="年份" style="width: 120px">
                <a-option v-for="y in yearOptions" :key="y" :value="y">{{ y }}年</a-option>
              </a-select>
              <a-button type="primary" @click="loadStatistics">查询</a-button>
            </a-space>
          </template>
          <a-row :gutter="20">
            <a-col :span="6">
              <a-card>
                <a-statistic title="订单总数" :value="consumerStatistics.total_orders" suffix="单" />
              </a-card>
            </a-col>
            <a-col :span="6">
              <a-card>
                <a-statistic title="消费总额" :value="consumerStatistics.total_amount" prefix="¥" />
              </a-card>
            </a-col>
            <a-col :span="6">
              <a-card>
                <a-statistic title="退款总额" :value="consumerStatistics.total_refund" prefix="¥" />
              </a-card>
            </a-col>
            <a-col :span="6">
              <a-card>
                <a-statistic title="实际消费" :value="consumerStatistics.total_expense" prefix="¥" />
              </a-card>
            </a-col>
          </a-row>
        </a-card>
      </a-tab-pane>
    </a-tabs>

    <a-modal v-model:visible="levelConfigVisible" :title="levelConfigForm.id ? '编辑等级' : '新增等级'" @ok="submitLevelConfig">
      <a-form :model="levelConfigForm" layout="vertical">
        <a-form-item label="等级">
          <a-input-number v-model="levelConfigForm.level" :min="1" :max="100" />
        </a-form-item>
        <a-form-item label="等级名称">
          <a-input v-model="levelConfigForm.level_name" placeholder="如：黄金会员" />
        </a-form-item>
        <a-form-item label="最小经验值">
          <a-input-number v-model="levelConfigForm.min_exp" :min="0" />
        </a-form-item>
        <a-form-item label="最大经验值">
          <a-input-number v-model="levelConfigForm.max_exp" :min="0" placeholder="不填表示无上限" />
        </a-form-item>
        <a-form-item label="折扣率">
          <a-input-number v-model="levelConfigForm.discount_rate" :min="0" :max="1" :step="0.01" />
        </a-form-item>
        <a-form-item label="颜色">
          <input type="color" v-model="levelConfigForm.color" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="tagFormVisible" :title="tagForm.id ? '编辑标签' : '新增标签'" @ok="submitTag">
      <a-form :model="tagForm" layout="vertical">
        <a-form-item label="标签名称">
          <a-input v-model="tagForm.name" placeholder="请输入标签名称" />
        </a-form-item>
        <a-form-item label="类型">
          <a-select v-model="tagForm.tag_type">
            <a-option value="system">系统</a-option>
            <a-option value="custom">自定义</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="分类">
          <a-input v-model="tagForm.category" placeholder="如：VIP、活动等" />
        </a-form-item>
        <a-form-item label="颜色">
          <input type="color" v-model="tagForm.color" />
        </a-form-item>
        <a-form-item label="描述">
          <a-textarea v-model="tagForm.description" placeholder="标签描述" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="unbanVisible" title="解禁用户" @ok="submitUnban">
      <a-form :model="unbanForm" layout="vertical">
        <a-form-item label="解禁原因">
          <a-textarea v-model="unbanForm.unban_reason" placeholder="请输入解禁原因" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from "vue";
import { Message } from "@arco-design/web-vue";
import {
  userExtensionApi,
  LevelConfigModel,
  UserTagModel,
  UserBanModel,
  ConsumerStatisticsModel
} from "@/api/modules/consumer/userExtension";

const activeTab = ref("level");

const levelLoading = ref(false);
const levelConfigs = ref<LevelConfigModel[]>([]);
const levelConfigVisible = ref(false);
const levelConfigForm = reactive({
  id: 0,
  level: 1,
  level_name: "",
  min_exp: 0,
  max_exp: 0,
  discount_rate: 1,
  color: "#1890ff"
});

const tagLoading = ref(false);
const tagList = ref<UserTagModel[]>([]);
const tagPagination = ref({ current: 1, pageSize: 10, total: 0 });
const tagFormVisible = ref(false);
const tagForm = reactive({
  id: 0,
  name: "",
  tag_type: "custom",
  category: "",
  color: "#1890ff",
  description: ""
});

const banLoading = ref(false);
const banList = ref<UserBanModel[]>([]);
const banPagination = ref({ current: 1, pageSize: 10, total: 0 });
const banSearch = reactive({ consumer_id: "", status: "" });
const unbanVisible = ref(false);
const unbanForm = reactive({ consumer_id: 0, unban_reason: "" });

const statisticsSearch = reactive({ consumer_id: 0, year: new Date().getFullYear().toString() });
const consumerStatistics = ref<ConsumerStatisticsModel>({
  id: 0,
  consumer_id: 0,
  year: new Date().getFullYear(),
  total_orders: 0,
  total_amount: "0",
  total_refund: "0",
  total_expense: "0",
  avg_order_amount: "0"
});

const yearOptions = computed(() => {
  const currentYear = new Date().getFullYear();
  return Array.from({ length: 5 }, (_, i) => currentYear - i);
});

const loadLevelConfigs = async () => {
  levelLoading.value = true;
  try {
    const res = await userExtensionApi.getLevelConfigs();
    levelConfigs.value = res.data || [];
  } catch (error) {
    console.error(error);
  } finally {
    levelLoading.value = false;
  }
};

const handleCreateConfig = () => {
  Object.assign(levelConfigForm, {
    id: 0,
    level: 1,
    level_name: "",
    min_exp: 0,
    max_exp: 0,
    discount_rate: 1,
    color: "#1890ff"
  });
  levelConfigVisible.value = true;
};

const handleEditConfig = (record: LevelConfigModel) => {
  Object.assign(levelConfigForm, record);
  levelConfigVisible.value = true;
};

const submitLevelConfig = async () => {
  try {
    await userExtensionApi.createLevelConfig({
      level: levelConfigForm.level,
      level_name: levelConfigForm.level_name,
      min_exp: levelConfigForm.min_exp,
      max_exp: levelConfigForm.max_exp || undefined,
      discount_rate: levelConfigForm.discount_rate.toString(),
      color: levelConfigForm.color
    });
    Message.success("保存成功");
    levelConfigVisible.value = false;
    loadLevelConfigs();
  } catch (error) {
    console.error(error);
  }
};

const loadTagList = async () => {
  tagLoading.value = true;
  try {
    const res = await userExtensionApi.listTags({
      page_num: tagPagination.value.current,
      page_size: tagPagination.value.pageSize
    });
    tagList.value = res.data?.list || [];
    tagPagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    tagLoading.value = false;
  }
};

const handleTagPageChange = (page: number) => {
  tagPagination.value.current = page;
  loadTagList();
};

const handleCreateTag = () => {
  Object.assign(tagForm, {
    id: 0,
    name: "",
    tag_type: "custom",
    category: "",
    color: "#1890ff",
    description: ""
  });
  tagFormVisible.value = true;
};

const handleEditTag = (record: UserTagModel) => {
  Object.assign(tagForm, record);
  tagFormVisible.value = true;
};

const submitTag = async () => {
  try {
    if (tagForm.id) {
      await userExtensionApi.updateTag({
        id: tagForm.id,
        name: tagForm.name,
        category: tagForm.category,
        color: tagForm.color,
        description: tagForm.description
      });
    } else {
      await userExtensionApi.createTag({
        name: tagForm.name,
        tag_type: tagForm.tag_type,
        category: tagForm.category,
        color: tagForm.color,
        description: tagForm.description
      });
    }
    Message.success("保存成功");
    tagFormVisible.value = false;
    loadTagList();
  } catch (error) {
    console.error(error);
  }
};

const handleDeleteTag = async (record: UserTagModel) => {
  try {
    await userExtensionApi.deleteTag(record.id);
    Message.success("删除成功");
    loadTagList();
  } catch (error) {
    console.error(error);
  }
};

const loadBanList = async () => {
  banLoading.value = true;
  try {
    const res = await userExtensionApi.getBanHistory({
      page_num: banPagination.value.current,
      page_size: banPagination.value.pageSize,
      consumer_id: banSearch.consumer_id ? parseInt(banSearch.consumer_id) : undefined,
      status: banSearch.status || undefined
    });
    banList.value = res.data?.list || [];
    banPagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    banLoading.value = false;
  }
};

const handleBanPageChange = (page: number) => {
  banPagination.value.current = page;
  loadBanList();
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
    loadBanList();
  } catch (error) {
    console.error(error);
  }
};

const loadStatistics = async () => {
  if (!statisticsSearch.consumer_id) {
    Message.warning("请输入用户ID");
    return;
  }
  try {
    const res = await userExtensionApi.getYearStatistics({
      consumer_id: statisticsSearch.consumer_id,
      year: statisticsSearch.year
    });
    consumerStatistics.value = res.data || consumerStatistics.value;
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadLevelConfigs();
  loadTagList();
  loadBanList();
});
</script>

<style scoped>
.level-container {
  padding: 20px;
}
</style>
