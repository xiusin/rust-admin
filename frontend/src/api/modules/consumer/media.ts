import axios from "axios";
import { Message } from "@arco-design/web-vue";

export interface MediaFile {
  id: number;
  consumer_id: number;
  file_name: string;
  file_type: "image" | "video";
  file_format?: string;
  file_size: number;
  file_url?: string;
  thumbnail_url?: string;
  oss_bucket?: string;
  oss_key?: string;
  is_deleted: boolean;
  created_at?: string;
  deleted_at?: string;
}

export interface UploadUrlResp {
  oss_key: string;
  upload_url: string;
  expires_in: number;
}

export interface GenerateUploadUrlParams {
  consumer_id: number;
  file_name: string;
  file_type: string;
}

export interface ConfirmUploadParams {
  consumer_id: number;
  oss_key: string;
  file_name: string;
  file_type: string;
  file_size: number;
  thumbnail_key?: string;
}

export interface PageParams {
  page_num?: number;
  page_size?: number;
  consumer_id?: number;
  file_type?: string;
  start_time?: string;
  end_time?: string;
}

export const mediaApi = {
  generateUploadUrl: async (params: GenerateUploadUrlParams) => {
    const res = await axios.post("/media/upload-url", params);
    return res.data;
  },

  confirmUpload: async (params: ConfirmUploadParams) => {
    const res = await axios.post("/media/confirm", params);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  },

  list: async (params: PageParams) => {
    const res = await axios.get("/media/list", { params });
    return res.data;
  },

  delete: async (id: number) => {
    const res = await axios.delete(`/media/${id}`);
    if (res.data.message !== "success") {
      Message.error(res.data.message);
    }
    return res.data;
  }
};
