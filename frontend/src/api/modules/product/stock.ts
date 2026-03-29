import axios from "@/api";

export interface StockListItem {
  productId: number;
  productName: string;
  productImage: string;
  categoryName?: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  stock: number;
  sales: number;
  alertStock?: number;
  isAlert: boolean;
  status: number;
}

export interface StockLogItem {
  id: number;
  productId: number;
  productName: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  changeType: number;
  changeTypeName: string;
  changeNum: number;
  beforeStock: number;
  afterStock: number;
  orderNo?: string;
  remark?: string;
  operatorName?: string;
  createdAt?: string;
}

export interface StockAlertItem {
  id: number;
  productId: number;
  productName: string;
  productImage: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  stock: number;
  alertStock: number;
  isAlert: number;
  lastAlertAt?: string;
}

export interface StockStatistics {
  totalStock: number;
  alertCount: number;
  outOfStockCount: number;
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
    throw new Error(res.message || "请求失败");
  }
  return res.data;
};

export const stockApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
    categoryId?: number;
    stockStatus?: string;
  }): Promise<ListResponse<StockListItem>> => {
    const res = await axios.get("/product/stock/list", { params });
    return getData(res);
  },

  logList: async (params?: {
    pageNum?: number;
    pageSize?: number;
    productId?: number;
    skuId?: number;
    changeType?: number;
    orderNo?: string;
    startTime?: string;
    endTime?: string;
  }): Promise<ListResponse<StockLogItem>> => {
    const res = await axios.get("/product/stock/log", { params });
    return getData(res);
  },

  adjust: async (data: {
    productId: number;
    skuId?: number;
    changeType: number;
    changeNum: number;
    remark?: string;
  }): Promise<void> => {
    const res = await axios.post("/product/stock/adjust", data);
    getData(res);
  },

  alertList: async (params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
    isAlert?: number;
  }): Promise<ListResponse<StockAlertItem>> => {
    const res = await axios.get("/product/stock/alertList", { params });
    return getData(res);
  },

  alertConfig: async (data: { productId: number; skuId?: number; alertStock: number }): Promise<void> => {
    const res = await axios.post("/product/stock/alertConfig", data);
    getData(res);
  },

  statistics: async (): Promise<StockStatistics> => {
    const res = await axios.get("/product/stock/statistics");
    return getData(res);
  }
};
