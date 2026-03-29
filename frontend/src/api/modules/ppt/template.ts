import axios from "@/api";

export interface PPTTemplate {
  id: number;
  name: string;
  description?: string;
  coverImage: string;
  category: string;
  categoryName: string;
  industry?: string;
  industryName?: string;
  pageCount: number;
  price: number;
  isFree: boolean;
  isFavorite: boolean;
  downloadCount: number;
  createdAt: string;
}

export interface PPTTemplateListParams {
  pageNum?: number;
  pageSize?: number;
  name?: string;
  category?: string;
  industry?: string;
  type?: string;
}

interface ApiResponse<T = any> {
  code: number;
  message: string;
  data: T;
}

const getData = <T>(res: ApiResponse<T>): T => {
  if (res.code !== 200) {
    throw new Error(res.message || "请求失败");
  }
  return res.data;
};

export const templateApi = {
  list: async (params?: PPTTemplateListParams): Promise<{ list: PPTTemplate[]; total: number }> => {
    const res = await axios.get("/ppt/template/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<PPTTemplate> => {
    const res = await axios.get(`/ppt/template/${id}`);
    return getData(res);
  },

  favorite: async (id: number): Promise<void> => {
    const res = await axios.post(`/ppt/template/favorite/${id}`);
    getData(res);
  },

  unfavorite: async (id: number): Promise<void> => {
    const res = await axios.delete(`/ppt/template/favorite/${id}`);
    getData(res);
  }
};
