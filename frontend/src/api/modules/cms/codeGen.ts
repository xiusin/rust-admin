import axios from "@/api";

export interface GeneratedFile {
  filePath: string;
  fileName: string;
  content: string;
  language: string;
  size?: number;
  lastModified?: string;
}

export interface FileTreeNode {
  name: string;
  path: string;
  isDir: boolean;
  children?: FileTreeNode[];
  extension?: string;
  size?: number;
}

export interface CodeGenConfig {
  modelId: number;
  outputPath?: string;
  overwrite?: boolean;
  templates?: string[];
  options?: CodeGenOptions;
}

export interface CodeGenOptions {
  useTypeScript?: boolean;
  useCompositionApi?: boolean;
  generateApi?: boolean;
  generateViews?: boolean;
  generateComponents?: boolean;
  generateRoutes?: boolean;
  generateStore?: boolean;
  author?: string;
  version?: string;
}

export interface CodeGenResult {
  success: boolean;
  message: string;
  files?: GeneratedFile[];
  errors?: string[];
  warnings?: string[];
}

export interface CodeTemplate {
  id: number;
  name: string;
  code: string;
  type: 'entity' | 'model' | 'args' | 'service' | 'api' | 'vue' | 'component' | 'store' | 'route';
  language: string;
  template: string;
  description?: string;
  isDefault: boolean;
  status: boolean;
}

export interface CodeTemplateItem extends CodeTemplate {
  createdAt?: string;
  updatedAt?: string;
}

export interface CodeTemplateDetail extends CodeTemplate {
  variables?: TemplateVariable[];
}

export interface TemplateVariable {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'array' | 'object';
  description?: string;
  defaultValue?: any;
  required?: boolean;
}

export interface TemplateAddParams {
  name: string;
  code: string;
  type: CodeTemplate['type'];
  language: string;
  template: string;
  description?: string;
  isDefault?: boolean;
  status?: boolean;
  variables?: TemplateVariable[];
}

export interface TemplateEditParams extends TemplateAddParams {
  id: number;
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

export const codeGenApi = {
  preview: async (modelId: number): Promise<GeneratedFile[]> => {
    const res = await axios.get("/cms/codegen/preview", { params: { modelId } });
    return getData(res);
  },

  download: async (modelId: number): Promise<Blob> => {
    const res = await axios.get("/cms/codegen/download", { 
      params: { modelId },
      responseType: 'blob'
    });
    return res as unknown as Blob;
  },

  getFileTree: async (modelId: number): Promise<FileTreeNode[]> => {
    const res = await axios.get("/cms/codegen/filetree", { params: { modelId } });
    return getData(res);
  },

  generate: async (config: CodeGenConfig): Promise<CodeGenResult> => {
    const res = await axios.post("/cms/codegen/generate", config);
    return getData(res);
  },

  generateSingle: async (modelId: number, type: string, template?: string): Promise<GeneratedFile> => {
    const res = await axios.post("/cms/codegen/generateSingle", null, { 
      params: { modelId, type, template } 
    });
    return getData(res);
  },

  getTemplates: async (type?: CodeTemplate['type']): Promise<CodeTemplateItem[]> => {
    const res = await axios.get("/cms/codegen/templates", { params: { type } });
    return getData(res);
  },

  getTemplateDetail: async (id: number): Promise<CodeTemplateDetail> => {
    const res = await axios.get(`/cms/codegen/templates/${id}`);
    return getData(res);
  },

  addTemplate: async (data: TemplateAddParams): Promise<number> => {
    const res = await axios.post("/cms/codegen/templates/add", data);
    return getData(res);
  },

  editTemplate: async (data: TemplateEditParams): Promise<void> => {
    const res = await axios.put("/cms/codegen/templates/edit", data);
    getData(res);
  },

  deleteTemplate: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/codegen/templates/delete", { params: { id } });
    getData(res);
  },

  previewTemplate: async (templateId: number, modelId: number): Promise<string> => {
    const res = await axios.get("/cms/codegen/templates/preview", { 
      params: { templateId, modelId } 
    });
    return getData(res);
  },

  setDefaultTemplate: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/codegen/templates/setDefault", null, { params: { id } });
    getData(res);
  },

  getPreviewFile: async (modelId: number, filePath: string): Promise<GeneratedFile> => {
    const res = await axios.get("/cms/codegen/previewFile", { 
      params: { modelId, filePath } 
    });
    return getData(res);
  },

  validateConfig: async (config: CodeGenConfig): Promise<{ valid: boolean; errors?: string[] }> => {
    const res = await axios.post("/cms/codegen/validate", config);
    return getData(res);
  },

  getAvailableTypes: async (): Promise<Array<{ value: string; label: string; extension: string }>> => {
    const res = await axios.get("/cms/codegen/availableTypes");
    return getData(res);
  },
};
