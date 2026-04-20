import axios from "@/api";

// 登录
export const loginAPI = async (data: any) => {
  return await axios({
    url: "/sys/auth/login",
    method: "post",
    data
  });
};

// 获取用户信息
export const getUserInfoAPI = async (params?: any) => {
  return await axios({
    url: "/sys/user/getUserInfo",
    method: "get",
    params
  });
};

