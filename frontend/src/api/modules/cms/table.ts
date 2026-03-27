import axios from "@/api";

export interface TableColumn {
  fieldId: number;
  fieldCode: string;
  fieldName: string;
  width?: number;
  minWidth?: number;
  fixed?: 'left' | 'right';
  align?: 'left' | 'center' | 'right';
  sortable?: boolean | 'custom';
  filterable?: boolean;
  showOverflowTooltip?: boolean;
  hidden?: boolean;
  formatter?: string;
  render?: string;
  children?: TableColumn[];
}

export interface TableSearch {
  enable: boolean;
  collapse?: boolean;
  defaultCollapsed?: boolean;
  fields: SearchField[];
}

export interface SearchField {
  fieldId: number;
  fieldCode: string;
  fieldName: string;
  operator: 'eq' | 'neq' | 'like' | 'gt' | 'gte' | 'lt' | 'lte' | 'in' | 'between' | 'isNull';
  defaultValue?: any;
  placeholder?: string;
  span?: number;
}

export interface TableFilter {
  enable: boolean;
  fields: FilterField[];
}

export interface FilterField {
  fieldId: number;
  fieldCode: string;
  fieldName: string;
  type: 'select' | 'checkbox' | 'radio' | 'date' | 'daterange';
  options?: Array<{ label: string; value: any }>;
  multiple?: boolean;
  defaultValue?: any;
}

export interface TableAction {
  key: string;
  label: string;
  type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'default';
  icon?: string;
  action: 'edit' | 'detail' | 'delete' | 'custom';
  handler?: string;
  confirm?: string;
  show?: string;
  disabled?: string;
}

export interface BatchAction {
  key: string;
  label: string;
  type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'default';
  icon?: string;
  action: 'delete' | 'publish' | 'offline' | 'move' | 'export' | 'custom';
  handler?: string;
  confirm?: string;
  show?: string;
  disabled?: string;
}

export interface ToolbarAction {
  key: string;
  label: string;
  type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'default';
  icon?: string;
  action: 'add' | 'import' | 'export' | 'refresh' | 'columns' | 'search' | 'custom';
  handler?: string;
  show?: string;
}

export interface Pagination {
  enable: boolean;
  pageSize?: number;
  pageSizes?: number[];
  layout?: string;
}

export interface TableFeatures {
  selection?: boolean;
  index?: boolean;
  expand?: boolean;
  stripe?: boolean;
  border?: boolean;
  highlightCurrentRow?: boolean;
  showSummary?: boolean;
  summaryMethod?: string;
  lazy?: boolean;
  tree?: {
    childrenField: string;
    hasChildrenField?: string;
  };
}

export interface TableConfig {
  id: number;
  modelId: number;
  name: string;
  code: string;
  columns: TableColumn[];
  search: TableSearch;
  filter: TableFilter;
  actions: TableAction[];
  batchActions: BatchAction[];
  toolbar: ToolbarAction[];
  pagination: Pagination;
  features: TableFeatures;
  isDefault: boolean;
  status: boolean;
  createdAt?: string;
  updatedAt?: string;
}

export interface TableConfigItem extends TableConfig {
  modelName?: string;
  columnCount?: number;
}

export interface TableConfigDetail extends TableConfig {
  modelName?: string;
  modelCode?: string;
}

export interface TableSchema {
  model: string;
  columns: TableColumn[];
  search: TableSearch;
  filter: TableFilter;
  actions: TableAction[];
  batchActions: BatchAction[];
  toolbar: ToolbarAction[];
  pagination: Pagination;
  features: TableFeatures;
  rowKey?: string;
}

export interface TableAddParams {
  modelId: number;
  name: string;
  code: string;
  columns: TableColumn[];
  search?: TableSearch;
  filter?: TableFilter;
  actions?: TableAction[];
  batchActions?: BatchAction[];
  toolbar?: ToolbarAction[];
  pagination?: Pagination;
  features?: TableFeatures;
  isDefault?: boolean;
  status?: boolean;
}

export interface TableEditParams extends TableAddParams {
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

export const tableApi = {
  list: async (modelId: number): Promise<TableConfigItem[]> => {
    const res = await axios.get("/cms/table/list", { params: { modelId } });
    return getData(res);
  },

  detail: async (id: number): Promise<TableConfigDetail> => {
    const res = await axios.get(`/cms/table/${id}`);
    return getData(res);
  },

  add: async (data: TableAddParams): Promise<number> => {
    const res = await axios.post("/cms/table/add", data);
    return getData(res);
  },

  edit: async (data: TableEditParams): Promise<void> => {
    const res = await axios.put("/cms/table/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/table/delete", { params: { id } });
    getData(res);
  },

  preview: async (id: number): Promise<TableSchema> => {
    const res = await axios.get(`/cms/table/${id}/preview`);
    return getData(res);
  },

  setDefault: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/table/setDefault", null, { params: { id } });
    getData(res);
  },

  updateStatus: async (id: number, status: boolean): Promise<void> => {
    const res = await axios.put("/cms/table/updateStatus", null, { params: { id, status } });
    getData(res);
  },

  copy: async (id: number): Promise<number> => {
    const res = await axios.post("/cms/table/copy", null, { params: { id } });
    return getData(res);
  },

  getDefault: async (modelId: number): Promise<TableConfigDetail> => {
    const res = await axios.get("/cms/table/getDefault", { params: { modelId } });
    return getData(res);
  },

  exportData: async (modelId: number, params?: Record<string, any>): Promise<Blob> => {
    const res = await axios.get("/cms/table/export", { 
      params: { modelId, ...params },
      responseType: 'blob'
    });
    return res as unknown as Blob;
  },

  importData: async (modelId: number, file: File): Promise<{ success: number; failed: number; errors?: string[] }> => {
    const formData = new FormData();
    formData.append('file', file);
    formData.append('modelId', String(modelId));
    const res = await axios.post("/cms/table/import", formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    });
    return getData(res);
  },
};
