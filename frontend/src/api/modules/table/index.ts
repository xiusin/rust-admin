import axios from "@/api";

// 获取列表数据
export const getCommonTableListAPI = (params?: any) => {
  return axios({
    url: "/sys/user/list",
    method: "get",
    params
  });
};

// 获取列表数据
export const getCustomTableListAPI = (params?: any) => {
  return axios({
    url: "/sys/role/list",
    method: "get",
    params
  });
};
