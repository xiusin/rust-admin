import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'

// 创建 axios 实例
const service: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器
service.interceptors.request.use(
  (config) => {
    // 可以在这里添加 token 等认证信息
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    console.error('请求错误:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  (response: AxiosResponse) => {
    const res = response.data
    
    // 根据实际业务逻辑处理响应
    if (res.code !== 200) {
      console.error('响应错误:', res.message || '未知错误')
      return Promise.reject(new Error(res.message || '未知错误'))
    }
    return res
  },
  (error) => {
    console.error('响应错误:', error.message)
    return Promise.reject(error)
  }
)

// 封装请求方法
export const request = {
  get: <T = any>(url: string, config?: AxiosRequestConfig) => {
    return service.get<T, T>(url, config)
  },
  post: <T = any>(url: string, data?: any, config?: AxiosRequestConfig) => {
    return service.post<T, T>(url, data, config)
  },
  put: <T = any>(url: string, data?: any, config?: AxiosRequestConfig) => {
    return service.put<T, T>(url, data, config)
  },
  delete: <T = any>(url: string, config?: AxiosRequestConfig) => {
    return service.delete<T, T>(url, config)
  }
}

export default service