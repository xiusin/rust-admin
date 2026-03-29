import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface AccountInfo {
  id: number;
  consumer_id: number;
  balance: string;
  frozen_balance: string;
  available_balance: string;
  created_at?: string;
  updated_at?: string;
}

export interface Transaction {
  id: number;
  consumer_id: number;
  transaction_no: string;
  transaction_type: "recharge" | "consume" | "withdraw" | "refund";
  amount: string;
  balance_before: string;
  balance_after: string;
  description?: string;
  related_order_no?: string;
  operator_id?: number;
  created_at?: string;
}

export interface RechargeParams {
  amount: number;
  payment_order_no: string;
}

export interface WithdrawParams {
  amount: number;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  transaction_type?: string;
  start_time?: string;
  end_time?: string;
}

export const financeApi = {
  getAccount: async (consumer_id: number) => {
    const res = await axios.get(`/finance/account/${consumer_id}`);
    return res.data;
  },

  recharge: async (consumer_id: number, params: RechargeParams) => {
    const res = await axios.post(`/finance/recharge/${consumer_id}`, params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  withdraw: async (consumer_id: number, params: WithdrawParams) => {
    const res = await axios.post(`/finance/withdraw/${consumer_id}`, params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  transactions: async (params: PageParams) => {
    const res = await axios.get("/finance/transactions", { params });
    return res.data;
  },

  statistics: async (consumer_id?: number) => {
    const res = await axios.get("/finance/statistics", { params: { consumer_id } });
    return res.data;
  }
};
