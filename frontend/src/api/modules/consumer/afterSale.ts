import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface AfterSaleItem {
  order_item_id: number;
  product_id: number;
  product_name: string;
  sku_id?: number;
  sku_name?: string;
  quantity: number;
  unit_price: string;
  refund_amount: string;
}

export interface AfterSaleModel {
  id: number;
  after_sale_no: string;
  order_id: number;
  order_no: string;
  consumer_id: number;
  type: "refund_only" | "return_refund" | "exchange";
  reason: string;
  description?: string;
  evidence_urls?: string[];
  status: string;
  total_refund_amount: string;
  processor_id?: number;
  processor_name?: string;
  processed_at?: string;
  completed_at?: string;
  closed_at?: string;
  created_at?: string;
  updated_at?: string;
}

export interface AfterSaleDetailModel extends AfterSaleModel {
  items: AfterSaleItemModel[];
  refunds: AfterSaleRefundModel[];
  logistics?: AfterSaleLogisticsModel;
  status_logs: AfterSaleStatusLogModel[];
}

export interface AfterSaleItemModel {
  id: number;
  after_sale_id: number;
  order_item_id: number;
  product_id: number;
  product_name: string;
  sku_id?: number;
  sku_name?: string;
  quantity: number;
  unit_price: string;
  refund_amount: string;
  created_at?: string;
}

export interface AfterSaleRefundModel {
  id: number;
  after_sale_id: number;
  refund_no: string;
  refund_channel: string;
  refund_amount: string;
  status: string;
  transaction_id?: string;
  callback_data?: string;
  refunded_at?: string;
  created_at?: string;
}

export interface AfterSaleLogisticsModel {
  id: number;
  after_sale_id: number;
  logistics_company: string;
  tracking_no: string;
  sender_name?: string;
  sender_phone?: string;
  sender_address?: string;
  receiver_name?: string;
  receiver_phone?: string;
  receiver_address?: string;
  status: string;
  shipped_at?: string;
  received_at?: string;
  created_at?: string;
}

export interface AfterSaleStatusLogModel {
  id: number;
  after_sale_id: number;
  from_status: string;
  to_status: string;
  operator_type: string;
  operator_id?: number;
  operator_name?: string;
  remark?: string;
  created_at?: string;
}

export interface AfterSaleListParams {
  page_num?: number;
  page_size?: number;
  consumer_id?: number;
  order_id?: number;
  status?: string;
  type?: string;
  start_time?: number;
  end_time?: number;
}

export interface ApplyAfterSaleParams {
  order_id: number;
  order_no: string;
  consumer_id: number;
  type: "refund_only" | "return_refund" | "exchange";
  reason: string;
  description?: string;
  evidence_urls?: string[];
  items: AfterSaleItem[];
}

export interface AuditAfterSaleParams {
  after_sale_id: number;
  agree: boolean;
  reject_reason?: string;
  processor_id: number;
  processor_name: string;
}

export interface CreateRefundParams {
  after_sale_id: number;
  refund_channel: string;
  transaction_id?: string;
  refund_amount: string;
}

export interface SubmitLogisticsParams {
  after_sale_id: number;
  logistics_company: string;
  tracking_no: string;
  sender_name?: string;
  sender_phone?: string;
  sender_address?: string;
}

export interface AfterSaleStatistics {
  pending_count: number;
  processing_count: number;
  completed_count: number;
  rejected_count: number;
  total_refund_amount: string;
}

export interface TimeoutConfigModel {
  id: number;
  config_key: string;
  config_name: string;
  timeout_hours: number;
  description?: string;
  created_at?: string;
  updated_at?: string;
}

export const afterSaleApi = {
  apply: async (params: ApplyAfterSaleParams) => {
    const res = await axios.post("/after-sale/apply", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  audit: async (params: AuditAfterSaleParams) => {
    const res = await axios.post("/after-sale/audit", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getDetail: async (afterSaleId: number) => {
    const res = await axios.get("/after-sale/detail", { params: { after_sale_id: afterSaleId } });
    return res.data;
  },

  list: async (params: AfterSaleListParams) => {
    const res = await axios.get("/after-sale/list", { params });
    return res.data;
  },

  close: async (params: { after_sale_id: number; operator_type: string; operator_id?: number; reason?: string }) => {
    const res = await axios.post("/after-sale/close", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  createRefund: async (params: CreateRefundParams) => {
    const res = await axios.post("/after-sale/refund/create", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  refundCallback: async (params: {
    refund_no: string;
    status: string;
    transaction_id?: string;
    callback_data?: string;
    fail_reason?: string;
  }) => {
    const res = await axios.post("/after-sale/refund/callback", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  submitLogistics: async (params: SubmitLogisticsParams) => {
    const res = await axios.post("/after-sale/logistics/submit", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  confirmReceive: async (params: { after_sale_id: number; operator_id: number; operator_name: string }) => {
    const res = await axios.post("/after-sale/logistics/confirm", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  getLogistics: async (afterSaleId: number) => {
    const res = await axios.get("/after-sale/logistics/get", { params: { after_sale_id: afterSaleId } });
    return res.data;
  },

  getStatistics: async () => {
    const res = await axios.get("/after-sale/statistics");
    return res.data;
  },

  getTimeoutConfigs: async () => {
    const res = await axios.get("/after-sale/timeout-configs");
    return res.data;
  }
};
