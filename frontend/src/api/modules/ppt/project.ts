import axios from "@/api";

export interface PPTProject {
  id: number;
  userId: number;
  title: string;
  description?: string;
  sourceType: string;
  status: string;
  industry?: string;
  industryName?: string;
  createdAt: string;
  updatedAt: string;
}

export interface PPTProjectListParams {
  pageNum?: number;
  pageSize?: number;
  title?: string;
  status?: string;
}

export interface PPTProjectAddParams {
  title: string;
  description?: string;
  sourceType: string;
  industry?: string;
  styleId?: number | null;
}

export interface PPTProjectEditParams extends PPTProjectAddParams {
  id: number;
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

export const projectApi = {
  list: async (params?: PPTProjectListParams): Promise<{ list: PPTProject[]; total: number }> => {
    const res = await axios.get("/ppt/project/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<PPTProject> => {
    const res = await axios.get(`/ppt/project/${id}`);
    return getData(res);
  },

  add: async (data: PPTProjectAddParams): Promise<number> => {
    const res = await axios.post("/ppt/project/add", data);
    return getData(res);
  },

  edit: async (data: PPTProjectEditParams): Promise<void> => {
    const res = await axios.put("/ppt/project/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete(`/ppt/project/del/${id}`);
    getData(res);
  },

  copy: async (id: number): Promise<number> => {
    const res = await axios.post(`/ppt/project/copy/${id}`);
    return getData(res);
  }
};
