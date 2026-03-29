import axios from "@/api";

export interface ModelConfig {
  enableCategory: boolean;
  enableTag: boolean;
  enableComment: boolean;
  enableAudit: boolean;
  enableVersion: boolean;
  enableRecycle: boolean;
  listPageSize: number;
  defaultSort: string;
}

export interface CmsModel {
  id: number;
  name: string;
  code: string;
  tableName: string;
  description?: string;
  icon?: string;
  categoryId?: number;
  isSystem: boolean;
  isEnabled: boolean;
  sort: number;
  config?: ModelConfig;
  createdAt?: string;
  updatedAt?: string;
}

export interface CmsModelList extends CmsModel {
  categoryName?: string;
  fieldCount?: number;
  contentCount?: number;
}

export interface CmsModelDetail extends CmsModel {
  fields?: CmsFieldInfo[];
  forms?: FormInfo[];
  tables?: TableInfo[];
}

export interface CmsFieldInfo {
  id: number;
  name: string;
  code: string;
  fieldType: string;
}

export interface FormInfo {
  id: number;
  name: string;
  code: string;
  isDefault: boolean;
}

export interface TableInfo {
  id: number;
  name: string;
  code: string;
  isDefault: boolean;
}

export interface ModelListParams {
  pageNum?: number;
  pageSize?: number;
  name?: string;
  code?: string;
  categoryId?: number;
  isEnabled?: boolean;
}

export interface ModelAddParams {
  name: string;
  code: string;
  tableName: string;
  description?: string;
  icon?: string;
  categoryId?: number;
  isSystem?: boolean;
  isEnabled?: boolean;
  sort?: number;
  config?: ModelConfig;
}

export interface ModelEditParams extends ModelAddParams {
  id: number;
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

export const modelApi = {
  list: async (params?: ModelListParams): Promise<ListResponse<CmsModelList>> => {
    const res = await axios.get("/cms/model/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<CmsModelDetail> => {
    const res = await axios.get(`/cms/model/${id}`);
    return getData(res);
  },

  add: async (data: ModelAddParams): Promise<number> => {
    const res = await axios.post("/cms/model/add", data);
    return getData(res);
  },

  edit: async (data: ModelEditParams): Promise<void> => {
    const res = await axios.put("/cms/model/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/model/delete", { params: { id } });
    getData(res);
  },

  enable: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/model/enable", null, { params: { id } });
    getData(res);
  },

  disable: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/model/disable", null, { params: { id } });
    getData(res);
  },

  copy: async (id: number): Promise<number> => {
    const res = await axios.post("/cms/model/copy", null, { params: { id } });
    return getData(res);
  },

  simpleList: async (params?: { name?: string; isEnabled?: boolean }): Promise<CmsModel[]> => {
    const res = await axios.get("/cms/model/simpleList", { params });
    return getData(res);
  }
};
