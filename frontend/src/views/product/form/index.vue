<template>
  <div class="product-form-page">
    <a-steps :current="currentStep" class="form-steps">
      <a-step title="基础信息" />
      <a-step title="商品规格" />
      <a-step title="价格库存" />
      <a-step title="其他设置" />
    </a-steps>

    <a-form
      ref="formRef"
      :model="formData"
      :rules="formRules"
      layout="vertical"
      :label-col="{ style: { width: '120px' } }"
    >
      <a-card class="form-card" v-show="currentStep === 0">
        <template #title>
          <div class="card-title">
            <icon-info-circle />
            基础信息
          </div>
        </template>
        <a-row :gutter="24">
          <a-col :span="12">
            <a-form-item field="categoryId" label="商品分类">
              <a-tree-select
                v-model="formData.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择商品分类"
                allow-clear
                style="width: 100%"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="brandId" label="商品品牌">
              <a-select v-model="formData.brandId" placeholder="请选择商品品牌" allow-clear style="width: 100%">
                <a-option v-for="brand in brandList" :key="brand.id" :value="brand.id">
                  {{ brand.name }}
                </a-option>
              </a-select>
            </a-form-item>
          </a-col>
        </a-row>
        <a-form-item field="name" label="商品名称">
          <a-input v-model="formData.name" placeholder="请输入商品名称" show-word-limit :max-length="100" />
        </a-form-item>
        <a-form-item field="subtitle" label="商品副标题">
          <a-input v-model="formData.subtitle" placeholder="请输入商品副标题" show-word-limit :max-length="200" />
        </a-form-item>
        <a-form-item field="coverImage" label="商品封面">
          <a-upload
            :file-list="coverImageList"
            :limit="1"
            :show-upload-list="true"
            list-type="picture-card"
            @change="handleCoverImageChange"
          >
            <div>
              <icon-plus />
              <div style="margin-top: 8px; color: #86909c">上传封面</div>
            </div>
          </a-upload>
        </a-form-item>
        <a-form-item field="images" label="商品图片">
          <a-upload
            :file-list="imageList"
            :limit="10"
            :show-upload-list="true"
            list-type="picture-card"
            @change="handleImageChange"
          >
            <div>
              <icon-plus />
              <div style="margin-top: 8px; color: #86909c">上传图片</div>
            </div>
          </a-upload>
        </a-form-item>
        <a-form-item field="keywords" label="商品关键词">
          <a-input v-model="formData.keywords" placeholder="请输入商品关键词，多个用逗号分隔" />
        </a-form-item>
        <a-form-item field="description" label="商品描述">
          <a-textarea v-model="formData.description" placeholder="请输入商品描述" :rows="4" show-word-limit :max-length="500" />
        </a-form-item>
      </a-card>

      <a-card class="form-card" v-show="currentStep === 1">
        <template #title>
          <div class="card-title">
            <icon-attribute />
            商品规格
          </div>
        </template>
        <a-form-item field="isMultiSpec" label="规格类型">
          <a-radio-group v-model="formData.isMultiSpec" direction="vertical">
            <a-radio :value="0">
              <div class="radio-option">
                <icon-apps />
                <div>
                  <div class="radio-title">单规格</div>
                  <div class="radio-desc">商品只有一个规格，如：红色、S码</div>
                </div>
              </div>
            </a-radio>
            <a-radio :value="1">
              <div class="radio-option">
                <icon-th-large />
                <div>
                  <div class="radio-title">多规格</div>
                  <div class="radio-desc">商品有多个规格组合，如：红色-S码、蓝色-M码</div>
                </div>
              </div>
            </a-radio>
          </a-radio-group>
        </a-form-item>
        <template v-if="formData.isMultiSpec === 1">
          <a-divider orientation="left">规格管理</a-divider>
          <div class="spec-manager">
            <div class="spec-list">
              <div class="spec-item" v-for="(spec, specIndex) in formData.specs" :key="specIndex">
                <div class="spec-header">
                  <a-input v-model="spec.name" placeholder="请输入规格名称，如：颜色" style="width: 200px" />
                  <a-button type="text" status="danger" @click="removeSpec(specIndex)">
                    <icon-delete />
                  </a-button>
                </div>
                <div class="spec-values">
                  <a-tag
                    v-for="(value, valueIndex) in spec.values"
                    :key="valueIndex"
                    closable
                    @close="removeSpecValue(specIndex, valueIndex)"
                    class="spec-value-tag"
                  >
                    {{ value.value }}
                  </a-tag>
                  <a-input
                    v-model="newSpecValue"
                    placeholder="按回车添加"
                    style="width: 150px"
                    @press-enter="addSpecValue(specIndex)"
                    class="spec-value-input"
                  />
                </div>
              </div>
            </div>
            <a-button type="dashed" long @click="addSpec">
              <template #icon><icon-plus /></template>
              添加规格
            </a-button>
          </div>
          <a-divider orientation="left">SKU 预览</a-divider>
          <a-table :data="skuPreviewData" :pagination="false" class="sku-preview-table">
            <template #columns>
              <a-table-column
                v-for="spec in formData.specs"
                :key="spec.name"
                :title="spec.name"
                :data-index="spec.name"
              />
              <a-table-column title="售价" data-index="salePrice">
                <template #cell="{ record }">
                  <a-input-number v-model="record.salePrice" :min="0" :precision="2" style="width: 100%" />
                </template>
              </a-table-column>
              <a-table-column title="原价" data-index="linePrice">
                <template #cell="{ record }">
                  <a-input-number v-model="record.linePrice" :min="0" :precision="2" style="width: 100%" />
                </template>
              </a-table-column>
              <a-table-column title="库存" data-index="stock">
                <template #cell="{ record }">
                  <a-input-number v-model="record.stock" :min="0" style="width: 100%" />
                </template>
              </a-table-column>
            </template>
          </a-table>
        </template>
      </a-card>

      <a-card class="form-card" v-show="currentStep === 2">
        <template #title>
          <div class="card-title">
            <icon-money />
            价格库存
          </div>
        </template>
        <template v-if="formData.isMultiSpec === 0">
          <a-row :gutter="24">
            <a-col :span="8">
              <a-form-item field="salePrice" label="售价">
                <a-input-number
                  v-model="formData.salePrice"
                  :min="0"
                  :precision="2"
                  placeholder="请输入售价"
                  style="width: 100%"
                >
                  <template #prefix>¥</template>
                </a-input-number>
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="linePrice" label="原价">
                <a-input-number
                  v-model="formData.linePrice"
                  :min="0"
                  :precision="2"
                  placeholder="请输入原价"
                  style="width: 100%"
                >
                  <template #prefix>¥</template>
                </a-input-number>
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="costPrice" label="成本价">
                <a-input-number
                  v-model="formData.costPrice"
                  :min="0"
                  :precision="2"
                  placeholder="请输入成本价"
                  style="width: 100%"
                >
                  <template #prefix>¥</template>
                </a-input-number>
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24">
            <a-col :span="8">
              <a-form-item field="stock" label="库存">
                <a-input-number v-model="formData.stock" :min="0" placeholder="请输入库存" style="width: 100%" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="virtualSales" label="虚拟销量">
                <a-input-number v-model="formData.virtualSales" :min="0" placeholder="请输入虚拟销量" style="width: 100%" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="skuCode" label="SKU编码">
                <a-input v-model="formData.skuCode" placeholder="请输入SKU编码" style="width: 100%" />
              </a-form-item>
            </a-col>
          </a-row>
        </template>
        <a-divider orientation="left">物流设置</a-divider>
        <a-row :gutter="24">
          <a-col :span="8">
            <a-form-item field="shippingMethod" label="配送方式">
              <a-select v-model="formData.shippingMethod" placeholder="请选择配送方式" style="width: 100%">
                <a-option :value="0">包邮</a-option>
                <a-option :value="1">运费模板</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="8" v-if="formData.shippingMethod === 1">
            <a-form-item field="shippingTemplateId" label="运费模板">
              <a-select v-model="formData.shippingTemplateId" placeholder="请选择运费模板" style="width: 100%">
                <a-option v-for="template in shippingTemplateList" :key="template.id" :value="template.id">
                  {{ template.name }}
                </a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="weight" label="商品重量(kg)">
              <a-input-number v-model="formData.weight" :min="0" :precision="2" placeholder="请输入商品重量" style="width: 100%" />
            </a-form-item>
          </a-col>
        </a-row>
        <a-row :gutter="24">
          <a-col :span="8">
            <a-form-item field="volume" label="商品体积(m³)">
              <a-input-number v-model="formData.volume" :min="0" :precision="3" placeholder="请输入商品体积" style="width: 100%" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="unit" label="计量单位">
              <a-input v-model="formData.unit" placeholder="请输入计量单位，如：件、个" style="width: 100%" />
            </a-form-item>
          </a-col>
        </a-row>
      </a-card>

      <a-card class="form-card" v-show="currentStep === 3">
        <template #title>
          <div class="card-title">
            <icon-settings />
            其他设置
          </div>
        </template>
        <a-form-item label="商品标签">
          <a-checkbox-group v-model="productTags">
            <a-checkbox value="isHot">
              <div class="checkbox-option">
                <icon-fire />
                <div>
                  <div class="checkbox-title">热销</div>
                  <div class="checkbox-desc">商品将标记为热销商品</div>
                </div>
              </div>
            </a-checkbox>
            <a-checkbox value="isNew">
              <div class="checkbox-option">
                <icon-star />
                <div>
                  <div class="checkbox-title">新品</div>
                  <div class="checkbox-desc">商品将标记为新品</div>
                </div>
              </div>
            </a-checkbox>
            <a-checkbox value="isRecommend">
              <div class="checkbox-option">
                <icon-thumb-up />
                <div>
                  <div class="checkbox-title">推荐</div>
                  <div class="checkbox-desc">商品将标记为推荐商品</div>
                </div>
              </div>
            </a-checkbox>
          </a-checkbox-group>
        </a-form-item>
        <a-form-item field="sort" label="排序">
          <a-input-number v-model="formData.sort" :min="0" placeholder="数字越小越靠前" style="width: 200px" />
        </a-form-item>
        <a-form-item field="groupIds" label="商品分组">
          <a-select v-model="formData.groupIds" multiple placeholder="请选择商品分组" style="width: 100%">
            <a-option v-for="group in groupList" :key="group.id" :value="group.id">
              {{ group.name }}
            </a-option>
          </a-select>
        </a-form-item>
        <a-divider orientation="left">商品详情</a-divider>
        <a-form-item field="detail" label="商品详情">
          <div class="detail-editor-placeholder">
            <icon-edit />
            <span>富文本编辑器区域</span>
          </div>
        </a-form-item>
      </a-card>
    </a-form>

    <div class="form-footer">
      <a-space size="large">
        <a-button v-if="currentStep > 0" @click="prevStep">上一步</a-button>
        <a-button v-if="currentStep < 3" type="primary" @click="nextStep">下一步</a-button>
        <a-button v-if="currentStep === 3" type="primary" size="large" @click="handleSubmit" :loading="submitting">
          <template #icon><icon-check /></template>
          保存商品
        </a-button>
        <a-button size="large" @click="handleCancel">取消</a-button>
      </a-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { productApi, ProductAddParams, ProductDetail, SpecItem, SpecValueItem } from "@/api/modules/product/product";
import { categoryApi } from "@/api/modules/product/category";
import { brandApi } from "@/api/modules/product/brand";
import { shippingTemplateApi } from "@/api/modules/product/shippingTemplate";
import { productGroupApi } from "@/api/modules/product/productGroup";

const route = useRoute();
const router = useRouter();
const formRef = ref();
const currentStep = ref(0);
const submitting = ref(false);
const productId = ref<number | null>(null);
const newSpecValue = ref("");

const categoryTree = ref<any[]>([]);
const brandList = ref<any[]>([]);
const shippingTemplateList = ref<any[]>([]);
const groupList = ref<any[]>([]);
const coverImageList = ref<any[]>([]);
const imageList = ref<any[]>([]);
const productTags = ref<string[]>([]);

const formData = reactive<Partial<ProductAddParams> & Partial<ProductDetail>>({
  categoryId: null,
  brandId: null,
  name: "",
  subtitle: "",
  coverImage: "",
  images: [],
  keywords: "",
  description: "",
  isMultiSpec: 0,
  specs: [],
  skus: [],
  salePrice: 0,
  linePrice: 0,
  costPrice: 0,
  stock: 0,
  virtualSales: 0,
  skuCode: "",
  shippingMethod: 0,
  shippingTemplateId: null,
  weight: 0,
  volume: 0,
  unit: "件",
  sort: 0,
  isHot: 0,
  isNew: 0,
  isRecommend: 0,
  groupIds: [],
  detail: ""
});

const formRules = {
  categoryId: [{ required: true, message: "请选择商品分类" }],
  name: [{ required: true, message: "请输入商品名称" }],
  coverImage: [{ required: true, message: "请上传商品封面" }],
  salePrice: [{ required: true, message: "请输入售价" }],
  stock: [{ required: true, message: "请输入库存" }]
};

const skuPreviewData = computed(() => {
  if (!formData.specs || formData.specs.length === 0) return [];
  const result: any[] = [];
  const generateSkus = (index: number, current: any) => {
    if (index === formData.specs!.length) {
      result.push({ ...current, salePrice: 0, linePrice: 0, stock: 0 });
      return;
    }
    const spec = formData.specs![index];
    for (const value of spec.values) {
      generateSkus(index + 1, { ...current, [spec.name]: value.value });
    }
  };
  generateSkus(0, {});
  return result;
});

watch(
  () => productTags.value,
  (tags) => {
    formData.isHot = tags.includes("isHot") ? 1 : 0;
    formData.isNew = tags.includes("isNew") ? 1 : 0;
    formData.isRecommend = tags.includes("isRecommend") ? 1 : 0;
  }
);

const getCategoryTree = async () => {
  try {
    const data = await categoryApi.tree();
    categoryTree.value = data || [];
  } catch (error) {
    console.error(error);
  }
};

const getBrandList = async () => {
  try {
    const data = await brandApi.list({ pageNum: 1, pageSize: 100 });
    brandList.value = data.list || [];
  } catch (error) {
    console.error(error);
  }
};

const getShippingTemplateList = async () => {
  try {
    const data = await shippingTemplateApi.list({ pageNum: 1, pageSize: 100 });
    shippingTemplateList.value = data.list || [];
  } catch (error) {
    console.error(error);
  }
};

const getGroupList = async () => {
  try {
    const data = await productGroupApi.list({ pageNum: 1, pageSize: 100 });
    groupList.value = data.list || [];
  } catch (error) {
    console.error(error);
  }
};

const getProductDetail = async (id: number) => {
  try {
    const data = await productApi.detail(id);
    Object.assign(formData, data);
    if (data.isHot) productTags.value.push("isHot");
    if (data.isNew) productTags.value.push("isNew");
    if (data.isRecommend) productTags.value.push("isRecommend");
    if (data.coverImage) {
      coverImageList.value = [{ url: data.coverImage, uid: Date.now(), status: "done" }];
    }
    if (data.images && data.images.length > 0) {
      imageList.value = data.images.map((url, index) => ({ url, uid: Date.now() + index, status: "done" }));
    }
  } catch (error) {
    console.error(error);
  }
};

const handleCoverImageChange = (fileList: any[]) => {
  coverImageList.value = fileList;
  if (fileList.length > 0) {
    formData.coverImage = fileList[0].url || "";
  }
};

const handleImageChange = (fileList: any[]) => {
  imageList.value = fileList;
  formData.images = fileList.map((file) => file.url || "");
};

const addSpec = () => {
  formData.specs?.push({
    id: Date.now(),
    name: "",
    sort: (formData.specs?.length || 0) + 1,
    values: []
  });
};

const removeSpec = (index: number) => {
  formData.specs?.splice(index, 1);
};

const addSpecValue = (specIndex: number) => {
  if (!newSpecValue.value.trim()) return;
  formData.specs![specIndex].values.push({
    id: Date.now(),
    value: newSpecValue.value.trim(),
    sort: formData.specs![specIndex].values.length + 1
  });
  newSpecValue.value = "";
};

const removeSpecValue = (specIndex: number, valueIndex: number) => {
  formData.specs![specIndex].values.splice(valueIndex, 1);
};

const prevStep = () => {
  currentStep.value--;
};

const nextStep = async () => {
  if (currentStep.value === 0) {
    try {
      await formRef.value?.validate(["categoryId", "name", "coverImage"]);
      currentStep.value++;
    } catch {}
  } else {
    currentStep.value++;
  }
};

const handleSubmit = async () => {
  try {
    await formRef.value?.validate();
    submitting.value = true;
    if (productId.value) {
      await productApi.edit({ ...formData, id: productId.value });
      arcoMessage.success("商品更新成功");
    } else {
      await productApi.add(formData as ProductAddParams);
      arcoMessage.success("商品添加成功");
    }
    router.back();
  } catch (error) {
    console.error(error);
  } finally {
    submitting.value = false;
  }
};

const handleCancel = () => {
  router.back();
};

onMounted(async () => {
  await Promise.all([
    getCategoryTree(),
    getBrandList(),
    getShippingTemplateList(),
    getGroupList()
  ]);
  if (route.params.id) {
    productId.value = Number(route.params.id);
    await getProductDetail(productId.value);
  }
});
</script>

<style scoped lang="scss">
.product-form-page {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.form-steps {
  margin-bottom: 32px;
  padding: 24px;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.form-card {
  margin-bottom: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  border-radius: 8px;

  .card-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    color: #1d2129;
  }
}

.radio-option,
.checkbox-option {
  display: flex;
  align-items: flex-start;
  padding: 12px 0;
  gap: 12px;

  .radio-title,
  .checkbox-title {
    font-weight: 500;
    color: #1d2129;
  }

  .radio-desc,
  .checkbox-desc {
    font-size: 12px;
    color: #86909c;
    margin-top: 4px;
  }
}

.spec-manager {
  .spec-list {
    margin-bottom: 16px;
  }

  .spec-item {
    background: #f7f8fa;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;

    .spec-header {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 12px;
    }

    .spec-values {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
      align-items: center;
    }

    .spec-value-tag {
      padding: 4px 12px;
    }

    .spec-value-input {
      margin-left: 8px;
    }
  }
}

.sku-preview-table {
  :deep(.arco-table-th) {
    background-color: #f7f8fa;
  }
}

.detail-editor-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
  background: #f7f8fa;
  border: 2px dashed #e5e6eb;
  border-radius: 8px;
  color: #86909c;
  gap: 12px;
}

.form-footer {
  display: flex;
  justify-content: center;
  padding: 24px;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  position: sticky;
  bottom: 24px;
  z-index: 100;
}
</style>
