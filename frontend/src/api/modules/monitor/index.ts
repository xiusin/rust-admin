import axios from "@/api";

// 获取在线用户
export const getOnlineuserAPI = () => {
  return axios({
    url: "/sys/useronline/list",
    method: "get"
  });
};

// 获取定时任务
export const getCrontabAPI = (params?: any) => {
  return axios({
    url: "/sys/job/list",
    method: "get",
    params
  });
};

// 获取定时任务日志
export const getCrontabLogsAPI = (params: any) => {
  return axios({
    url: "/sys/jobinfo/list",
    method: "get",
    params
  });
};
