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

export const shippingTemplateApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }) => {
    const res = await axios.get("/api/product/shipping/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/shipping/${id}`);
    return res.data;
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
  }) => {
    const res = await axios.post("/api/product/shipping/add", data);
    return res.data;
  },

  edit: async (data: any) => {
    const res = await axios.put("/api/product/shipping/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/shipping/delete", { data: { ids } });
    return res.data;
  },

  calculate: async (params: ShippingCalculateParams) => {
    const res = await axios.post("/api/product/shipping/calculate", params);
    return res.data;
  },

  simpleList: async () => {
    const res = await axios.get("/api/product/shipping/simpleList");
    return res.data;
  },
};
