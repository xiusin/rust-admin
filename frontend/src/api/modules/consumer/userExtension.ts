import axios from 'axios';
import { Message } from '@arco-design/web-vue';

export interface UserOauthModel {
  id: number;
  consumer_id: number;
  oauth_type: string;
  oauth_id: string;
  oauth_name?: string;
  oauth_avatar?: string;
  oauth_token?: string;
  refresh_token?: string;
  token_expires_at?: string;
  is_primary: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface LevelConfigModel {
  id: number;
  level: number;
  level_name: string;
  min_exp: number;
  max_exp?: number;
  icon?: string;
  color?: string;
  discount_rate: string;
  privileges?: Record<string, unknown>;
  created_at?: string;
  updated_at?: string;
}

export interface UserLevelModel {
  id: number;
  consumer_id: number;
  level: number;
  level_name: string;
  total_exp: number;
  current_exp: number;
  icon?: string;
  color?: string;
  discount_rate: string;
  privileges?: Record<string, unknown>;
  created_at?: string;
  updated_at?: string;
}

export interface LevelRecordModel {
  id: number;
  consumer_id: number;
  old_level: number;
  new_level: number;
  exp_change: number;
  source: string;
  source_id?: string;
  remark?: string;
  created_at?: string;
}

export interface UserTagModel {
  id: number;
  name: string;
  tag_type: string;
  category?: string;
  color?: string;
  icon?: string;
  description?: string;
  created_by?: number;
  created_at?: string;
  updated_at?: string;
}

export interface UserBanModel {
  id: number;
  consumer_id: number;
  ban_type: string;
  reason: string;
  status: string;
  start_at?: string;
  end_at?: string;
  banned_by?: number;
  unban_at?: string;
  unban_by?: number;
  unban_reason?: string;
  created_at?: string;
  updated_at?: string;
}

export interface ConsumerStatisticsModel {
  id: number;
  consumer_id: number;
  year: number;
  total_orders: number;
  total_amount: string;
  total_refund: string;
  total_expense: string;
  avg_order_amount: string;
  created_at?: string;
  updated_at?: string;
}

export interface BindOauthParams {
  consumer_id: number;
  oauth_type: string;
  oauth_id: string;
  oauth_name?: string;
  oauth_avatar?: string;
  oauth_token?: string;
  refresh_token?: string;
  token_expires_at?: string;
}

export interface AddExpParams {
  consumer_id: number;
  exp: number;
  source: string;
  source_id?: string;
  remark?: string;
}

export interface CreateTagParams {
  name: string;
  tag_type: string;
  category?: string;
  color?: string;
  icon?: string;
  description?: string;
  created_by?: number;
}

export interface BanUserParams {
  consumer_id: number;
  ban_type: string;
  reason: string;
  end_at?: string;
  banned_by?: number;
}

export const userExtensionApi = {
  bindOauth: async (params: BindOauthParams) => {
    const res = await axios.post('/user-ext/oauth/bind', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  unbindOauth: async (params: { consumer_id: number; oauth_type: string }) => {
    const res = await axios.post('/user-ext/oauth/unbind', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  listOauth: async (consumerId: number) => {
    const res = await axios.get('/user-ext/oauth/list', { params: { consumer_id: consumerId } });
    return res.data;
  },

  setPrimaryBind: async (params: { consumer_id: number; oauth_id: number }) => {
    const res = await axios.post('/user-ext/oauth/set-primary', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getUserLevel: async (consumerId: number) => {
    const res = await axios.get('/user-ext/level/get', { params: { consumer_id: consumerId } });
    return res.data;
  },

  addExp: async (params: AddExpParams) => {
    const res = await axios.post('/user-ext/level/add-exp', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getLevelConfigs: async () => {
    const res = await axios.get('/user-ext/level/configs');
    return res.data;
  },

  getLevelRecords: async (params: { consumer_id: number; page_num?: number; page_size?: number }) => {
    const res = await axios.get('/user-ext/level/records', { params });
    return res.data;
  },

  createLevelConfig: async (params: { level: number; level_name: string; min_exp: number; max_exp?: number; icon?: string; color?: string; discount_rate: string; privileges?: Record<string, unknown> }) => {
    const res = await axios.post('/user-ext/level/create-config', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  createTag: async (params: CreateTagParams) => {
    const res = await axios.post('/user-ext/tag/create', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  updateTag: async (params: { id: number; name?: string; category?: string; color?: string; icon?: string; description?: string }) => {
    const res = await axios.put('/user-ext/tag/update', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  deleteTag: async (id: number) => {
    const res = await axios.delete('/user-ext/tag/delete', { params: { id } });
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  listTags: async (params?: { page_num?: number; page_size?: number; tag_type?: string; category?: string; name?: string }) => {
    const res = await axios.get('/user-ext/tag/list', { params });
    return res.data;
  },

  addUserTags: async (params: { consumer_id: number; tag_ids: number[]; source?: string; source_desc?: string }) => {
    const res = await axios.post('/user-ext/tag/add-user', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  removeUserTags: async (params: { consumer_id: number; tag_ids: number[] }) => {
    const res = await axios.post('/user-ext/tag/remove-user', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getUserTags: async (consumerId: number) => {
    const res = await axios.get('/user-ext/tag/user-tags', { params: { consumer_id: consumerId } });
    return res.data;
  },

  banUser: async (params: BanUserParams) => {
    const res = await axios.post('/user-ext/ban/user', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  unbanUser: async (params: { consumer_id: number; unban_reason?: string; unban_by?: number }) => {
    const res = await axios.post('/user-ext/ban/unban', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  checkBanStatus: async (consumerId: number) => {
    const res = await axios.get('/user-ext/ban/check', { params: { consumer_id: consumerId } });
    return res.data;
  },

  getBanHistory: async (params?: { page_num?: number; page_size?: number; consumer_id?: number; status?: string }) => {
    const res = await axios.get('/user-ext/ban/history', { params });
    return res.data;
  },

  getStatistics: async (consumerId: number) => {
    const res = await axios.get('/user-ext/statistics/get', { params: { consumer_id: consumerId } });
    return res.data;
  },

  getConsumeTrend: async (params: { consumer_id: number; months?: number }) => {
    const res = await axios.get('/user-ext/statistics/trend', { params });
    return res.data;
  },

  getYearStatistics: async (params: { consumer_id: number; year: string }) => {
    const res = await axios.get('/user-ext/statistics/year', { params });
    return res.data;
  },

  updatePhone: async (params: { consumer_id: number; phone: string }) => {
    const res = await axios.post('/user-ext/update-phone', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  updateEmail: async (params: { consumer_id: number; email: string }) => {
    const res = await axios.post('/user-ext/update-email', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  resetPassword: async (params: { consumer_id: number; new_password: string }) => {
    const res = await axios.post('/user-ext/reset-password', params);
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },

  deactivate: async (consumerId: number) => {
    const res = await axios.post('/user-ext/deactivate', { consumer_id: consumerId });
    if (res.data.message !== 'success') {
      Message.error(res.data.message);
    }
    return res.data;
  },
};
