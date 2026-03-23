import axios from "@/api";
import axiosMock from "@/api";

// 登录 - 优先真实后端，兜底mock
export const loginAPI = async (data: any) => {
  try {
    // 优先调用真实后端
    const res = await axios({
      url: "/sys/auth/login",
      method: "post",
      data
    });
    return res;
  } catch (error: any) {
    // 如果真实后端失败，尝试mock
    console.warn("Real backend failed, trying mock:", error?.message);
    const mockRes = await axiosMock({
      url: "/mock/login",
      method: "post",
      data
    });
    return mockRes;
  }
};

// 获取用户信息 - 优先真实后端，兜底mock
export const getUserInfoAPI = async (params?: any) => {
  try {
    // 优先调用真实后端
    const res = await axios({
      url: "/sys/user/getUserInfo",
      method: "get",
      params
    });
    return res;
  } catch (error: any) {
    // 如果真实后端失败，尝试mock
    console.warn("Real backend failed, trying mock:", error?.message);
    const mockRes = await axiosMock({
      url: "/mock/user/getUserInfo",
      method: "get",
      params
    });
    return mockRes;
  }
};