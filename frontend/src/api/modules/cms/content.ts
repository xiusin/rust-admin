import axios from "@/api";
import type { CmsTag } from "./tag";

export type ContentType = "article" | "image" | "video" | "audio" | "file" | "link" | "custom";
export type ContentStatus = "draft" | "pending" | "published" | "offline" | "rejected" | "recycled";

export interface Attachment {
  id: number;
  name: string;
  url: string;
  size: number;
  type: string;
  extension: string;
}

export interface CmsContent {
  id: number;
  modelId: number;
  categoryId?: number;
  title: string;
  slug?: string;
  keywords?: string;
  description?: string;
  authorId?: number;
  source?: string;
  thumbnail?: string;
  images?: string[];
  attachments?: Attachment[];
  contentType: ContentType;
  status: ContentStatus;
  publishTime?: string;
  expireTime?: string;
  sort: number;
  viewCount: number;
  likeCount: number;
  commentCount: number;
  isTop: boolean;
  isRecommend: boolean;
  isHot: boolean;
  allowComment: boolean;
  password?: string;
  template?: string;
  extraData?: Record<string, any>;
  tags?: CmsTag[];
  createdAt?: string;
  updatedAt?: string;
}

export interface CmsContentList extends CmsContent {
  modelName?: string;
  categoryName?: string;
  authorName?: string;
  tagNames?: string[];
}

export interface CmsContentDetail extends CmsContent {
  categoryName?: string;
  authorName?: string;
  categoryPath?: string[];
  content?: string;
  extraFields?: Record<string, any>;
  auditRecords?: AuditRecord[];
}

export interface AuditRecord {
  id: number;
  contentId: number;
  auditorId: number;
  auditorName: string;
  status: ContentStatus;
  remark?: string;
  createdAt: string;
}

export interface ContentVersion {
  id: number;
  contentId: number;
  version: number;
  title: string;
  content?: string;
  extraData?: Record<string, any>;
  operatorId: number;
  operatorName: string;
  remark?: string;
  createdAt: string;
}

export interface ContentListParams {
  pageNum?: number;
  pageSize?: number;
  modelId: number;
  categoryId?: number;
  title?: string;
  status?: ContentStatus;
  contentType?: ContentType;
  authorId?: number;
  isTop?: boolean;
  isRecommend?: boolean;
  isHot?: boolean;
  startTime?: string;
  endTime?: string;
  keywords?: string;
  tagIds?: number[];
}

export interface ContentAddParams {
  modelId: number;
  categoryId?: number;
  title: string;
  slug?: string;
  keywords?: string;
  description?: string;
  source?: string;
  thumbnail?: string;
  images?: string[];
  attachments?: Attachment[];
  contentType?: ContentType;
  status?: ContentStatus;
  publishTime?: string;
  expireTime?: string;
  sort?: number;
  isTop?: boolean;
  isRecommend?: boolean;
  isHot?: boolean;
  allowComment?: boolean;
  password?: string;
  template?: string;
  content?: string;
  extraData?: Record<string, any>;
  tagIds?: number[];
}

export interface ContentEditParams extends ContentAddParams {
  id: number;
}

export interface AuditParams {
  id: number;
  status: "approved" | "rejected";
  remark?: string;
}

export interface RollbackParams {
  contentId: number;
  versionId: number;
}

interface ApiResponse<T = any> {
  code: number;
  message: string;
  data: T;
}

interface ListResponse<T> {
  list: T[];
  total: number;
  total_pages: number;
  page_num: number;
}

const getData = <T>(res: ApiResponse<T>): T => {
  if (res.code !== 200) {
    throw new Error(res.message || "请求失败");
  }
  return res.data;
};

export const contentApi = {
  list: async (params: ContentListParams): Promise<ListResponse<CmsContentList>> => {
    const res = await axios.get("/cms/content/list", { params });
    return getData(res);
  },

  detail: async (id: number): Promise<CmsContentDetail> => {
    const res = await axios.get(`/cms/content/${id}`);
    return getData(res);
  },

  add: async (data: ContentAddParams): Promise<number> => {
    const res = await axios.post("/cms/content/add", data);
    return getData(res);
  },

  edit: async (data: ContentEditParams): Promise<void> => {
    const res = await axios.put("/cms/content/edit", data);
    getData(res);
  },

  delete: async (id: number): Promise<void> => {
    const res = await axios.delete("/cms/content/delete", { params: { id } });
    getData(res);
  },

  restore: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/content/restore", null, { params: { id } });
    getData(res);
  },

  publish: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/content/publish", null, { params: { id } });
    getData(res);
  },

  offline: async (id: number): Promise<void> => {
    const res = await axios.put("/cms/content/offline", null, { params: { id } });
    getData(res);
  },

  audit: async (data: AuditParams): Promise<void> => {
    const res = await axios.put("/cms/content/audit", data);
    getData(res);
  },

  versions: async (id: number): Promise<ContentVersion[]> => {
    const res = await axios.get(`/cms/content/${id}/versions`);
    return getData(res);
  },

  rollback: async (data: RollbackParams): Promise<void> => {
    const res = await axios.put("/cms/content/rollback", data);
    getData(res);
  },

  batchDelete: async (ids: number[]): Promise<void> => {
    const res = await axios.delete("/cms/content/batchDelete", { data: { ids } });
    getData(res);
  },

  batchPublish: async (ids: number[]): Promise<void> => {
    const res = await axios.put("/cms/content/batchPublish", { ids });
    getData(res);
  },

  batchOffline: async (ids: number[]): Promise<void> => {
    const res = await axios.put("/cms/content/batchOffline", { ids });
    getData(res);
  },

  batchMove: async (ids: number[], categoryId: number): Promise<void> => {
    const res = await axios.put("/cms/content/batchMove", { ids, categoryId });
    getData(res);
  },

  updateTop: async (id: number, isTop: boolean): Promise<void> => {
    const res = await axios.put("/cms/content/updateTop", null, { params: { id, isTop } });
    getData(res);
  },

  updateRecommend: async (id: number, isRecommend: boolean): Promise<void> => {
    const res = await axios.put("/cms/content/updateRecommend", null, { params: { id, isRecommend } });
    getData(res);
  },

  updateHot: async (id: number, isHot: boolean): Promise<void> => {
    const res = await axios.put("/cms/content/updateHot", null, { params: { id, isHot } });
    getData(res);
  },

  recycleList: async (params: {
    modelId: number;
    pageNum?: number;
    pageSize?: number;
  }): Promise<ListResponse<CmsContentList>> => {
    const res = await axios.get("/cms/content/recycle/list", { params });
    return getData(res);
  },

  clearRecycle: async (ids?: number[]): Promise<void> => {
    const res = await axios.delete("/cms/content/recycle/clear", { data: { ids } });
    getData(res);
  }
};
