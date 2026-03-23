import axios from "@/api";

export interface StoreListItem {
  id: number;
  name: string;
  logo?: string;
  coverImage?: string;
  contactName?: string;
  contactPhone?: string;
  province?: string;
  city?: string;
  district?: string;
  address?: string;
  longitude?: number;
  latitude?: number;
  businessHours?: string;
  sort: number;
  status: string;
  createdAt?: string;
}

export interface StoreDetail {
  id: number;
  name: string;
  logo?: string;
  coverImage?: string;
  contactName?: string;
  contactPhone?: string;
  province?: string;
  city?: string;
  district?: string;
  address?: string;
  longitude?: number;
  latitude?: number;
  businessHours?: string;
  description?: string;
  sort: number;
  status: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface StoreSimple {
  id: number;
  name: string;
  address?: string;
  contactPhone?: string;
}

export interface StoreStockItem {
  id: number;
  storeId: number;
  storeName: string;
  productId: number;
  productName: string;
  productImage: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  stock: number;
  alertStock: number;
  isAlert: boolean;
}

export interface StoreStatistics {
  totalStores: number;
  activeStores: number;
  inactiveStores: number;
}

export const storeApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    city?: string;
    status?: string;
  }) => {
    const res = await axios.get("/api/product/store/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/store/${id}`);
    return res.data;
  },

  add: async (data: {
    name: string;
    logo?: string;
    coverImage?: string;
    contactName?: string;
    contactPhone?: string;
    province?: string;
    city?: string;
    district?: string;
    address?: string;
    longitude?: number;
    latitude?: number;
    businessHours?: string;
    description?: string;
    sort?: number;
    status?: string;
  }) => {
    const res = await axios.post("/api/product/store/add", data);
    return res.data;
  },

  edit: async (data: {
    id: number;
    name: string;
    logo?: string;
    coverImage?: string;
    contactName?: string;
    contactPhone?: string;
    province?: string;
    city?: string;
    district?: string;
    address?: string;
    longitude?: number;
    latitude?: number;
    businessHours?: string;
    description?: string;
    sort?: number;
    status?: string;
  }) => {
    const res = await axios.put("/api/product/store/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/store/delete", { data: { ids } });
    return res.data;
  },

  stockList: async (storeId: number, params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
  }) => {
    const res = await axios.get(`/api/product/store/stock/${storeId}`, { params });
    return res.data;
  },

  stockAdjust: async (data: {
    storeId: number;
    productId: number;
    skuId?: number;
    changeNum: number;
    remark?: string;
  }) => {
    const res = await axios.post("/api/product/store/stock/adjust", data);
    return res.data;
  },

  simpleList: async () => {
    const res = await axios.get("/api/product/store/simpleList");
    return res.data;
  },

  statistics: async () => {
    const res = await axios.get("/api/product/store/statistics");
    return res.data;
  },
};
