import axios from "@/api";

export type FormType = 'create' | 'edit' | 'detail' | 'search';
export type FormLayout = 'horizontal' | 'vertical' | 'inline';

export interface FormGroup {
  key: string;
  title?: string;
  collapsible?: boolean;
  defaultCollapsed?: boolean;
  fields: FormField[];
  columns?: number;
}

export interface FormField {
  fieldId: number;
  fieldCode: string;
  fieldName: string;
  span?: number;
  offset?: number;
  labelWidth?: number | string;
  hidden?: boolean;
  condition?: FieldCondition;
  defaultValue?: any;
  props?: Record<string, any>;
}

export interface FieldCondition {
  field: string;
  operator: 'eq' | 'neq' | 'gt' | 'gte' | 'lt' | 'lte' | 'in' | 'notIn' | 'empty' | 'notEmpty';
  value: any;
}

export interface FormAction {
  key: string;
  label: string;
  type: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'default';
  icon?: string;
  action: 'submit' | 'reset' | 'cancel' | 'custom';
  handler?: string;
  confirm?: string;
  show?: FieldCondition;
  loading?: boolean;
}

export interface FormRule {
  field: string;
  rules: ValidationRule[];
}

export interface ValidationRule {
  type: 'required' | 'email' | 'url' | 'phone' | 'pattern' | 'min' | 'max' | 'minLength' | 'maxLength' | 'custom';
  value?: any;
  message: string;
  trigger?: 'blur' | 'change';
}

export interface FormHook {
  event: 'beforeLoad' | 'afterLoad' | 'beforeSubmit' | 'afterSubmit' | 'onFieldChange';
  handler: string;
  async?: boolean;
}

export interface FormConfig {
  id: number;
  modelId: number;
  name: string;
  code: string;
  formType: FormType;
  layout: FormLayout;
  groups: FormGroup[];
  actions: FormAction[];
  rules: FormRule[];
  hooks: FormHook[];
  isDefault: boolean;
  status: boolean;
  createdAt?: string;
  updatedAt?: string;
}

export interface FormConfigItem extends FormConfig {
  modelName?: string;
  fieldCount?: number;
}

export interface FormConfigDetail extends FormConfig {
  modelName?: string;
  modelCode?: string;
}

export interface FormSchema {
  model: string;
  formType: FormType;
  layout: FormLayout;
  labelWidth?: number | string;
  labelPosition?: 'left' | 'right' | 'top';
  size?: 'large' | 'default' | 'small';
  groups: FormGroup[];
  actions: FormAction[];
  rules: Record<string, ValidationRule[]>;
}

export interface FormAddParams {
  modelId: number;
  name: string;
  code: string;
  formType: FormType;
  layout?: FormLayout;
  groups: FormGroup[];
  actions?: FormAction[];
  rules?: FormRule[];
  hooks?: FormHook[];
  isDefault?: boolean;
  status?: boolean;
}

export interface FormEditParams extends FormAddParams {
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

export const formApi = {
  list: async (modelId: number): Promise<FormConfigItem[]> => {
    const res = await axios.get("/cms/form/list", { params: { modelId } });
    return getData(res);
  },

  detail: async (id: number): Promise<FormConfigDetail> => {
    const res = await axios.get(`/cms/form/${id}`);
    return getData(res);
  },

  add: async (data: FormAddParams): Promise<number> => {
    const res = await axios.post("/cms/form/add", data);
    return getData(res);
  },

  edit: async (data: FormEditParams): Promise<void> => {
    const res = await axios.put("/cms/form/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/form/delete", { params: { id } });
    getData(res);
  },

  preview: async (id: number): Promise<FormSchema> => {
    const res = await axios.get(`/cms/form/${id}/preview`);
    return getData(res);
  },

  setDefault: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/form/setDefault", null, { params: { id } });
    getData(res);
  },

  updateStatus: async (id: number, status: boolean): Promise<void> => {
    const res = await axios.put("/cms/form/updateStatus", null, { params: { id, status } });
    getData(res);
  },

  copy: async (id: number): Promise<number> => {
    const res = await axios.post("/cms/form/copy", null, { params: { id } });
    return getData(res);
  },

  getByModel: async (modelId: number, formType: FormType): Promise<FormConfigDetail> => {
    const res = await axios.get("/cms/form/getByModel", { params: { modelId, formType } });
    return getData(res);
  },

  getDefault: async (modelId: number, formType: FormType): Promise<FormConfigDetail> => {
    const res = await axios.get("/cms/form/getDefault", { params: { modelId, formType } });
    return getData(res);
  },
};
