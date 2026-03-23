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

export const productGroupApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
  }) => {
    const res = await axios.get("/api/product/group/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/group/${id}`);
    return res.data;
  },

  add: async (data: { name: string; description?: string; sort?: number; status?: string }) => {
    const res = await axios.post("/api/product/group/add", data);
    return res.data;
  },

  edit: async (data: { id: number; name: string; description?: string; sort?: number; status?: string }) => {
    const res = await axios.put("/api/product/group/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/group/delete", { data: { ids } });
    return res.data;
  },

  simpleList: async () => {
    const res = await axios.get("/api/product/group/simpleList");
    return res.data;
  },
};
