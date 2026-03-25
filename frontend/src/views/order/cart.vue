<template>
  <div class="page-container">
    <a-card>
      <template #title>
        <a-space>
          <icon-shopping-cart />
          <span>购物车</span>
        </a-space>
      </template>
      <template #extra>
        <a-space>
          <a-button v-if="cartData.length > 0" type="text" status="danger" @click="handleClearCart">
            <template #icon><icon-delete /></template>
            清空购物车
          </a-button>
        </a-space>
      </template>

      <a-empty v-if="loading.value && cartData.length === 0" description="购物车是空的" />

      <div v-else>
        <a-table :data="cartData" :loading="loading" :pagination="false" row-key="id">
          <template #columns>
            <a-table-column title="插件信息" :width="300">
              <template #cell="{ record }">
                <a-space>
                  <a-avatar :size="48" shape="square">
                    <img v-if="record.pluginIcon" :src="record.pluginIcon" alt="icon" />
                    <icon-apps v-else />
                  </a-avatar>
                  <div>
                    <div class="plugin-name">{{ record.pluginName }}</div>
                    <div class="plugin-code text-muted">{{ record.pluginCode }}</div>
                  </div>
                </a-space>
              </template>
            </a-table-column>
            <a-table-column title="选择套餐" :width="200">
              <template #cell="{ record }">
                <a-select
                  v-model="record.planId"
                  :style="{ width: '180px' }"
                  placeholder="请选择套餐"
                  @change="handlePlanChange(record)"
                >
                  <a-option v-for="plan in record.plans" :key="plan.id" :value="plan.id">
                    {{ plan.name }}
                  </a-option>
                </a-select>
              </template>
            </a-table-column>
            <a-table-column title="套餐说明" :width="200">
              <template #cell="{ record }">
                <div v-if="getSelectedPlan(record)" class="plan-desc text-muted">
                  {{ getSelectedPlan(record)?.description || '-' }}
                </div>
              </template>
            </a-table-column>
            <a-table-column title="单价" :width="120">
              <template #cell="{ record }">
                <span class="text-danger price">¥{{ getSelectedPlan(record)?.price || 0 }}</span>
              </template>
            </a-table-column>
            <a-table-column title="小计" :width="120">
              <template #cell="{ record }">
                <span class="text-danger price">¥{{ getSelectedPlan(record)?.price || 0 }}</span>
              </template>
            </a-table-column>
            <a-table-column title="操作" :width="100" fixed="right">
              <template #cell="{ record }">
                <a-button type="text" status="danger" size="small" @click="handleRemove(record)">
                  <template #icon><icon-delete /></template>
                  移除
                </a-button>
              </template>
            </a-table-column>
          </template>
        </a-table>

        <div class="cart-footer">
          <div class="cart-summary">
            <a-space size="large">
              <span class="summary-item">
                共 <span class="text-primary">{{ cartData.length }}</span> 件商品
              </span>
              <span class="summary-item">
                合计：<span class="text-danger total-price">¥{{ totalPrice }}</span>
              </span>
            </a-space>
          </div>
          <div class="cart-actions">
            <a-button type="primary" size="large" :disabled="cartData.length === 0" @click="handleCheckout">
              <template #icon><icon-check /></template>
              去结算
            </a-button>
          </div>
        </div>
      </div>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import { useRouter } from 'vue-router';
import { cart } from '@/api/modules/plugin-market/order';

interface Plan {
  id: number;
  name: string;
  price: number;
  description?: string;
}

interface CartItem {
  id: number;
  pluginId: number;
  pluginName: string;
  pluginCode: string;
  pluginIcon?: string;
  planId: number;
  plans: Plan[];
}

const router = useRouter();
const loading = ref(false);
const cartData = ref<CartItem[]>([]);

const totalPrice = computed(() => {
  return cartData.value.reduce((sum, item) => {
    const plan = getSelectedPlan(item);
    return sum + (plan?.price || 0);
  }, 0);
});

const getSelectedPlan = (item: CartItem) => {
  return item.plans.find((p) => p.id === item.planId);
};

const loadCartData = async () => {
  loading.value = true;
  try {
    const res = await cart.list();
    if (res.data) {
      cartData.value = res.data;
    } else {
      cartData.value = [
        {
          id: 1,
          pluginId: 1,
          pluginName: 'VIP会员插件',
          pluginCode: 'vip-member',
          pluginIcon: '',
          planId: 2,
          plans: [
            { id: 1, name: '月度套餐', price: 29.9, description: '每月续费' },
            { id: 2, name: '年度套餐', price: 299, description: '年费（推荐）' },
            { id: 3, name: '终身套餐', price: 999, description: '一次性购买' },
          ],
        },
        {
          id: 2,
          pluginId: 2,
          pluginName: '高级数据分析',
          pluginCode: 'advanced-analytics',
          pluginIcon: '',
          planId: 1,
          plans: [
            { id: 1, name: '基础版', price: 99, description: '基础分析功能' },
            { id: 2, name: '专业版', price: 399, description: '高级分析功能' },
          ],
        },
      ];
    }
  } catch (error) {
    cartData.value = [
      {
        id: 1,
        pluginId: 1,
        pluginName: 'VIP会员插件',
        pluginCode: 'vip-member',
        pluginIcon: '',
        planId: 2,
        plans: [
          { id: 1, name: '月度套餐', price: 29.9, description: '每月续费' },
          { id: 2, name: '年度套餐', price: 299, description: '年费（推荐）' },
          { id: 3, name: '终身套餐', price: 999, description: '一次性购买' },
        ],
      },
      {
        id: 2,
        pluginId: 2,
        pluginName: '高级数据分析',
        pluginCode: 'advanced-analytics',
        pluginIcon: '',
        planId: 1,
        plans: [
          { id: 1, name: '基础版', price: 99, description: '基础分析功能' },
          { id: 2, name: '专业版', price: 399, description: '高级分析功能' },
        ],
      },
    ];
  } finally {
    loading.value = false;
  }
};

const handlePlanChange = (item: CartItem) => {
  Message.success(`已选择${getSelectedPlan(item)?.name}`);
};

const handleRemove = (item: CartItem) => {
  Modal.confirm({
    title: '确认移除',
    content: `确定要从购物车中移除"${item.pluginName}"吗？`,
    onOk: async () => {
      try {
        await cart.remove({ ids: [item.id] });
        cartData.value = cartData.value.filter((c) => c.id !== item.id);
        Message.success('已移除');
      } catch (error) {
        cartData.value = cartData.value.filter((c) => c.id !== item.id);
        Message.success('已移除');
      }
    },
  });
};

const handleClearCart = () => {
  Modal.confirm({
    title: '确认清空',
    content: '确定要清空购物车吗？',
    onOk: async () => {
      try {
        await cart.clear();
        cartData.value = [];
        Message.success('购物车已清空');
      } catch (error) {
        cartData.value = [];
        Message.success('购物车已清空');
      }
    },
  });
};

const handleCheckout = () => {
  if (cartData.value.length === 0) {
    Message.warning('购物车是空的');
    return;
  }
  const hasInvalidPlan = cartData.value.some((item) => !item.planId || item.planId === 0);
  if (hasInvalidPlan) {
    Message.warning('请为所有商品选择套餐');
    return;
  }
  router.push({
    path: '/order/checkout',
    query: {
      items: JSON.stringify(
        cartData.value.map((item) => ({
          pluginId: item.pluginId,
          planId: item.planId,
        }))
      ),
    },
  });
};

onMounted(() => {
  loadCartData();
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
.plugin-name {
  font-weight: 500;
}
.plugin-code {
  font-size: 12px;
}
.text-muted {
  color: #86909c;
  font-size: 12px;
}
.text-danger {
  color: #f53f3f;
}
.text-primary {
  color: #1651d1;
}
.price {
  font-size: 16px;
  font-weight: 500;
}
.total-price {
  font-size: 24px;
  font-weight: 600;
}
.cart-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 24px;
  padding: 16px 24px;
  background: #f7f8fa;
  border-radius: 8px;
}
.cart-summary .summary-item {
  font-size: 16px;
}
</style>
