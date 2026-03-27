import axios from "@/api";

export interface CmsCategory {
  id: number;
  parentId?: number;
  modelId?: number;
  name: string;
  code: string;
  slug?: string;
  icon?: string;
  image?: string;
  description?: string;
  keywords?: string;
  templateList?: string;
  templateDetail?: string;
  pageSize: number;
  sort: number;
  status: boolean;
  seoTitle?: string;
  seoKeywords?: string;
  seoDescription?: string;
  children?: CmsCategory[];
}

export interface CmsCategoryItem extends CmsCategory {
  parentName?: string;
  modelName?: string;
  contentCount?: number;
  level?: number;
}

export interface CmsCategoryTree extends CmsCategory {
  label: string;
  value: number;
  children?: CmsCategoryTree[];
}

export interface CmsCategoryDetail extends CmsCategory {
  parentName?: string;
  parentPath?: string[];
  modelName?: string;
  childrenCount?: number;
  contentCount?: number;
}

export interface CategoryListParams {
  modelId?: number;
  parentId?: number;
  name?: string;
  code?: string;
  status?: boolean;
}

export interface CategoryTreeParams {
  modelId?: number;
  status?: boolean;
  includeContentCount?: boolean;
}

export interface CategoryAddParams {
  parentId?: number;
  modelId?: number;
  name: string;
  code: string;
  slug?: string;
  icon?: string;
  image?: string;
  description?: string;
  keywords?: string;
  templateList?: string;
  templateDetail?: string;
  pageSize?: number;
  sort?: number;
  status?: boolean;
  seoTitle?: string;
  seoKeywords?: string;
  seoDescription?: string;
}

export interface CategoryEditParams extends CategoryAddParams {
  id: number;
}

export interface CategorySortParams {
  items: Array<{
    id: number;
    parentId?: number;
    sort: number;
  }>;
}

interface ApiResponse<T = any> {
  code: number;
  message: string;
  data: T;
}

const getData = <T>(res: ApiResponse<T>): T => {
  if (res.code !== 200) {
    throw new Error(res.message || '请求失败');
  }
  return res.data;
};

export const categoryApi = {
  list: async (params?: CategoryListParams): Promise<CmsCategoryItem[]> => {
    const res = await axios.get("/cms/category/list", { params });
    return getData(res);
  },

  tree: async (params?: CategoryTreeParams): Promise<CmsCategoryTree[]> => {
    const res = await axios.get("/cms/category/tree", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<CmsCategoryDetail> => {
    const res = await axios.get(`/cms/category/${id}`);
    return getData(res);
  },

  add: async (data: CategoryAddParams): Promise<number> => {
    const res = await axios.post("/cms/category/add", data);
    return getData(res);
  },

  edit: async (data: CategoryEditParams): Promise<void> => {
    const res = await axios.put("/cms/category/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/category/delete", { params: { id } });
    getData(res);
  },

  sort: async (data: CategorySortParams): Promise<void> => {
    const res = await axios.put("/cms/category/sort", data);
    getData(res);
  },

  updateStatus: async (id: number, status: boolean): Promise<void> => {
    const res = await axios.put("/cms/category/updateStatus", null, { params: { id, status } });
    getData(res);
  },

  move: async (id: number, parentId?: number): Promise<void> => {
    const res = await axios.put("/cms/category/move", null, { params: { id, parentId } });
    getData(res);
  },

  checkCode: async (code: string, excludeId?: number): Promise<boolean> => {
    const res = await axios.get("/cms/category/checkCode", { params: { code, excludeId } });
    return getData(res);
  },

  getChildren: async (parentId: number): Promise<CmsCategoryItem[]> => {
    const res = await axios.get(`/cms/category/${parentId}/children`);
    return getData(res);
  },

  getPath: async (id: number): Promise<CmsCategoryItem[]> => {
    const res = await axios.get(`/cms/category/${id}/path`);
    return getData(res);
  },
};
