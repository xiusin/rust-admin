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

export const stockApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
    categoryId?: number;
    stockStatus?: string;
  }) => {
    const res = await axios.get("/api/product/stock/list", { params });
    return res.data;
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
  }) => {
    const res = await axios.get("/api/product/stock/log", { params });
    return res.data;
  },

  adjust: async (data: {
    productId: number;
    skuId?: number;
    changeType: number;
    changeNum: number;
    remark?: string;
  }) => {
    const res = await axios.post("/api/product/stock/adjust", data);
    return res.data;
  },

  alertList: async (params?: {
    pageNum?: number;
    pageSize?: number;
    productName?: string;
    isAlert?: number;
  }) => {
    const res = await axios.get("/api/product/stock/alertList", { params });
    return res.data;
  },

  alertConfig: async (data: {
    productId: number;
    skuId?: number;
    alertStock: number;
  }) => {
    const res = await axios.post("/api/product/stock/alertConfig", data);
    return res.data;
  },

  statistics: async () => {
    const res = await axios.get("/api/product/stock/statistics");
    return res.data;
  },
};
