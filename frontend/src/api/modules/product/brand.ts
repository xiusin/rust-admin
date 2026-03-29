import axios from "@/api";

export interface BrandListItem {
  id: number;
  name: string;
  logo?: string;
  description?: string;
  sort: number;
  status: string;
  productCount: number;
  createdAt?: string;
}

export interface BrandDetail {
  id: number;
  name: string;
  logo?: string;
  description?: string;
  sort: number;
  status: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface BrandSimple {
  id: number;
  name: string;
  logo?: string;
}

export interface BrandAddParams {
  name: string;
  logo?: string;
  description?: string;
  sort?: number;
  status?: string;
}

export interface BrandEditParams {
  id: number;
  name: string;
  logo?: string;
  description?: string;
  sort?: number;
  status?: string;
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

export const brandApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }): Promise<ListResponse<BrandListItem>> => {
    const res = await axios.get("/product/brand/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<BrandDetail> => {
    const res = await axios.get(`/product/brand/${id}`);
    return getData(res);
  },

  add: async (data: BrandAddParams): Promise<void> => {
    const res = await axios.post("/product/brand/add", data);
    getData(res);
  },

  edit: async (data: BrandEditParams): Promise<void> => {
    const res = await axios.put("/product/brand/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/brand/delete", { data: { ids } });
    getData(res);
  },

  updateStatus: async (id: number, status: string): Promise<void> => {
    const res = await axios.put("/product/brand/updateStatus", null, {
      params: { id, status }
    });
    getData(res);
  },

  simpleList: async (): Promise<BrandSimple[]> => {
    const res = await axios.get("/product/brand/simpleList");
    return getData(res);
  }
};
