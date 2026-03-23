import axios from 'axios';

export interface WechatAuthUrlResp {
  url: string;
}

export interface WechatCallbackResp {
  openid: string;
  unionid?: string;
}

export interface MiniLoginResp {
  openid: string;
  session_key: string;
}

export interface AuthUrlParams {
  redirect_uri: string;
  scope: string;
}

export interface CallbackParams {
  code?: string;
  state?: string;
}

export interface MiniLoginParams {
  code: string;
}

export const wechatApi = {
  getAuthUrl: async (params: AuthUrlParams): Promise<WechatAuthUrlResp> => {
    const res = await axios.post('/wechat/auth-url', params);
    return res.data;
  },

  callback: async (params: CallbackParams): Promise<WechatCallbackResp> => {
    const res = await axios.get('/wechat/callback', { params });
    return res.data;
  },

  miniLogin: async (params: MiniLoginParams): Promise<MiniLoginResp> => {
    const res = await axios.post('/wechat/mini/login', params);
    return res.data;
  },
};