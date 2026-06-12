import { defineStore } from "pinia";
import persistedstateConfig from "@/store/config/index";
import { getDictAPI } from "@/api/modules/system/index";
/**
 * 用户信息
 * @methods setAccount 设置账号信息
 * @methods setToken 设置token
 * @methods logOut 退出登录
 */
const systemStore = () => {
  // 字典数据
  const dict = ref<any>([]);

  // 请求防抖/缓存Promise，防止并发请求
  let fetchPromise: Promise<any> | null = null;

  // 设置字典数据
  async function setDictData() {
    if (fetchPromise) {
      return fetchPromise;
    }

    fetchPromise = (async () => {
      try {
        let dictData = await getDictAPI();
        dict.value = dictData.data || [];
      } finally {
        // 请求结束后，延迟一段时间清除缓存（防抖），避免短时间内频繁请求
        setTimeout(() => {
          fetchPromise = null;
        }, 1000);
      }
    })();

    return fetchPromise;
  }

  return { dict, setDictData };
};

export const useSystemStore = defineStore("system", systemStore, {
  persist: persistedstateConfig("system", ["dict"], sessionStorage)
});
