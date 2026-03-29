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

export const categoryApi = {
  tree: async (params?: { status?: string }): Promise<CategoryTreeItem[]> => {
    const res = await axios.get("/product/category/tree", { params });
    return getData(res);
  },

  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    status?: string;
    parentId?: number;
  }): Promise<ListResponse<CategoryListItem>> => {
    const res = await axios.get("/product/category/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<CategoryListItem> => {
    const res = await axios.get(`/product/category/${id}`);
    return getData(res);
  },

  add: async (data: CategoryAddParams): Promise<void> => {
    const res = await axios.post("/product/category/add", data);
    getData(res);
  },

  edit: async (data: CategoryEditParams): Promise<void> => {
    const res = await axios.put("/product/category/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/category/delete", { data: { ids } });
    getData(res);
  },

  updateStatus: async (id: number, status: string): Promise<void> => {
    const res = await axios.put("/product/category/updateStatus", null, {
      params: { id, status }
    });
    getData(res);
  }
};
