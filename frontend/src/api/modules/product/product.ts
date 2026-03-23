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
  isMultiSpec: number;
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
  isMultiSpec?: number;
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
  }) => {
    const res = await axios.get("/api/product/list/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/list/${id}`);
    return res.data;
  },

  add: async (data: ProductAddParams) => {
    const res = await axios.post("/api/product/list/add", data);
    return res.data;
  },

  edit: async (data: any) => {
    const res = await axios.put("/api/product/list/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/list/delete", { data: { ids } });
    return res.data;
  },

  updateStatus: async (id: number, status: number) => {
    const res = await axios.put("/api/product/list/updateStatus", null, {
      params: { id, status },
    });
    return res.data;
  },

  audit: async (data: { id: number; auditStatus: number; auditRemark?: string }) => {
    const res = await axios.put("/api/product/list/audit", data);
    return res.data;
  },

  batchUpdateStatus: async (ids: number[], status: number) => {
    const res = await axios.put("/api/product/list/batchUpdateStatus", { ids, status });
    return res.data;
  },

  statistics: async () => {
    const res = await axios.get("/api/product/list/statistics");
    return res.data;
  },

  simpleList: async (params?: { name?: string; status?: number }) => {
    const res = await axios.get("/api/product/list/simpleList", { params });
    return res.data;
  },
};

export const skuApi = {
  list: async (productId: number) => {
    const res = await axios.get(`/api/product/sku/list/${productId}`);
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/sku/${id}`);
    return res.data;
  },

  add: async (data: any) => {
    const res = await axios.post("/api/product/sku/add", data);
    return res.data;
  },

  edit: async (data: any) => {
    const res = await axios.put("/api/product/sku/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/sku/delete", { data: { ids } });
    return res.data;
  },

  generate: async (productId: number, specs: any[]) => {
    const res = await axios.post("/api/product/sku/generate", { productId, specs });
    return res.data;
  },
};
