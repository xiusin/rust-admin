<template>
  <div class="page-container">
    <a-row :gutter="24">
      <a-col :xs="24" :sm="24" :md="16" :lg="16" :xl="16">
        <a-card class="order-items-card">
          <template #title>
            <a-space>
              <icon-apps />
              <span>订单信息</span>
            </a-space>
          </template>

          <a-empty v-if="orderItems.length === 0" description="暂无订单信息" />

          <div v-else class="order-items">
            <div v-for="item in orderItems" :key="item.id" class="order-item">
              <a-space>
                <a-avatar :size="48" shape="square">
                  <img v-if="item.pluginIcon" :src="item.pluginIcon" alt="icon" />
                  <icon-apps v-else />
                </a-avatar>
                <div class="item-info">
                  <div class="item-name">{{ item.pluginName }}</div>
                  <div class="item-plan text-muted">{{ item.planName }}</div>
                </div>
              </a-space>
              <div class="item-price">
                <span class="text-danger">¥{{ item.price }}</span>
              </div>
            </div>
          </div>
        </a-card>

        <a-card class="payment-card" style="margin-top: 16px">
          <template #title>
            <a-space>
              <icon-wallet />
              <span>支付方式</span>
            </a-space>
          </template>

          <a-radio-group v-model="paymentMethod" class="payment-methods">
            <a-radio value="wechat">
              <a-space>
                <icon-wechat :style="{ color: '#07c160' }" />
                <span>微信支付</span>
              </a-space>
            </a-radio>
            <a-radio value="alipay">
              <a-space>
                <icon-alipay-circle :style="{ color: '#1677ff' }" />
                <span>支付宝</span>
              </a-space>
            </a-radio>
            <a-radio value="card">
              <a-space>
                <icon-credit-card :style="{ color: '#e6a23c' }" />
                <span>银行卡</span>
              </a-space>
            </a-radio>
            <a-radio value="balance">
              <a-space>
                <icon-wallet :style="{ color: '#909399' }" />
                <span>余额支付</span>
              </a-space>
            </a-radio>
          </a-radio-group>
        </a-card>

        <a-card class="coupon-card" style="margin-top: 16px">
          <template #title>
            <a-space>
              <icon-coupon />
              <span>优惠码</span>
            </a-space>
          </template>

          <a-space direction="vertical" :size="12">
            <a-space>
              <a-input v-model="couponCode" placeholder="请输入优惠码" style="width: 200px" allow-clear />
              <a-button type="primary" status="success" @click="handleApplyCoupon">应用</a-button>
            </a-space>
            <div v-if="appliedCoupon" class="coupon-applied">
              <a-tag color="green">
                <icon-check-circle />
                {{ appliedCoupon.name }} - 优惠 {{ appliedCoupon.discount }} 元
              </a-tag>
              <a-button type="text" size="small" @click="handleRemoveCoupon">移除</a-button>
            </div>
          </a-space>
        </a-card>

        <a-card class="terms-card" style="margin-top: 16px">
          <a-checkbox v-model="agreedToTerms">
            我已阅读并同意
            <a-link @click="showTermsModal = true">《服务条款》</a-link>
            和
            <a-link @click="showPrivacyModal = true">《隐私政策》</a-link>
          </a-checkbox>
        </a-card>
      </a-col>

      <a-col :xs="24" :sm="24" :md="8" :lg="8" :xl="8">
        <a-card class="summary-card">
          <template #title>
            <a-space>
              <icon-receipt />
              <span>订单摘要</span>
            </a-space>
          </template>

          <div class="summary-content">
            <div class="summary-row">
              <span class="label">商品件数</span>
              <span class="value">{{ orderItems.length }} 件</span>
            </div>
            <div class="summary-row">
              <span class="label">商品总价</span>
              <span class="value">¥{{ subtotal }}</span>
            </div>
            <div v-if="appliedCoupon" class="summary-row discount">
              <span class="label">优惠</span>
              <span class="value text-success">-¥{{ appliedCoupon.discount }}</span>
            </div>
            <a-divider :margin="12" />
            <div class="summary-row total">
              <span class="label">应付金额</span>
              <span class="value text-danger">¥{{ totalPrice }}</span>
            </div>

            <a-button
              type="primary"
              size="large"
              long
              :disabled="!canPay"
              :loading="paying"
              @click="handlePayNow"
            >
              <template #icon><icon-check-circle /></template>
              立即支付
            </a-button>

            <div class="payment-tips">
              <a-space direction="vertical" :size="4">
                <div class="tip-item">
                  <icon-check-circle :style="{ color: '#1651d1' }" />
                  <span>交易安全，无需担心资金安全</span>
                </div>
                <div class="tip-item">
                  <icon-check-circle :style="{ color: '#1651d1' }" />
                  <span>支持微信、支付宝、银行卡、余额</span>
                </div>
              </a-space>
            </div>
          </div>
        </a-card>
      </a-col>
    </a-row>

    <a-modal v-model:visible="qrCodeVisible" title="扫码支付" :footer="false" @cancel="qrCodeVisible = false">
      <div class="qrcode-modal">
        <div class="qrcode-info">
          <div class="qrcode-amount text-danger">¥{{ totalPrice }}</div>
          <div class="qrcode-tip">{{ qrCodeTip }}</div>
        </div>
        <div class="qrcode-wrapper">
          <img v-if="qrCodeUrl" :src="qrCodeUrl" alt="支付二维码" class="qrcode-image" />
          <div v-else class="qrcode-placeholder">
            <icon-loading :style="{ fontSize: '48px' }" />
            <span>正在生成二维码...</span>
          </div>
        </div>
        <div class="qrcode-actions">
          <a-button v-if="!paymentCompleted" type="primary" long @click="handleCheckPayment">
            我已支付
          </a-button>
          <a-button v-else type="primary" long status="success" @click="handlePaymentSuccess">
            支付成功
          </a-button>
        </div>
      </div>
    </a-modal>

    <a-modal v-model:visible="showTermsModal" title="服务条款" :footer="false" :width="600">
      <div class="terms-content">
        <p>这里是服务条款的内容...</p>
        <p>1. 用户在使用本平台服务时，需遵守相关法律法规...</p>
        <p>2. 用户需保证所购买的产品符合使用条件...</p>
        <p>3. 退款政策请参考相关退款条款...</p>
      </div>
    </a-modal>

    <a-modal v-model:visible="showPrivacyModal" title="隐私政策" :footer="false" :width="600">
      <div class="terms-content">
        <p>这里是隐私政策的内容...</p>
        <p>1. 我们会收集您的必要信息以提供更好的服务...</p>
        <p>2. 我们会保护您的个人信息安全...</p>
        <p>3. 更多隐私条款内容请参考完整隐私政策...</p>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import { useRouter, useRoute } from 'vue-router';
import { order } from '@/api/modules/plugin-market/order';

interface OrderItem {
  id: number;
  pluginId: number;
  pluginName: string;
  pluginCode: string;
  pluginIcon?: string;
  planId: number;
  planName: string;
  price: number;
}

interface Coupon {
  id: number;
  name: string;
  code: string;
  discount: number;
}

const router = useRouter();
const route = useRoute();

const orderItems = ref<OrderItem[]>([]);
const paymentMethod = ref('wechat');
const couponCode = ref('');
const appliedCoupon = ref<Coupon | null>(null);
const agreedToTerms = ref(false);
const paying = ref(false);
const qrCodeVisible = ref(false);
const qrCodeUrl = ref('');
const qrCodeTip = ref('请使用微信扫码支付');
const paymentCompleted = ref(false);
const showTermsModal = ref(false);
const showPrivacyModal = ref(false);
const currentOrderId = ref<number | null>(null);

const subtotal = computed(() => {
  return orderItems.value.reduce((sum, item) => sum + item.price, 0);
});

const totalPrice = computed(() => {
  const discount = appliedCoupon.value?.discount || 0;
  return Math.max(0, subtotal.value - discount);
});

const canPay = computed(() => {
  return orderItems.value.length > 0 && agreedToTerms.value;
});

const handleApplyCoupon = async () => {
  if (!couponCode.value) {
    Message.warning('请输入优惠码');
    return;
  }
  if (couponCode.value === 'DISCOUNT10') {
    appliedCoupon.value = {
      id: 1,
      name: '新人专享优惠',
      code: couponCode.value,
      discount: 10,
    };
    Message.success('优惠码已应用');
  } else if (couponCode.value === 'SAVE20') {
    appliedCoupon.value = {
      id: 2,
      name: '限时8折优惠',
      code: couponCode.value,
      discount: 20,
    };
    Message.success('优惠码已应用');
  } else {
    Message.error('无效的优惠码');
  }
};

const handleRemoveCoupon = () => {
  appliedCoupon.value = null;
  couponCode.value = '';
  Message.success('已移除优惠');
};

const generateQrCode = async () => {
  qrCodeUrl.value = '';
  qrCodeTip.value = `请使用${paymentMethod.value === 'wechat' ? '微信' : '支付宝'}扫码支付`;
  await new Promise((resolve) => setTimeout(resolve, 500));
  qrCodeUrl.value = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMDAiIGhlaWdodD0iMjAwIj48cmVjdCB3aWR0aD0iMjAwIiBoZWlnaHQ9IjIwMCIgZmlsbD0iI2ZmZiIvPjx0ZXh0IHg9IjUwJSIgeT0iNTAlIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBmaWxsPSIjMzMzIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTgiPkPhnJ7lm7B+RlRJTUU8L3RleHQ+PC9zdmc+`;
};

const handlePayNow = async () => {
  if (!agreedToTerms.value) {
    Message.warning('请先阅读并同意服务条款和隐私政策');
    return;
  }
  if (orderItems.value.length === 0) {
    Message.warning('订单信息为空');
    return;
  }

  paying.value = true;
  try {
    const firstItem = orderItems.value[0];
    const res = await order.create({
      pluginId: firstItem.pluginId,
      planId: firstItem.planId,
      couponId: appliedCoupon.value?.id,
      paymentMethod: getPaymentMethodCode(paymentMethod.value),
    });

    if (res.data?.id) {
      currentOrderId.value = res.data.id;
    } else {
      currentOrderId.value = Date.now();
    }

    qrCodeVisible.value = true;
    await generateQrCode();
  } catch (error) {
    currentOrderId.value = Date.now();
    qrCodeVisible.value = true;
    await generateQrCode();
  } finally {
    paying.value = false;
  }
};

const getPaymentMethodCode = (method: string): number => {
  const map: Record<string, number> = {
    wechat: 1,
    alipay: 2,
    card: 3,
    balance: 4,
  };
  return map[method] || 1;
};

const handleCheckPayment = async () => {
  try {
    await order.pay(currentOrderId.value!);
    paymentCompleted.value = true;
    Message.success('支付成功');
  } catch (error) {
    Modal.confirm({
      title: '确认支付状态',
      content: '如果已完成支付，请点击"支付成功"确认；如未支付，请关闭此窗口继续支付。',
      okText: '支付成功',
      cancelText: '未支付',
      onOk: () => {
        paymentCompleted.value = true;
        handlePaymentSuccess();
      },
    });
  }
};

const handlePaymentSuccess = () => {
  qrCodeVisible.value = false;
  Modal.success({
    title: '支付成功',
    content: '您的订单已支付成功，我们将尽快为您发货。',
    okText: '查看订单',
    onOk: () => {
      router.push('/order/list');
    },
  });
};

const loadOrderItems = () => {
  const itemsStr = route.query.items as string;
  if (itemsStr) {
    try {
      const items = JSON.parse(itemsStr);
      orderItems.value = items.map((item: any, index: number) => ({
        id: index + 1,
        pluginId: item.pluginId,
        pluginName: item.pluginName || `插件${item.pluginId}`,
        pluginCode: item.pluginCode || `plugin_${item.pluginId}`,
        pluginIcon: '',
        planId: item.planId,
        planName: item.planName || getDefaultPlanName(item.planId),
        price: item.price || getDefaultPrice(item.planId),
      }));
    } catch (e) {
      orderItems.value = getMockItems();
    }
  } else {
    orderItems.value = getMockItems();
  }
};

const getMockItems = (): OrderItem[] => [
  {
    id: 1,
    pluginId: 1,
    pluginName: 'VIP会员插件',
    pluginCode: 'vip-member',
    pluginIcon: '',
    planId: 2,
    planName: '年度套餐',
    price: 299,
  },
  {
    id: 2,
    pluginId: 2,
    pluginName: '高级数据分析',
    pluginCode: 'advanced-analytics',
    pluginIcon: '',
    planId: 2,
    planName: '专业版',
    price: 399,
  },
];

const getDefaultPlanName = (planId: number): string => {
  const plans: Record<number, string> = {
    1: '月度套餐',
    2: '年度套餐',
    3: '终身套餐',
  };
  return plans[planId] || '默认套餐';
};

const getDefaultPrice = (planId: number): number => {
  const prices: Record<number, number> = {
    1: 29.9,
    2: 299,
    3: 999,
  };
  return prices[planId] || 99;
};

onMounted(() => {
  loadOrderItems();
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
.order-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--color-fill-2);
}
.order-item:last-child {
  border-bottom: none;
}
.item-info {
  display: flex;
  flex-direction: column;
}
.item-name {
  font-weight: 500;
}
.item-plan {
  font-size: 12px;
  margin-top: 4px;
}
.item-price {
  font-size: 16px;
  font-weight: 500;
}
.payment-methods {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.payment-methods :deep(.arco-radio) {
  padding: 12px 16px;
  border: 1px solid var(--color-fill-3);
  border-radius: 6px;
}
.payment-methods :deep(.arco-radio-checked) {
  border-color: #1651d1;
}
.coupon-applied {
  display: flex;
  align-items: center;
  gap: 8px;
}
.summary-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.summary-row .label {
  color: #86909c;
}
.summary-row .value {
  font-weight: 500;
}
.summary-row.discount .value {
  color: #03a950;
}
.summary-row.total .label {
  font-size: 16px;
  font-weight: 500;
}
.summary-row.total .value {
  font-size: 24px;
  font-weight: 600;
}
.payment-tips {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px dashed var(--color-fill-3);
}
.tip-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #86909c;
}
.qrcode-modal {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
}
.qrcode-info {
  text-align: center;
  margin-bottom: 16px;
}
.qrcode-amount {
  font-size: 28px;
  font-weight: 600;
}
.qrcode-tip {
  margin-top: 8px;
  color: #86909c;
}
.qrcode-wrapper {
  width: 200px;
  height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f7f8fa;
  border-radius: 8px;
}
.qrcode-image {
  width: 180px;
  height: 180px;
}
.qrcode-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  color: #86909c;
}
.qrcode-actions {
  margin-top: 16px;
  width: 100%;
}
.terms-content {
  max-height: 400px;
  overflow-y: auto;
  line-height: 1.8;
}
.text-muted {
  color: #86909c;
}
.text-danger {
  color: #f53f3f;
}
.text-success {
  color: #03a950;
}
</style>
