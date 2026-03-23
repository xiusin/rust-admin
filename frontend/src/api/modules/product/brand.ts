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

export const brandApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }) => {
    const res = await axios.get("/api/product/brand/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/brand/${id}`);
    return res.data;
  },

  add: async (data: BrandAddParams) => {
    const res = await axios.post("/api/product/brand/add", data);
    return res.data;
  },

  edit: async (data: BrandEditParams) => {
    const res = await axios.put("/api/product/brand/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/brand/delete", { data: { ids } });
    return res.data;
  },

  updateStatus: async (id: number, status: string) => {
    const res = await axios.put("/api/product/brand/updateStatus", null, {
      params: { id, status },
    });
    return res.data;
  },

  simpleList: async () => {
    const res = await axios.get("/api/product/brand/simpleList");
    return res.data;
  },
};
