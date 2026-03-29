import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface PaymentOrder {
  id: number;
  order_no: string;
  consumer_id: number;
  payment_method: "wechat" | "alipay" | "yeepay";
  payment_type: "app" | "h5" | "mini" | "qrcode";
  amount: string;
  status: "pending" | "success" | "failed" | "closed" | "refunded";
  transaction_id?: string;
  callback_data?: string;
  paid_at?: string;
  closed_at?: string;
  expires_at?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreatePaymentParams {
  consumer_id: number;
  payment_method: string;
  payment_type: string;
  amount: number;
  order_no?: string;
}

export interface RefundParams {
  order_no: string;
  refund_amount: number;
  reason?: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  status?: string;
  start_time?: string;
  end_time?: string;
}

export const paymentApi = {
  create: async (params: CreatePaymentParams) => {
    const res = await axios.post("/payment/create", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  get: async (order_no: string) => {
    const res = await axios.get(`/payment/${order_no}`);
    return res.data;
  },

  list: async (params: PageParams) => {
    const res = await axios.get("/payment/list", { params });
    return res.data;
  },

  refund: async (params: RefundParams) => {
    const res = await axios.post("/payment/refund", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  close: async (order_no: string) => {
    const res = await axios.post(`/payment/close/${order_no}`);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  statistics: async (consumer_id?: number) => {
    const res = await axios.get("/payment/statistics", { params: { consumer_id } });
    return res.data;
  }
};
