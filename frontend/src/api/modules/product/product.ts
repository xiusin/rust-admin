import axios from "@/api";

export interface ProductListItem {
  id: number;
  categoryId: number;
  categoryName?: string;
  brandId?: number;
  brandName?: string;
  name: string;
  subtitle?: string;
  coverImage: string;
  images?: string[];
  productType: number;
  productTypeName: string;
  status: number;
  statusName: string;
  auditStatus: number;
  auditStatusName: string;
  saleStatus: number;
  saleStatusName: string;
  linePrice: number;
  salePrice: number;
  costPrice: number;
  stock: number;
  sales: number;
  virtualSales: number;
  isMultiSpec: number;
  isHot: number;
  isNew: number;
  isRecommend: number;
  sort: number;
  createdAt?: string;
  updatedAt?: string;
  skuCount: number;
  groupNames?: string[];
}

export interface ProductDetail {
  id: number;
  categoryId: number;
  categoryName?: string;
  brandId?: number;
  brandName?: string;
  name: string;
  subtitle?: string;
  coverImage: string;
  images?: string[];
  video?: string;
  detail?: string;
  productType: number;
  status: number;
  auditStatus: number;
  auditRemark?: string;
  saleStatus: number;
  saleTime?: string;
  linePrice: number;
  salePrice: number;
  costPrice: number;
  stock: number;
  sales: number;
  virtualSales: number;
  limitBuy: number;
  limitType: number;
  shippingMethod: number;
  shippingTemplateId?: number;
  shippingTemplateName?: string;
  weight: number;
  volume: number;
  unit: string;
  sort: number;
  isHot: number;
  isNew: number;
  isRecommend: number;
  keywords?: string;
  description?: string;
  createdAt?: string;
  updatedAt?: string;
  categoryIds?: number[];
  groupIds?: number[];
  specs: SpecItem[];
  skus: SkuItem[];
  attributes: ProductAttributeItem[];
}

export interface SpecItem {
  id: number;
  name: string;
  sort: number;
  values: SpecValueItem[];
}

export interface SpecValueItem {
  id: number;
  value: string;
  image?: string;
  colorCode?: string;
  sort: number;
}

export interface SkuItem {
  id: number;
  skuCode: string;
  specValueIds: string;
  specText: string;
  image?: string;
  salePrice: number;
  linePrice: number;
  costPrice: number;
  stock: number;
  sales: number;
  weight: number;
  volume: number;
  status: number;
}

export interface ProductAttributeItem {
  id: number;
  attributeId: number;
  attributeName: string;
  attributeValue: string;
}

export interface ProductAddParams {
  categoryId: number;
  brandId?: number;
  name: string;
  subtitle?: string;
  coverImage: string;
  images?: string[];
  video?: string;
  detail?: string;
  productType?: number;
  linePrice?: number;
  salePrice?: number;
  costPrice?: number;
  stock?: number;
  virtualSales?: number;
  limitBuy?: number;
  limitType?: number;
  shippingMethod?: number;
  shippingTemplateId?: number;
  weight?: number;
  volume?: number;
  unit?: string;
  sort?: number;
  isHot?: number;
  isNew?: number;
  isRecommend?: number;
  keywords?: string;
  description?: string;
  groupIds?: number[];
  categoryIds?: number[];
  specs?: SpecAddParams[];
  skus?: SkuAddParams[];
  attributes?: ProductAttributeParams[];
}

export interface SpecAddParams {
  name: string;
  sort?: number;
  values: SpecValueAddParams[];
}

export interface SpecValueAddParams {
  value: string;
  image?: string;
  colorCode?: string;
  sort?: number;
}

export interface SkuAddParams {
  skuCode?: string;
  specValueIds?: string;
  specText?: string;
  image?: string;
  salePrice: number;
  linePrice?: number;
  costPrice?: number;
  stock?: number;
  weight?: number;
  volume?: number;
  status?: number;
}

export interface ProductAttributeParams {
  attributeId?: number;
  attributeName: string;
  attributeValue: string;
}

export interface ProductStatistics {
  totalProducts: number;
  onShelfCount: number;
  offShelfCount: number;
  pendingAuditCount: number;
  lowStockCount: number;
}

interface ApiResponse<T = any> {
  code: number;
  message: string;
  data: T;
}

interface ListResponse<T> {
  list: T[];
  total: number;
  total_pages: number;
  page_num: number;
}

const getData = <T>(res: ApiResponse<T>): T => {
  if (res.code !== 200) {
    throw new Error(res.message || '请求失败');
  }
  return res.data;
};

export const productApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    categoryId?: number;
    brandId?: number;
    status?: number;
    auditStatus?: number;
    productType?: number;
    isHot?: number;
    isNew?: number;
    isRecommend?: number;
  }): Promise<ListResponse<ProductListItem>> => {
    const res = await axios.get("/product/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<ProductDetail> => {
    const res = await axios.get(`/product/${id}`);
    return getData(res);
  },

  add: async (data: ProductAddParams): Promise<void> => {
    const res = await axios.post("/product/add", data);
    getData(res);
  },

  edit: async (data: any): Promise<void> => {
    const res = await axios.put("/product/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/delete", { data: { ids } });
    getData(res);
  },

  updateStatus: async (id: number, status: number): Promise<void> => {
    const res = await axios.put("/product/updateStatus", null, {
      params: { id, status },
    });
    getData(res);
  },

  audit: async (data: { id: number; auditStatus: number; auditRemark?: string }): Promise<void> => {
    const res = await axios.put("/product/audit", data);
    getData(res);
  },

  batchUpdateStatus: async (ids: number[], status: number): Promise<void> => {
    const res = await axios.put("/product/batchUpdateStatus", { ids, status });
    getData(res);
  },

  statistics: async (): Promise<ProductStatistics> => {
    const res = await axios.get("/product/statistics");
    return getData(res);
  },

  simpleList: async (params?: { name?: string; status?: number }): Promise<ProductListItem[]> => {
    const res = await axios.get("/product/simpleList", { params });
    return getData(res);
  },
};

export const skuApi = {
  list: async (productId: number): Promise<SkuItem[]> => {
    const res = await axios.get(`/product/sku/list/${productId}`);
    return getData(res);
  },

  detail: async (id: number): Promise<SkuItem> => {
    const res = await axios.get(`/product/sku/${id}`);
    return getData(res);
  },

  add: async (data: any): Promise<void> => {
    const res = await axios.post("/product/sku/add", data);
    getData(res);
  },

  edit: async (data: any): Promise<void> => {
    const res = await axios.put("/product/sku/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/sku/delete", { data: { ids } });
    getData(res);
  },

  generate: async (productId: number, specs: any[]): Promise<SkuItem[]> => {
    const res = await axios.post("/product/sku/generate", { productId, specs });
    return getData(res);
  },
};
