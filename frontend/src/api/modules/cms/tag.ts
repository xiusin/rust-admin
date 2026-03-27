import axios from "@/api";

export interface CmsTag {
  id: number;
  name: string;
  code: string;
  slug?: string;
  color?: string;
  icon?: string;
  description?: string;
  seoTitle?: string;
  seoKeywords?: string;
  seoDescription?: string;
  contentCount: number;
  sort: number;
  status: boolean;
}

export interface CmsTagItem extends CmsTag {
  createdAt?: string;
  updatedAt?: string;
}

export interface CmsTagDetail extends CmsTag {
  contents?: {
    id: number;
    title: string;
    modelName: string;
  }[];
}

export interface TagListParams {
  pageNum?: number;
  pageSize?: number;
  name?: string;
  code?: string;
  status?: boolean;
  orderBy?: 'sort' | 'contentCount' | 'createdAt';
  orderDirection?: 'asc' | 'desc';
}

export interface TagAddParams {
  name: string;
  code: string;
  slug?: string;
  color?: string;
  icon?: string;
  description?: string;
  seoTitle?: string;
  seoKeywords?: string;
  seoDescription?: string;
  sort?: number;
  status?: boolean;
}

export interface TagEditParams extends TagAddParams {
  id: number;
}

export interface BatchAddParams {
  tags: Array<{
    name: string;
    code: string;
    color?: string;
    description?: string;
  }>;
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
    throw new Error(res.message || '请求失败');
  }
  return res.data;
};

export const tagApi = {
  list: async (params?: TagListParams): Promise<ListResponse<CmsTagItem>> => {
    const res = await axios.get("/cms/tag/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<CmsTagDetail> => {
    const res = await axios.get(`/cms/tag/${id}`);
    return getData(res);
  },

  add: async (data: TagAddParams): Promise<number> => {
    const res = await axios.post("/cms/tag/add", data);
    return getData(res);
  },

  edit: async (data: TagEditParams): Promise<void> => {
    const res = await axios.put("/cms/tag/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/tag/delete", { params: { id } });
    getData(res);
  },

  batchAdd: async (data: BatchAddParams): Promise<number[]> => {
    const res = await axios.post("/cms/tag/batchAdd", data);
    return getData(res);
  },

  cloud: async (limit?: number): Promise<CmsTagItem[]> => {
    const res = await axios.get("/cms/tag/cloud", { params: { limit } });
    return getData(res);
  },

  updateStatus: async (id: number, status: boolean): Promise<void> => {
    const res = await axios.put("/cms/tag/updateStatus", null, { params: { id, status } });
    getData(res);
  },

  checkCode: async (code: string, excludeId?: number): Promise<boolean> => {
    const res = await axios.get("/cms/tag/checkCode", { params: { code, excludeId } });
    return getData(res);
  },

  checkName: async (name: string, excludeId?: number): Promise<boolean> => {
    const res = await axios.get("/cms/tag/checkName", { params: { name, excludeId } });
    return getData(res);
  },

  merge: async (sourceId: number, targetId: number): Promise<void> => {
    const res = await axios.put("/cms/tag/merge", null, { params: { sourceId, targetId } });
    getData(res);
  },

  batchDelete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/cms/tag/batchDelete", { data: { ids } });
    getData(res);
  },

  simpleList: async (params?: { name?: string; status?: boolean }): Promise<CmsTagItem[]> => {
    const res = await axios.get("/cms/tag/simpleList", { params });
    return getData(res);
  },

  getByContent: async (contentId: number): Promise<CmsTagItem[]> => {
    const res = await axios.get(`/cms/tag/byContent/${contentId}`);
    return getData(res);
  },
};
