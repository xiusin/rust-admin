import axios from "@/api";

// ==================== 字典类型 API ====================

// 获取字典类型列表
export const getDictTypeListAPI = (params?: {
  dict_name?: string;
  dict_type?: string;
  status?: string;
  page_num?: number;
  page_size?: number;
}) => {
  return axios({
    url: "/sys/dicttype/list",
    method: "get",
    params
  });
};

// 添加字典类型
export const addDictTypeAPI = (data: {
  dict_name: string;
  dict_type: string;
  order?: number;
  remark?: string;
  status?: string;
}) => {
  return axios({
    url: "/sys/dicttype/add",
    method: "post",
    data
  });
};

// 编辑字典类型
export const editDictTypeAPI = (data: {
  dict_id: string;
  dict_name: string;
  dict_type: string;
  order?: number;
  remark?: string;
  status?: string;
}) => {
  return axios({
    url: "/sys/dicttype/edit",
    method: "put",
    data
  });
};

// 删除字典类型
export const deleteDictTypeAPI = (params: { dict_id: string }) => {
  return axios({
    url: "/sys/dicttype/del",
    method: "delete",
    params
  });
};

// ==================== 字典数据 API ====================

// 获取字典数据列表
export const getDictDataListAPI = (params: {
  dict_type_id: string;
  dict_label?: string;
  dict_value?: string;
  page_num?: number;
  page_size?: number;
}) => {
  return axios({
    url: "/sys/dictdata/list",
    method: "get",
    params
  });
};

// 添加字典数据
export const addDictDataAPI = (data: {
  dict_type_id: string;
  dict_label: string;
  dict_value: string;
  dict_sort?: number;
  remark?: string;
  status?: string;
}) => {
  return axios({
    url: "/sys/dictdata/add",
    method: "post",
    data
  });
};

// 编辑字典数据
export const editDictDataAPI = (data: {
  dict_code: string;
  dict_type_id: string;
  dict_label: string;
  dict_value: string;
  dict_sort?: number;
  remark?: string;
  status?: string;
}) => {
  return axios({
    url: "/sys/dictdata/edit",
    method: "put",
    data
  });
};

// 删除字典数据
export const deleteDictDataAPI = (params: { dict_code: string }) => {
  return axios({
    url: "/sys/dictdata/del",
    method: "delete",
    params
  });
};

// 根据字典类型获取字典数据（用于下拉选择等）
export const getDictDataByTypeAPI = (params: { dict_type: string }) => {
  return axios({
    url: "/sys/dictdata/get_by_type",
    method: "get",
    params
  });
};
