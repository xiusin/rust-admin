import axios from "@/api";

export interface PPTSlide {
  id: number;
  projectId: number;
  slideIndex: number;
  title?: string;
  content: any;
  style: any;
  animation?: any;
  createdAt: string;
  updatedAt: string;
}

export interface PPTSlideListParams {
  projectId: number;
}

export interface PPTSlideAddParams {
  projectId: number;
  title?: string;
  content?: any;
  style?: any;
}

export interface PPTSlideEditParams extends PPTSlideAddParams {
  id: number;
}

export interface PPTSlideSortParams {
  projectId: number;
  slideIds: number[];
}

export interface PPTExportParams {
  project_id: number;
  format: string;
}

export interface PolishParams {
  text: string;
}

export interface ContinueParams {
  content: string;
}

export interface SummarizeParams {
  content: string;
}

export interface TranslateParams {
  content: string;
  target_language: string;
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

export const editorApi = {
  getSlides: async (projectId: number): Promise<PPTSlide[]> => {
    const res = await axios.get("/ppt/editor/slides", { params: { projectId } });
    return getData(res);
  },

  addSlide: async (data: PPTSlideAddParams): Promise<number> => {
    const res = await axios.post("/ppt/editor/slide/add", data);
    return getData(res);
  },

  editSlide: async (data: PPTSlideEditParams): Promise<void> => {
    const res = await axios.put("/ppt/editor/slide/edit", data);
    getData(res);
  },

  deleteSlide: async (id: number): Promise<void> => {
    const res = await axios.delete(`/ppt/editor/slide/del/${id}`);
    getData(res);
  },

  sortSlides: async (data: PPTSlideSortParams): Promise<void> => {
    const res = await axios.put("/ppt/editor/slides/sort", data);
    getData(res);
  },

  export: async (data: PPTExportParams): Promise<{ downloadUrl: string }> => {
    const res = await axios.post("/ppt/editor/export", data);
    return getData(res);
  }
};

export const exportPPT = async (data: PPTExportParams): Promise<{ download_url: string }> => {
  const res = await axios.post("/ppt/editor/export", data);
  return getData(res);
};

export const polishText = async (data: PolishParams): Promise<{ polished: string }> => {
  const res = await axios.post("/ppt/ai-edit/polish", { content: data.text });
  return getData(res);
};

export const continueContent = async (data: ContinueParams): Promise<{ continued: string }> => {
  const res = await axios.post("/ppt/ai-edit/continue", data);
  return getData(res);
};

export const summarizeContent = async (data: SummarizeParams): Promise<{ summary: string }> => {
  const res = await axios.post("/ppt/ai-edit/summarize", data);
  return getData(res);
};

export const translateContent = async (data: TranslateParams): Promise<{ translated: string }> => {
  const res = await axios.post("/ppt/ai-edit/translate", data);
  return getData(res);
};
