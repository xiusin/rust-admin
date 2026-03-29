import axios from "@/api";

export interface AttributeTemplateListItem {
  id: number;
  name: string;
  categoryId: number;
  categoryName?: string;
  sort: number;
  status: string;
  attributeCount: number;
  createdAt?: string;
}

export interface AttributeItem {
  id: number;
  name: string;
  attrType: number;
  isRequired: number;
  isFilter: number;
  sort: number;
  status: string;
  values: AttributeValueItem[];
}

export interface AttributeValueItem {
  id: number;
  value: string;
  sort: number;
}

export interface AttributeTemplateDetail {
  id: number;
  name: string;
  categoryId: number;
  categoryName?: string;
  sort: number;
  status: string;
  createdAt?: string;
  updatedAt?: string;
  attributes: AttributeItem[];
}

export interface AttributeTemplateSimple {
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

export const attributeTemplateApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    categoryId?: number;
    status?: string;
  }): Promise<ListResponse<AttributeTemplateListItem>> => {
    const res = await axios.get("/product/attribute/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<AttributeTemplateDetail> => {
    const res = await axios.get(`/product/attribute/${id}`);
    return getData(res);
  },

  add: async (data: {
    name: string;
    categoryId: number;
    sort?: number;
    status?: string;
    attributes?: {
      name: string;
      attrType?: number;
      isRequired?: number;
      isFilter?: number;
      sort?: number;
      status?: string;
      values?: {
        value: string;
        sort?: number;
      }[];
    }[];
  }): Promise<void> => {
    const res = await axios.post("/product/attribute/add", data);
    getData(res);
  },

  edit: async (data: {
    id: number;
    name: string;
    categoryId: number;
    sort?: number;
    status?: string;
    attributes?: {
      name: string;
      attrType?: number;
      isRequired?: number;
      isFilter?: number;
      sort?: number;
      status?: string;
      values?: {
        value: string;
        sort?: number;
      }[];
    }[];
  }): Promise<void> => {
    const res = await axios.put("/product/attribute/edit", data);
    getData(res);
  },

  delete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/product/attribute/delete", { data: { ids } });
    getData(res);
  },

  byCategory: async (categoryId: number): Promise<AttributeTemplateSimple[]> => {
    const res = await axios.get(`/product/attribute/byCategory/${categoryId}`);
    return getData(res);
  }
};
