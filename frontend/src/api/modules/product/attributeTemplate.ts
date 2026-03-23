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

export const attributeTemplateApi = {
  list: async (params?: {
    pageNum?: number;
    pageSize?: number;
    name?: string;
    categoryId?: number;
    status?: string;
  }) => {
    const res = await axios.get("/api/product/attributeTemplate/list", { params });
    return res.data;
  },

  detail: async (id: number) => {
    const res = await axios.get(`/api/product/attributeTemplate/${id}`);
    return res.data;
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
  }) => {
    const res = await axios.post("/api/product/attributeTemplate/add", data);
    return res.data;
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
  }) => {
    const res = await axios.put("/api/product/attributeTemplate/edit", data);
    return res.data;
  },

  delete: async (ids: number[]) => {
    const res = await axios.delete("/api/product/attributeTemplate/delete", { data: { ids } });
    return res.data;
  },

  simpleList: async (categoryId?: number) => {
    const res = await axios.get("/api/product/attributeTemplate/simpleList", { params: { categoryId } });
    return res.data;
  },
};
