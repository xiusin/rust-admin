import axios from "@/api";

export interface CategoryTreeItem {
  id: number;
  parentId: number;
  name: string;
  icon?: string;
  image?: string;
  sort: number;
  level: number;
  path: string;
  status: string;
  showInNav: number;
  children?: CategoryTreeItem[];
}

export interface CategoryListItem {
  id: number;
  parentId: number;
  name: string;
  icon?: string;
  image?: string;
  sort: number;
  level: number;
  path: string;
  status: string;
  showInNav: number;
  createdAt?: string;
  parentName?: string;
}

export interface CategoryAddParams {
  name: string;
  parentId?: number;
  icon?: string;
  image?: string;
  sort?: number;
  status?: string;
  showInNav?: number;
}

export interface CategoryEditParams {
  id: number;
  name: string;
  icon?: string;
  image?: string;
  sort?: number;
  status?: string;
  showInNav?: number;
}

export const categoryApi = {
  tree: async (params?: { status?: string }) => {
    const res = await axios.get("/api/product/category/tree", { params });
    return res.data;
  },

  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
    parentId?: number;
  }) => {
    const res = await axios.get("/api/product/category/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/category/${id}`);
    return res.data;
  },

  add: async (data: CategoryAddParams) => {
    const res = await axios.post("/api/product/category/add", data);
    return res.data;
  },

  edit: async (data: CategoryEditParams) => {
    const res = await axios.put("/api/product/category/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/category/delete", { data: { ids } });
    return res.data;
  },

  updateStatus: async (id: number, status: string) => {
    const res = await axios.put("/api/product/category/updateStatus", null, {
      params: { id, status },
    });
    return res.data;
  },
};
