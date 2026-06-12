<template>
  <div class="shortcut-box">
    <div class="box-title">
      <div>常用功能</div>
      <div class="customize">
        <span><icon-edit /></span>
        <span class="margin-left-text">自定义</span>
      </div>
    </div>
    <a-row>
      <a-col
        :xs="24"
        :sm="12"
        :lg="8"
        :xl="4"
        v-for="(item, index) in shortcut"
        :key="item.id"
        class="card-box"
        :class="'animated-fade-up-' + index"
      >
        <div class="shortcut-icon">
          <s-svg-icon :name="item.svg" :size="32" />
        </div>
        <div class="shortcut-card-label">{{ item.name }}</div>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
const shortcut = ref([
  {
    id: 1,
    name: "数据查询",
    svg: "data-queries"
  },
  {
    id: 2,
    name: "新增凭证",
    svg: "add-voucher"
  },
  {
    id: 3,
    name: "余额查询",
    svg: "balance-inquiry"
  },
  {
    id: 4,
    name: "财务报表",
    svg: "financial-statement"
  },
  {
    id: 5,
    name: "数据统计",
    svg: "data-analysis"
  },
  {
    id: 6,
    name: "更多",
    svg: "more"
  }
]);
</script>

<style lang="scss" scoped>
.shortcut-box {
  background: $color-bg-1;
  border-radius: $radius-box-3;
  padding: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  margin-bottom: 20px;
  transition: all 0.3s ease;
  &:hover {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  }
  .box-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: $font-size-title-1;
    font-weight: 600;
    color: $color-text-1;
    margin-bottom: 16px;
    .customize {
      display: flex;
      align-items: center;
      color: $color-primary;
      font-size: $font-size-body-1;
      cursor: pointer;
      transition: all 0.2s ease;
      &:hover {
        opacity: 0.8;
        transform: translateY(-1px);
      }
    }
  }
  .card-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px 16px;
    background: $color-bg-white;
    border-radius: $radius-box-2;
    cursor: pointer;
    transition: all 0.3s ease;
    border: 1px solid $color-border-2;
    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 6px 16px rgba(0, 0, 0, 0.1);
      border-color: $color-primary;
    }
    .shortcut-icon {
      width: 56px;
      height: 56px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: rgba(var(--primary-6), 0.1);
      border-radius: $radius-box-2;
      margin-bottom: 12px;
      transition: all 0.3s ease;
      &:hover {
        background: rgba(var(--primary-6), 0.2);
        transform: scale(1.05);
      }
    }
    .shortcut-card-label {
      font-size: $font-size-body-2;
      font-weight: 500;
      color: $color-text-1;
      text-align: center;
    }
  }
}
.margin-left-text {
  margin-left: $margin-text;
}

// 动画效果
@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animated-fade-up-0 { animation: fadeUp 0.3s ease 0.1s both; }
.animated-fade-up-1 { animation: fadeUp 0.3s ease 0.2s both; }
.animated-fade-up-2 { animation: fadeUp 0.3s ease 0.3s both; }
.animated-fade-up-3 { animation: fadeUp 0.3s ease 0.4s both; }
.animated-fade-up-4 { animation: fadeUp 0.3s ease 0.5s both; }
.animated-fade-up-5 { animation: fadeUp 0.3s ease 0.6s both; }
</style>
