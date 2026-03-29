import NProgress from "@/config/nprogress";
import pinia from "@/store/index";
import { createRouter, createWebHashHistory } from "vue-router";
import { staticRoutes, notFoundAndNoPower } from "@/router/route.ts";
import { currentlyRoute } from "@/router/route-output";
import { storeToRefs } from "pinia";
import { useUserInfoStore } from "@/store/modules/user-info";
import { useRouteConfigStore } from "@/store/modules/route-config";
import { useRoutingMethod } from "@/hooks/useRoutingMethod";
import { removeToken } from "@/api/token";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [...staticRoutes, ...notFoundAndNoPower]
});

router.beforeEach(async (to: any, _: any, next: any) => {
  NProgress.start();
  const store = useUserInfoStore(pinia);
  const routeStore = useRouteConfigStore(pinia);
  const { token } = storeToRefs(store);
  const { routeTree } = storeToRefs(routeStore);

  // 1、去登录页，无token，放行
  if (to.path === "/login" && !token.value) return next();

  // 2、没有token，直接重定向到登录页
  if (!token.value) return next("/login");

  // 3、去登录页，有token，直接重定向到home页
  if (to.path === "/login" && token.value) {
    currentlyRoute(to);
    return next("/home");
  }

  // 4、去非登录页，有token，路由已加载则放行
  if (routeTree.value && routeTree.value.length > 0) {
    const { openExternalLinks } = useRoutingMethod();
    openExternalLinks(to);
    currentlyRoute(to);
    return next();
  }

  // 5、路由未加载，先获取账号信息和路由信息
  try {
    await store.setAccount();
  } catch (e) {
    console.error("setAccount failed:", e);
    removeToken();
    return next("/login");
  }

  try {
    await routeStore.initSetRouter();
  } catch (e) {
    console.error("initSetRouter failed:", e);
    // 路由加载失败仍然放行，避免死循环
    return next();
  }

  const { isDynamicRoute } = useRoutingMethod();
  if (isDynamicRoute(to.path)) {
    return next({ name: to.name, params: to.params });
  } else {
    return next({ path: to.path, query: to.query });
  }
});

router.onError((error: any) => {
  NProgress.done();
  console.warn("路由错误", error.message);
});

router.afterEach(() => {
  NProgress.done();
});

export default router;
