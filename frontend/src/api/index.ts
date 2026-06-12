import axios from "axios";
import router from "@/router";
import { Message } from "@arco-design/web-vue";
import { getToken, setToken, removeToken } from "./token";

// 防止重复跳转登录页的标志
let isRedirectingToLogin = false;

// 是否正在刷新 token
let isRefreshing = false;
// 等待刷新 token 的请求队列
let requests: any[] = [];

const service = axios.create({
  baseURL: "/api"
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
      return handleUnauthorized(response.config, res);
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
      return handleUnauthorized(error.config, error);
    }

    // 其他错误直接返回
    return Promise.reject(error);
  }
);

// 处理 401 状态，尝试刷新 Token
async function handleUnauthorized(config: any, error: any) {
  // 如果是刷新 token 接口本身返回 401，直接跳转登录
  if (config.url.includes("/sys/user/refersh_token") || config.url.includes("/sys/auth/login")) {
    redirectToLogin();
    return Promise.reject(error);
  }

  if (!isRefreshing) {
    isRefreshing = true;
    try {
      // 调用刷新 token 接口
      const res = await axios.put("/api/sys/user/refersh_token", null, {
        headers: {
          Authorization: `Bearer ${getToken()}`
        }
      });
      const newToken = res.data?.data?.token || res.data?.token || res.data?.data;
      if (newToken) {
        setToken(newToken);
        // 执行队列中的请求
        requests.forEach(cb => cb(newToken));
        requests = [];
        // 重试当前请求
        config.headers.Authorization = `Bearer ${newToken}`;
        return service(config);
      } else {
        throw new Error("刷新 Token 失败");
      }
    } catch (e) {
      requests = [];
      redirectToLogin();
      return Promise.reject(e);
    } finally {
      isRefreshing = false;
    }
  } else {
    // 正在刷新 token，将请求加入队列
    return new Promise(resolve => {
      requests.push((newToken: string) => {
        config.headers.Authorization = `Bearer ${newToken}`;
        resolve(service(config));
      });
    });
  }
}

export { service as axios };
export default service;
