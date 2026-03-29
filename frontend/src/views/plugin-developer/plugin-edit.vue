<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-row :gutter="16">
        <a-col :xs="24" :lg="16">
          <a-card title="基本信息">
            <a-form ref="formRef" :model="form" layout="vertical">
              <a-form-item field="name" label="插件名称" :rules="[{ required: true, message: '请输入插件名称' }]">
                <a-input v-model="form.name" placeholder="请输入插件名称" :max-length="50" show-word-limit />
              </a-form-item>
              <a-form-item field="code" label="插件编码" :rules="[{ required: true, message: '请输入插件编码' }]">
                <a-input v-model="form.code" placeholder="请输入插件编码，如 plugin_coupon" :disabled="isEdit" />
              </a-form-item>
              <a-form-item field="categoryId" label="插件分类" :rules="[{ required: true, message: '请选择插件分类' }]">
                <a-cascader
                  v-model="form.categoryId"
                  :options="categoryOptions"
                  placeholder="请选择插件分类"
                  allow-search
                  check-strictly
                />
              </a-form-item>
              <a-form-item field="summary" label="插件简介" :rules="[{ required: true, message: '请输入插件简介' }]">
                <a-input v-model="form.summary" placeholder="请输入插件简介，控制在100字以内" :max-length="100" show-word-limit />
              </a-form-item>
              <a-form-item field="description" label="详细描述">
                <a-textarea v-model="form.description" placeholder="请输入详细描述，支持Markdown格式" :rows="8" />
              </a-form-item>
            </a-form>
          </a-card>

          <a-card title="定价配置" style="margin-top: 16px">
            <a-form ref="pricingFormRef" :model="form" layout="vertical">
              <a-form-item field="priceType" label="价格类型">
                <a-radio-group v-model="form.priceType">
                  <a-radio :value="0">免费</a-radio>
                  <a-radio :value="1">一次性付费</a-radio>
                  <a-radio :value="2">订阅付费</a-radio>
                </a-radio-group>
              </a-form-item>
              <a-form-item v-if="form.priceType !== 0" field="price" label="价格">
                <a-input-number v-model="form.price" placeholder="请输入价格" :min="0" :precision="2" mode="button" />
                <span style="margin-left: 8px">元</span>
              </a-form-item>
              <a-form-item v-if="form.priceType === 2" field="originalPrice" label="原价（划线价格）">
                <a-input-number v-model="form.originalPrice" placeholder="请输入原价" :min="0" :precision="2" mode="button" />
                <span style="margin-left: 8px">元</span>
              </a-form-item>
            </a-form>
          </a-card>

          <a-card title="验证级别" style="margin-top: 16px">
            <a-form ref="verifyFormRef" :model="form" layout="vertical">
              <a-form-item field="verifyLevel" label="验证级别">
                <a-select v-model="form.verifyLevel" placeholder="请选择验证级别">
                  <a-option :value="0">无验证</a-option>
                  <a-option :value="1">基础验证</a-option>
                  <a-option :value="2">高级验证</a-option>
                </a-select>
              </a-form-item>
              <a-form-item label="验证说明">
                <a-alert v-if="form.verifyLevel === 0" type="info"> 无需验证，插件可被任意用户安装使用 </a-alert>
                <a-alert v-else-if="form.verifyLevel === 1" type="success">
                  基础验证：验证用户 license 授权信息，确保插件被正确授权使用
                </a-alert>
                <a-alert v-else type="warning"> 高级验证：启用设备绑定、到期时间校验、API调用次数限制等完整功能 </a-alert>
              </a-form-item>
            </a-form>
          </a-card>
        </a-col>

        <a-col :xs="24" :lg="8">
          <a-card title="插件封面">
            <a-form layout="vertical">
              <a-form-item label="封面图片">
                <div class="cover-upload" @click="onUploadCover">
                  <img v-if="form.coverImage" :src="form.coverImage" alt="封面" />
                  <div v-else class="upload-placeholder">
                    <icon-plus />
                    <span>上传封面</span>
                  </div>
                </div>
                <div class="upload-tip">建议尺寸 400x300 像素，支持 JPG、PNG 格式</div>
              </a-form-item>
            </a-form>
          </a-card>

          <a-card title="插件截图" style="margin-top: 16px">
            <div class="screenshots-list">
              <div v-for="(screenshot, index) in form.screenshots" :key="index" class="screenshot-item">
                <img :src="screenshot" alt="截图" />
                <div class="screenshot-actions">
                  <a-button type="text" size="small" @click="onMoveUp(index)" :disabled="index === 0">
                    <icon-up />
                  </a-button>
                  <a-button type="text" size="small" @click="onMoveDown(index)" :disabled="index === form.screenshots.length - 1">
                    <icon-down />
                  </a-button>
                  <a-button type="text" size="small" status="danger" @click="onRemoveScreenshot(index)">
                    <icon-delete />
                  </a-button>
                </div>
              </div>
              <div v-if="form.screenshots.length < 6" class="screenshot-add" @click="onAddScreenshot">
                <icon-plus />
                <span>添加截图</span>
              </div>
            </div>
            <div class="upload-tip">最多上传6张截图，建议尺寸 800x600 像素</div>
          </a-card>

          <a-card title="标签管理" style="margin-top: 16px">
            <a-form layout="vertical">
              <a-form-item label="插件标签">
                <div class="tags-input">
                  <a-tag v-for="(tag, index) in form.tags" :key="index" closable @close="onRemoveTag(index)">
                    {{ tag }}
                  </a-tag>
                  <a-input
                    v-if="tagInputVisible"
                    ref="tagInputRef"
                    v-model="newTag"
                    size="small"
                    class="tag-input"
                    @enter="onAddTag"
                    @blur="onAddTag"
                  />
                  <a-button v-else type="text" size="small" @click="onShowTagInput">
                    <template #icon><icon-plus /></template>
                    添加标签
                  </a-button>
                </div>
              </a-form-item>
              <a-form-item label="推荐标签">
                <a-space wrap>
                  <a-tag v-for="tag in recommendedTags" :key="tag" style="cursor: pointer" @click="onUseRecommendedTag(tag)">
                    {{ tag }}
                  </a-tag>
                </a-space>
              </a-form-item>
            </a-form>
          </a-card>

          <a-card title="其他设置" style="margin-top: 16px">
            <a-form layout="vertical">
              <a-form-item label="开发者信息">
                <a-input :value="developerName" disabled />
              </a-form-item>
            </a-form>
          </a-card>
        </a-col>
      </a-row>

      <a-row :gutter="16" style="margin-top: 16px">
        <a-col :span="24" style="text-align: center">
          <a-space size="large">
            <a-button @click="onCancel">取消</a-button>
            <a-button @click="onSaveDraft">保存草稿</a-button>
            <a-button type="primary" @click="onSubmitReview">提交审核</a-button>
          </a-space>
        </a-col>
      </a-row>
    </div>

    <a-modal
      v-model:visible="uploadModalVisible"
      title="上传图片"
      :width="500"
      @ok="onConfirmUpload"
      @cancel="uploadModalVisible = false"
    >
      <a-form layout="vertical">
        <a-form-item label="图片地址">
          <a-input v-model="uploadUrl" placeholder="请输入图片URL" />
        </a-form-item>
        <a-form-item label="或上传文件">
          <a-upload ref="uploadRef" :custom-request="customRequest" :show-upload-list="false" accept="image/*">
            <a-button type="outline">
              <template #icon><icon-upload /></template>
              选择文件
            </a-button>
          </a-upload>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from "vue";
import { useRouter, useRoute } from "vue-router";
import { developer, category } from "@/api/modules/plugin-market/market";
import { Message } from "@arco-design/web-vue";

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const formRef = ref();
const pricingFormRef = ref();
const verifyFormRef = ref();
const tagInputRef = ref();
const uploadRef = ref();
const tagInputVisible = ref(false);
const newTag = ref("");
const uploadModalVisible = ref(false);
const uploadUrl = ref("");
const uploadType = ref<"cover" | "screenshot">("cover");
const uploadIndex = ref(0);
const developerName = ref("奇络科技");

const categoryOptions = ref([
  {
    value: 1,
    label: "营销工具",
    children: [
      { value: 11, label: "优惠券" },
      { value: 12, label: "秒杀" },
      { value: 13, label: "拼团" }
    ]
  },
  {
    value: 2,
    label: "客服系统",
    children: [
      { value: 21, label: "在线客服" },
      { value: 22, label: "工单系统" }
    ]
  },
  {
    value: 3,
    label: "数据分析",
    children: [
      { value: 31, label: "数据统计" },
      { value: 32, label: "用户分析" }
    ]
  }
]);

const recommendedTags = ["官方", "热门", "稳定", "免费", "实用", "AI", "高评分"];

const isEdit = computed(() => route.params.id !== "new");
const pluginId = computed(() => (isEdit.value ? Number(route.params.id) : null));

const form = reactive({
  name: "",
  code: "",
  categoryId: [] as number[],
  summary: "",
  description: "",
  priceType: 0,
  price: 0,
  originalPrice: 0,
  verifyLevel: 0,
  coverImage: "",
  screenshots: [] as string[],
  tags: [] as string[]
});

const loadPluginDetail = async () => {
  if (!pluginId.value) return;
  loading.value = true;
  try {
    const res = await developer.list({ id: pluginId.value });
    const data = res.data?.list?.[0];
    if (data) {
      Object.assign(form, {
        name: data.name,
        code: data.code,
        categoryId: [data.categoryId],
        summary: data.summary,
        description: data.description || "",
        priceType: data.priceType,
        price: data.price,
        originalPrice: data.originalPrice || 0,
        verifyLevel: data.verifyLevel,
        coverImage: data.coverImage || "",
        screenshots: data.screenshots || [],
        tags: data.tags || []
      });
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const customRequest = (options: any) => {
  const { fileItem, onSuccess, onError } = options;
  setTimeout(() => {
    const url = `https://picsum.photos/400/300?random=${Date.now()}`;
    if (uploadType.value === "cover") {
      form.coverImage = url;
    } else {
      if (uploadType.value === "screenshot" && uploadIndex.value >= 0) {
        form.screenshots[uploadIndex.value] = url;
      } else {
        form.screenshots.push(url);
      }
    }
    uploadModalVisible.value = false;
    uploadUrl.value = "";
    onSuccess?.();
  }, 500);
};

const onUploadCover = () => {
  uploadType.value = "cover";
  uploadModalVisible.value = true;
};

const onAddScreenshot = () => {
  uploadType.value = "screenshot";
  uploadIndex.value = -1;
  uploadModalVisible.value = true;
};

const onConfirmUpload = () => {
  if (uploadUrl.value) {
    if (uploadType.value === "cover") {
      form.coverImage = uploadUrl.value;
    } else {
      form.screenshots.push(uploadUrl.value);
    }
  }
  uploadModalVisible.value = false;
  uploadUrl.value = "";
};

const onMoveUp = (index: number) => {
  if (index > 0) {
    const temp = form.screenshots[index];
    form.screenshots[index] = form.screenshots[index - 1];
    form.screenshots[index - 1] = temp;
  }
};

const onMoveDown = (index: number) => {
  if (index < form.screenshots.length - 1) {
    const temp = form.screenshots[index];
    form.screenshots[index] = form.screenshots[index + 1];
    form.screenshots[index + 1] = temp;
  }
};

const onRemoveScreenshot = (index: number) => {
  form.screenshots.splice(index, 1);
};

const onShowTagInput = () => {
  tagInputVisible.value = true;
  nextTick(() => {
    tagInputRef.value?.focus();
  });
};

const onAddTag = () => {
  const tag = newTag.value.trim();
  if (tag && !form.tags.includes(tag)) {
    form.tags.push(tag);
  }
  newTag.value = "";
  tagInputVisible.value = false;
};

const onRemoveTag = (index: number) => {
  form.tags.splice(index, 1);
};

const onUseRecommendedTag = (tag: string) => {
  if (!form.tags.includes(tag)) {
    form.tags.push(tag);
  }
};

const validateForm = async () => {
  try {
    await formRef.value?.validate();
    return true;
  } catch (error) {
    return false;
  }
};

const onSaveDraft = async () => {
  const valid = await validateForm();
  if (!valid) return;
  try {
    const data = {
      name: form.name,
      code: form.code,
      categoryId: form.categoryId[form.categoryId.length - 1],
      summary: form.summary,
      description: form.description,
      priceType: form.priceType,
      price: form.price,
      originalPrice: form.originalPrice,
      verifyLevel: form.verifyLevel,
      coverImage: form.coverImage,
      screenshots: form.screenshots,
      tags: form.tags,
      status: -1
    };
    if (isEdit.value) {
      await developer.edit({ id: pluginId.value, ...data });
    } else {
      await developer.add(data);
    }
    Message.success("保存草稿成功");
    router.push({ name: "plugin-list" });
  } catch (error) {
    console.error(error);
  }
};

const onSubmitReview = async () => {
  const valid = await validateForm();
  if (!valid) {
    Message.error("请完善插件信息");
    return;
  }
  if (!form.coverImage) {
    Message.error("请上传插件封面");
    return;
  }
  try {
    const data = {
      name: form.name,
      code: form.code,
      categoryId: form.categoryId[form.categoryId.length - 1],
      summary: form.summary,
      description: form.description,
      priceType: form.priceType,
      price: form.price,
      originalPrice: form.originalPrice,
      verifyLevel: form.verifyLevel,
      coverImage: form.coverImage,
      screenshots: form.screenshots,
      tags: form.tags,
      status: 0
    };
    if (isEdit.value) {
      await developer.edit({ id: pluginId.value, ...data });
    } else {
      await developer.add(data);
    }
    Message.success("提交审核成功");
    router.push({ name: "plugin-list" });
  } catch (error) {
    console.error(error);
  }
};

const onCancel = () => {
  router.push({ name: "plugin-list" });
};

onMounted(() => {
  if (isEdit.value) {
    loadPluginDetail();
  }
});
</script>

<style scoped lang="scss">
.cover-upload {
  width: 100%;
  height: 200px;
  border: 1px dashed var(--color-border);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    border-color: rgb(var(--primary-6));
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .upload-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: $color-text-3;
    gap: 8px;

    .arco-icon {
      font-size: 32px;
    }
  }
}

.screenshots-list {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;

  .screenshot-item {
    position: relative;
    width: 120px;
    height: 90px;
    border-radius: 4px;
    overflow: hidden;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }

    .screenshot-actions {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      display: flex;
      justify-content: center;
      gap: 4px;
      padding: 4px;
      background: rgba(0, 0, 0, 0.6);
    }
  }

  .screenshot-add {
    width: 120px;
    height: 90px;
    border: 1px dashed var(--color-border);
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: $color-text-3;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      border-color: rgb(var(--primary-6));
      color: rgb(var(--primary-6));
    }

    .arco-icon {
      font-size: 24px;
      margin-bottom: 4px;
    }
  }
}

.tags-input {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;

  .tag-input {
    width: 100px;
  }
}

.upload-tip {
  margin-top: 8px;
  font-size: 12px;
  color: $color-text-3;
}
</style>
