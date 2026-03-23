import axios from 'axios';
import { Message } from '@arco-design/web-vue';

export interface FreightTemplate {
  id: number;
  name: string;
  calculation_type: 'by_weight' | 'by_distance';
  first_weight?: string;
  first_price?: string;
  additional_weight?: string;
  additional_price?: string;
  region_rules: any;
  free_shipping_rules: any;
  is_active: boolean;
  created_by?: number;
  updated_by?: number;
  created_at?: string;
  updated_at?: string;
}

export interface FreightCalculateResp {
  template_id: number;
  base_price: string;
  additional_price: string;
  total_price: string;
  free_shipping: boolean;
}

export interface CalculateParams {
  template_id: number;
  weight: number;
  province?: string;
  city?: string;
  district?: string;
  order_amount?: number;
}

export interface CreateTemplateParams {
  name: string;
  calculation_type: string;
  first_weight?: number;
  first_price?: number;
  additional_weight?: number;
  additional_price?: number;
  region_rules?: string;
  free_shipping_rules?: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  name?: string;
}

export const freightApi = {
  calculate: async (params: CalculateParams): Promise<FreightCalculateResp> => {
    const res = await axios.post('/freight/calculate', params);
    return res.data;
  },

  createTemplate: async (params: CreateTemplateParams) => {
    const res = await axios.post('/freight/template', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  listTemplates: async (params: PageParams) => {
    const res = await axios.get('/freight/templates', { params });
    return res.data;
  },

  updateTemplate: async (id: number, params: Partial<CreateTemplateParams>) => {
    const res = await axios.put(`/freight/template/${id}`, params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  deleteTemplate: async (id: number) => {
    const res = await axios.delete(`/freight/template/${id}`);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },
};
