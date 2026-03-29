import axios from "axios";
import axiosMock from "axios";
import router from "@/router";
import { Message } from "@arco-design/web-vue";
import { getToken, removeToken } from "./token";

// 防止重复跳转登录页的标志
let isRedirectingToLogin = false;

const service = axios.create({
  baseURL: "/api"
});

const mockService = axiosMock.create({
  baseURL: ""
});

// 跳转到登录页的统一处理
const redirectToLogin = () => {
  if (isRedirectingToLogin) return;
  isRedirectingToLogin = true;

  removeToken();
  Message.error("登录状态已过期，请重新登录");

  // 延迟跳转，避免消息提示被立即清除
  setTimeout(() => {
    router.push("/login");
    // 重置标志，允许下次跳转
    setTimeout(() => {
      isRedirectingToLogin = false;
    }, 1000);
  }, 100);
};

service.interceptors.request.use(
  function (config: any) {
    const token = getToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  function (error: any) {
    return Promise.reject(error);
  }
);

mockService.interceptors.request.use(
  function (config: any) {
    return config;
  },
  function (error: any) {
    return Promise.reject(error);
  }
);

service.interceptors.response.use(
  function (response: any) {
    if (response.status != 200) {
      Message.error("服务器异常，请联系管理员");
      return Promise.reject(response.data);
    }
    let res = response.data;

    // 兼容旧版接口（无 code 字段，仅有 message）
    if (res.code === undefined) {
      // HTTP 状态码为 200 即视为成功
      return Promise.resolve(res);
    }

    if (res.code == 401) {
      redirectToLogin();
      return Promise.reject(res);
    } else if (res.code == 404) {
      Message.error("请求连接超时");
      return Promise.reject(res);
    } else if (res.code != 200) {
      Message.error(res.message);
      return Promise.reject(res);
    } else {
      return Promise.resolve(res);
    }
  },
  function (error: any) {
    // 处理HTTP状态码401
    if (error.response && error.response.status === 401) {
      redirectToLogin();
      return Promise.reject(error);
    }

    // 其他错误直接返回
    return Promise.reject(error);
  }
);

mockService.interceptors.response.use(
  function (response: any) {
    return Promise.resolve(response.data);
  },
  function (error: any) {
    return Promise.reject(error);
  }
);

export { service as axios, mockService as axiosMock };
export default service;
