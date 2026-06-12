import axios from "@/api";

// 获取机构树
export const getDocumentLibraryTreeAPI = (params?: any) => {
  return axios({
    url: "/sys/dept/tree",
    method: "get",
    params
  });
};

// 获取列表数据
export const getDocumentLibraryTableAPI = (params?: any) => {
  return axios({
    url: "/sys/user/list",
    method: "get",
    params
  });
};
