import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface LogisticsDetail {
  id: number;
  tracking_no: string;
  courier_company: string;
  status: "pending" | "in_transit" | "delivering" | "delivered";
  current_location?: string;
  signed_at?: string;
  created_at?: string;
  updated_at?: string;
}

export interface LogisticsNode {
  status: string;
  location: string;
  description: string;
  timestamp: string;
}

export interface LogisticsTrace {
  tracking_no: string;
  courier_company: string;
  nodes: LogisticsNode[];
}

export interface QueryParams {
  tracking_no: string;
  courier_company: string;
}

export interface SubscribeParams {
  tracking_no: string;
  courier_company: string;
  callback_url?: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  tracking_no?: string;
}

export const logisticsApi = {
  query: async (params: QueryParams) => {
    const res = await axios.post("/logistics/query", params);
    return res.data;
  },

  subscribe: async (params: SubscribeParams) => {
    const res = await axios.post("/logistics/subscribe", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  history: async (params: PageParams) => {
    const res = await axios.get("/logistics/history", { params });
    return res.data;
  }
};
