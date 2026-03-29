import axios from "@/api";

export interface ShippingTemplateListItem {
  id: number;
  name: string;
  chargeType: number;
  chargeTypeName: string;
  isFree: number;
  freeConditionType: number;
  freeConditionValue: number;
  status: string;
  productCount: number;
  createdAt?: string;
}

export interface ShippingRegionItem {
  id: number;
  templateId: number;
  regionType: number;
  regionIds?: string;
  regionNames?: string;
  firstUnit: number;
  firstFee: number;
  continueUnit: number;
  continueFee: number;
  isFree: number;
  freeConditionValue: number;
}

export interface ShippingTemplateDetail {
  id: number;
  name: string;
  chargeType: number;
  isFree: number;
  freeConditionType: number;
  freeConditionValue: number;
  status: string;
  createdAt?: string;
  updatedAt?: string;
  regions: ShippingRegionItem[];
}

export interface ShippingTemplateSimple {
  id: number;
  name: string;
  chargeType: number;
}

export interface ShippingCalculateParams {
  templateId: number;
  province: string;
  city?: string;
  totalWeight?: number;
  totalVolume?: number;
  totalQuantity?: number;
  totalAmount?: number;
}

export interface ShippingFeeResult {
  fee: number;
  templateName: string;
  calculationDetail: string;
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

export const shippingTemplateApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }): Promise<ListResponse<ShippingTemplateListItem>> => {
    const res = await axios.get("/product/shipping/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<ShippingTemplateDetail> => {
    const res = await axios.get(`/product/shipping/${id}`);
    return getData(res);
  },

  add: async (data: {
    name: string;
    chargeType?: number;
    isFree?: number;
    freeConditionType?: number;
    freeConditionValue?: number;
    status?: string;
    regions?: {
      regionType?: number;
      regionIds?: string;
      regionNames?: string;
      firstUnit?: number;
      firstFee?: number;
      continueUnit?: number;
      continueFee?: number;
      isFree?: number;
      freeConditionValue?: number;
    }[];
  }): Promise<void> => {
    const res = await axios.post("/product/shipping/add", data);
    getData(res);
  },

  edit: async (data: any): Promise<void> => {
    const res = await axios.put("/product/shipping/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/shipping/delete", { data: { ids } });
    getData(res);
  },

  calculate: async (params: ShippingCalculateParams): Promise<ShippingFeeResult> => {
    const res = await axios.post("/product/shipping/calculate", params);
    return getData(res);
  },

  simpleList: async (): Promise<ShippingTemplateSimple[]> => {
    const res = await axios.get("/product/shipping/simpleList");
    return getData(res);
  }
};
