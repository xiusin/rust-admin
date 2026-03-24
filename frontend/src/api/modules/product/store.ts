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

export const storeApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    city?: string;
    status?: string;
  }): Promise<ListResponse<StoreListItem>> => {
    const res = await axios.get("/product/store/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<StoreDetail> => {
    const res = await axios.get(`/product/store/${id}`);
    return getData(res);
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
  }): Promise<void> => {
    const res = await axios.post("/product/store/add", data);
    getData(res);
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
  }): Promise<void> => {
    const res = await axios.put("/product/store/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/store/delete", { data: { ids } });
    getData(res);
  },

  stockList: async (storeId: number, params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
  }): Promise<ListResponse<StoreStockItem>> => {
    const res = await axios.get(`/product/store/stock/${storeId}`, { params });
    return getData(res);
  },

  stockAdjust: async (data: {
    storeId: number;
    productId: number;
    skuId?: number;
    changeNum: number;
    remark?: string;
  }): Promise<void> => {
    const res = await axios.post("/product/store/stock/adjust", data);
    getData(res);
  },

  simpleList: async (): Promise<StoreSimple[]> => {
    const res = await axios.get("/product/store/simpleList");
    return getData(res);
  },

  statistics: async (): Promise<StoreStatistics> => {
    const res = await axios.get("/product/store/statistics");
    return getData(res);
  },
};
