import axios from 'axios';
import { Message } from '@arco-design/web-vue';

export interface UserTagBrief {
  id: number;
  name: string;
  color?: string;
  category?: string;
}

export interface ConsumerListItem {
  id: number;
  phone: string;
  email?: string;
  nickname?: string;
  avatar?: string;
  status: string;
  risk_score: number;
  wechat_openid?: string;
  wechat_unionid?: string;
  last_login_at?: string;
  last_login_ip?: string;
  created_at?: string;
  level?: number;
  level_name?: string;
  total_exp?: number;
  total_consumption?: string;
  order_count?: number;
  oauth_types?: string[];
  tags?: UserTagBrief[];
}

export interface ConsumerInfo {
  id: number;
  phone: string;
  email?: string;
  nickname?: string;
  avatar?: string;
  status: 'normal' | 'locked' | 'deactivated';
  risk_score: number;
  wechat_openid?: string;
  wechat_unionid?: string;
  last_login_at?: string;
  last_login_ip?: string;
  created_at?: string;
  updated_at?: string;
}

export interface LoginLog {
  id: number;
  consumer_id?: number;
  phone?: string;
  login_type: string;
  success: boolean;
  fail_reason?: string;
  ip_address?: string;
  user_agent?: string;
  device_type?: string;
  login_at: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  phone?: string;
  status?: string;
  keyword?: string;
  level?: number;
}

export interface LoginParams {
  phone: string;
  password: string;
}

export interface RegisterParams {
  phone: string;
  password: string;
  sms_code: string;
}

export interface UpdateParams {
  nickname?: string;
  email?: string;
  avatar?: string;
  phone?: string;
  gender?: string;
  birthday?: string;
}

export const consumerApi = {
  register: async (params: RegisterParams) => {
    const res = await axios.post('/consumer/register', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  login: async (params: LoginParams) => {
    const res = await axios.post('/consumer/login', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getInfo: async (id: number) => {
    const res = await axios.get(`/consumer/info/${id}`);
    return res.data;
  },

  update: async (id: number, params: UpdateParams) => {
    const res = await axios.put(`/consumer/update/${id}`, params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  list: async (params: PageParams) => {
    const res = await axios.get('/consumer/list', { params });
    return res.data;
  },

  loginLogs: async (params: PageParams & { consumer_id?: number; login_type?: string; success?: boolean }) => {
    const res = await axios.get('/consumer/login-logs', { params });
    return res.data;
  },
};
