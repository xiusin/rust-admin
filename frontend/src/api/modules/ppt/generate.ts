import axios from "@/api";

export interface PPTGenerateParams {
  title: string;
  description?: string;
  sourceType: string;
  topic?: string;
  outline?: string;
  documentFile?: File | null;
  industry?: string;
  styleId?: number | null;
}

export interface PPTGenerateResult {
  projectId: number;
  taskId: string;
  status: string;
}

export interface PPTGenerateProgress {
  taskId: string;
  status: string;
  progress: number;
  message?: string;
  result?: any;
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

export const generateApi = {
  create: async (data: PPTGenerateParams): Promise<PPTGenerateResult> => {
    const res = await axios.post("/ppt/generate/create", data);
    return getData(res);
  },

  progress: async (taskId: string): Promise<PPTGenerateProgress> => {
    const res = await axios.get(`/ppt/generate/progress/${taskId}`);
    return getData(res);
  },

  cancel: async (taskId: string): Promise<void> => {
    const res = await axios.post(`/ppt/generate/cancel/${taskId}`);
    getData(res);
  }
};
