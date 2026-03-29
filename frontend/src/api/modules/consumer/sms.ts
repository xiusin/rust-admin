import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface SendCodeParams {
  phone: string;
  sms_type: string;
}

export interface VerifyCodeParams {
  phone: string;
  code: string;
}

export interface SMSLog {
  id: number;
  phone: string;
  sms_type: string;
  content?: string;
  code?: string;
  channel: string;
  status: string;
  error_message?: string;
  sent_at?: string;
  expires_at?: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  phone?: string;
  sms_type?: string;
  status?: string;
}

export const smsApi = {
  sendCode: async (params: SendCodeParams) => {
    const res = await axios.post("/sms/send-code", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  verifyCode: async (params: VerifyCodeParams) => {
    const res = await axios.post("/sms/verify-code", params);
    return res.data;
  },

  listLogs: async (params: PageParams) => {
    const res = await axios.get("/sms/logs", { params });
    return res.data;
  },

  statistics: async (phone?: string) => {
    const res = await axios.get("/sms/statistics", { params: { phone } });
    return res.data;
  }
};
