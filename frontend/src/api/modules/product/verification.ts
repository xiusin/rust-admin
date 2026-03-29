import axios from "@/api";

export interface VerificationCodeListItem {
  id: number;
  orderId: number;
  orderNo: string;
  productId: number;
  productName: string;
  productImage: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  code: string;
  qrCode?: string;
  totalCount: number;
  usedCount: number;
  remainCount: number;
  status: number;
  statusName: string;
  expireAt?: string;
  verifiedAt?: string;
  storeName?: string;
  createdAt?: string;
}

export interface VerificationCodeDetail {
  id: number;
  orderId: number;
  orderNo: string;
  orderItemId: number;
  productId: number;
  productName: string;
  productImage: string;
  skuId?: number;
  skuCode?: string;
  specText?: string;
  code: string;
  qrCode?: string;
  totalCount: number;
  usedCount: number;
  remainCount: number;
  status: number;
  expireAt?: string;
  verifiedAt?: string;
  verifiedBy?: number;
  verifiedByName?: string;
  storeId?: number;
  storeName?: string;
  createdAt?: string;
  updatedAt?: string;
  verificationLogs: VerificationLogItem[];
}

export interface VerificationLogItem {
  id: number;
  verificationCodeId: number;
  code: string;
  orderNo: string;
  productName: string;
  storeId?: number;
  storeName?: string;
  verifiedBy?: number;
  verifiedByName?: string;
  remark?: string;
  createdAt?: string;
}

export interface VerificationResult {
  success: boolean;
  message: string;
  code: string;
  productName: string;
  specText?: string;
  orderNo: string;
  remainCount: number;
  storeName?: string;
  verifiedAt?: string;
}

export interface VerificationQueryResult {
  code: string;
  productName: string;
  productImage: string;
  specText?: string;
  orderNo: string;
  totalCount: number;
  usedCount: number;
  remainCount: number;
  status: number;
  statusName: string;
  expireAt?: string;
  isValid: boolean;
  message: string;
}

export interface VerificationStatistics {
  totalCodes: number;
  pendingCount: number;
  verifiedCount: number;
  expiredCount: number;
  refundedCount: number;
  todayVerifiedCount: number;
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

export const verificationApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    code?: string;
    orderNo?: string;
    status?: number;
    storeId?: number;
    startTime?: string;
    endTime?: string;
  }): Promise<ListResponse<VerificationCodeListItem>> => {
    const res = await axios.get("/product/verification/list", { params });
    return getData(res);
  },

  verify: async (data: { code: string; storeId?: number; remark?: string }): Promise<VerificationResult> => {
    const res = await axios.post("/product/verification/verify", data);
    return getData(res);
  },

  query: async (code: string): Promise<VerificationQueryResult> => {
    const res = await axios.get("/product/verification/query", { params: { code } });
    return getData(res);
  },

  logList: async (params?: {
    pageNum?: number;
    pageSize?: number;
    code?: string;
    orderNo?: string;
    storeId?: number;
    startTime?: string;
    endTime?: string;
  }): Promise<ListResponse<VerificationLogItem>> => {
    const res = await axios.get("/product/verification/log", { params });
    return getData(res);
  },

  statistics: async (): Promise<VerificationStatistics> => {
    const res = await axios.get("/product/verification/statistics");
    return getData(res);
  }
};
