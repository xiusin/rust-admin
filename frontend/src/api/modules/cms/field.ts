import axios from "@/api";

export type FieldType = 
  | 'text' 
  | 'textarea' 
  | 'editor' 
  | 'number' 
  | 'decimal' 
  | 'date' 
  | 'datetime' 
  | 'time' 
  | 'select' 
  | 'multiSelect' 
  | 'radio' 
  | 'checkbox' 
  | 'switch' 
  | 'image' 
  | 'images' 
  | 'file' 
  | 'files' 
  | 'video' 
  | 'audio' 
  | 'color' 
  | 'icon' 
  | 'link' 
  | 'json' 
  | 'relation' 
  | 'custom';

export interface FormFieldConfig {
  inputType: string;
  placeholder?: string;
  defaultValue?: string;
  disabled?: boolean;
  readonly?: boolean;
  required?: boolean;
  min?: number;
  max?: number;
  step?: number;
  rows?: number;
  maxlength?: number;
  showWordLimit?: boolean;
  clearable?: boolean;
  showPassword?: boolean;
  prefixIcon?: string;
  suffixIcon?: string;
  options?: FieldOption[];
  multiple?: boolean;
  filterable?: boolean;
  allowCreate?: boolean;
  uploadConfig?: UploadConfig;
  editorConfig?: EditorConfig;
  relationConfig?: RelationConfig;
}

export interface TableFieldConfig {
  width?: number;
  minWidth?: number;
  fixed?: 'left' | 'right' | boolean;
  align?: 'left' | 'center' | 'right';
  sortable?: boolean;
  filterable?: boolean;
  showOverflowTooltip?: boolean;
  formatter?: string;
  render?: string;
}

export interface ValidationRules {
  required?: boolean;
  pattern?: string;
  min?: number;
  max?: number;
  minLength?: number;
  maxLength?: number;
  customRules?: CustomRule[];
}

export interface CustomRule {
  validator: string;
  trigger: 'blur' | 'change';
  message?: string;
}

export interface FieldOption {
  label: string;
  value: string | number;
  disabled?: boolean;
  children?: FieldOption[];
}

export interface UploadConfig {
  accept?: string;
  maxSize?: number;
  maxCount?: number;
  multiple?: boolean;
  drag?: boolean;
  listType?: 'text' | 'picture' | 'picture-card';
}

export interface EditorConfig {
  height?: number;
  placeholder?: string;
  mode?: 'simple' | 'classic' | 'full';
  toolbar?: string[];
}

export interface RelationConfig {
  modelId: number;
  fieldId: number;
  displayField: string;
  multiple?: boolean;
}

export interface CmsField {
  id: number;
  modelId: number;
  name: string;
  code: string;
  fieldType: FieldType;
  dbType: string;
  defaultValue?: string;
  isRequired: boolean;
  isUnique: boolean;
  isSearchable: boolean;
  isSortable: boolean;
  isFilterable: boolean;
  isListShow: boolean;
  isFormShow: boolean;
  isDetailShow: boolean;
  formConfig?: FormFieldConfig;
  tableConfig?: TableFieldConfig;
  validation?: ValidationRules;
  sort: number;
}

export interface CmsFieldItem extends CmsField {
  modelName?: string;
}

export interface CmsFieldDetail extends CmsField {
  modelInfo?: {
    id: number;
    name: string;
    code: string;
  };
}

export interface FieldAddParams {
  modelId: number;
  name: string;
  code: string;
  fieldType: FieldType;
  dbType?: string;
  defaultValue?: string;
  isRequired?: boolean;
  isUnique?: boolean;
  isSearchable?: boolean;
  isSortable?: boolean;
  isFilterable?: boolean;
  isListShow?: boolean;
  isFormShow?: boolean;
  isDetailShow?: boolean;
  formConfig?: FormFieldConfig;
  tableConfig?: TableFieldConfig;
  validation?: ValidationRules;
  sort?: number;
}

export interface FieldEditParams extends FieldAddParams {
  id: number;
}

export interface FieldSortParams {
  modelId: number;
  fieldIds: number[];
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

export const fieldApi = {
  list: async (modelId: number): Promise<CmsFieldItem[]> => {
    const res = await axios.get("/cms/field/list", { params: { modelId } });
    return getData(res);
  },

  detail: async (id: number): Promise<CmsFieldDetail> => {
    const res = await axios.get(`/cms/field/${id}`);
    return getData(res);
  },

  add: async (data: FieldAddParams): Promise<number> => {
    const res = await axios.post("/cms/field/add", data);
    return getData(res);
  },

  edit: async (data: FieldEditParams): Promise<void> => {
    const res = await axios.put("/cms/field/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/field/delete", { params: { id } });
    getData(res);
  },

  sort: async (data: FieldSortParams): Promise<void> => {
    const res = await axios.put("/cms/field/sort", data);
    getData(res);
  },

  batchAdd: async (data: FieldAddParams[]): Promise<number[]> => {
    const res = await axios.post("/cms/field/batchAdd", data);
    return getData(res);
  },

  getFieldTypes: async (): Promise<{ value: FieldType; label: string; dbTypes: string[] }[]> => {
    const res = await axios.get("/cms/field/fieldTypes");
    return getData(res);
  },
};
