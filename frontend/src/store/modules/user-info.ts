import { defineStore } from "pinia";
import { getToken, setToken, removeToken } from "@/api/token";
import { loginAPI, getUserInfoAPI } from "@/api/modules/system/index";
import { useRouteConfigStore } from "./route-config";
import { Message } from "@arco-design/web-vue";

const TOKEN_KEY = "x-token";

export interface UserInfo {
  username: string;
  nickname: string;
  email: string;
  phone: string;
  token: string;
  did: number;
  rid: number;
  roles: string[];
  permissions: string[];
}

export const useUserStore = defineStore("user", {
  state: () => ({
    token: getToken(),
    userInfo: null as UserInfo | null
  }),
  getters: {
    account: state => state.userInfo
  },
  actions: {
    async login(loginForm: any) {
      try {
        const res: any = await loginAPI(loginForm);

        const data = res.data || res;

        let token = "";
        if (data.token) {
          if (typeof data.token === "string") {
            token = data.token;
          } else if (data.token.token) {
            token = data.token.token;
          }
        }

        if (!token) {
          throw new Error("登录失败：未获取到token");
        }

        this.token = token;
        setToken(token);
        localStorage.setItem(TOKEN_KEY, token);

        return data;
      } catch (error: any) {
        Message.error(error?.message || "登录失败");
        return Promise.reject(error);
      }
    },
    async setUserInfo() {
      const token = getToken();
      if (!token) {
        return;
      }

      try {
        const res: any = await getUserInfoAPI();
        const data = res.data || res;

        const userData = data.userinfo || data.user || data;
        const roles = data.roles || userData.roles || [];
        const permissions = data.permissions || userData.permissions || [];

        this.userInfo = {
          username: userData.username || userData.user_name || "",
          nickname: userData.nickname || userData.nick_name || "",
          email: userData.email || "",
          phone: userData.phone || userData.phonenumber || "",
          token: this.token,
          did: parseInt(userData.did || userData.dept_id || 0) || 0,
          rid: parseInt(userData.rid || userData.role_id || 0) || 0,
          roles,
          permissions
        };

        localStorage.setItem("user-info", JSON.stringify(this.userInfo));

        const routeConfigStore = useRouteConfigStore();
        await routeConfigStore.initSetRouter();
      } catch (error: any) {
        console.error("获取用户信息失败:", error);
      }
    },
    setToken(token: string) {
      this.token = token;
      setToken(token);
    },
    async setAccount() {
      await this.setUserInfo();
    },
    logOut() {
      this.token = "";
      this.userInfo = null;
      removeToken();
      localStorage.removeItem("user-info");
    }
  }
});

export const useUserInfoStore = useUserStore;
