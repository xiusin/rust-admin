import axios from "@/api";

export interface ProductGroupListItem {
  id: number;
  name: string;
  description?: string;
  sort: number;
  status: string;
  productCount: number;
  createdAt?: string;
}

export interface ProductGroupDetail {
  id: number;
  name: string;
  description?: string;
  sort: number;
  status: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface ProductGroupSimple {
  id: number;
  name: string;
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

export const productGroupApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }): Promise<ListResponse<ProductGroupListItem>> => {
    const res = await axios.get("/product/group/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<ProductGroupDetail> => {
    const res = await axios.get(`/product/group/${id}`);
    return getData(res);
  },

  add: async (data: { name: string; description?: string; sort?: number; status?: string }): Promise<void> => {
    const res = await axios.post("/product/group/add", data);
    getData(res);
  },

  edit: async (data: { id: number; name: string; description?: string; sort?: number; status?: string }): Promise<void> => {
    const res = await axios.put("/product/group/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/group/delete", { data: { ids } });
    getData(res);
  },

  simpleList: async (): Promise<ProductGroupSimple[]> => {
    const res = await axios.get("/product/group/simpleList");
    return getData(res);
  }
};
