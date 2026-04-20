import { HOME_PATH } from "@/config/index";
import Layout from "@/layout/index.vue";
/**
 * 路由path路径与文件夹名称相同，找文件可以浏览器地址快速查找，方便定位文件
 *
 * 路由meta对象参数，我们通常将属性放到meta对象中
 * meta: {
 *   title:     菜单栏以及 tabsView 栏、菜单搜索名称（国际化）
 *   hide:      是否隐藏此路由，不会显示在菜单树，可以访问
 *   disable:   是否停用，不会显示在菜单树，且不可访问
 *   keepAlive: 是否缓存组件状态
 *   affix:     是否固定在 tabsView 栏上
 *   link:      是否是超链接菜单，开启外链条件：1、 link：链接地址不为空  2、iframe: false
 *   iframe:    是否内嵌窗口，开启条件：1、iframe：true  2、link：链接地址不为空
 *   roles:     当前路由权限表示，取角色管理。路由控制显示、隐藏。 超级管理员：admin；普通角色：common
 *   icon:      菜单、tabsView 图标等
 *   svgIcon:   svg图标
 *   sort:      菜单顺序
 * }
 */

/**
 * 静态路由 （默认路由）
 * 此路由不要动，用于做静态路由定向，如果要添加路由，请在 `layout-children` 中添加
 * @description 前端控制路由 直接改 mock/_data/system_menu 中的路由，后端控制则不需要
 * @returns 返回路由菜单数据
 */
export const staticRoutes = [
  {
    path: "/",
    redirect: HOME_PATH
  },
  {
    path: "/login",
    name: "login",
    component: () => import("@/views/login/login.vue"),
    meta: {
      title: "login"
    }
  },
  {
    path: "/layout",
    name: "layout",
    redirect: HOME_PATH,
    component: Layout,
    children: [
      {
        path: "/cms",
        name: "cms",
        redirect: "/cms/model/list",
        meta: {
          title: "cms",
          icon: "icon-apps"
        },
        children: [
          {
            path: "/cms/model",
            name: "cms-model",
            redirect: "/cms/model/list",
            meta: {
              title: "cms-model",
              icon: "icon-apps"
            }
          },
          {
            path: "/cms/model/list",
            name: "cms-model-list",
            component: () => import("@/views/cms/model/list.vue"),
            meta: {
              title: "cms-model-list",
              icon: "icon-list"
            }
          },
          {
            path: "/cms/model/design",
            name: "cms-model-design",
            component: () => import("@/views/cms/model/design.vue"),
            meta: {
              title: "cms-model-design",
              hide: true
            }
          },
          {
            path: "/cms/content",
            name: "cms-content",
            redirect: "/cms/model/list",
            meta: {
              title: "cms-content",
              icon: "icon-file"
            }
          },
          {
            path: "/cms/content/list",
            name: "cms-content-list",
            component: () => import("@/views/cms/content/list.vue"),
            meta: {
              title: "cms-content-list",
              hide: true
            }
          },
          {
            path: "/cms/content/form",
            name: "cms-content-form",
            component: () => import("@/views/cms/content/form.vue"),
            meta: {
              title: "cms-content-form",
              hide: true
            }
          },
          {
            path: "/cms/content/detail",
            name: "cms-content-detail",
            component: () => import("@/views/cms/content/detail.vue"),
            meta: {
              title: "cms-content-detail",
              hide: true
            }
          },
          {
            path: "/cms/content/recycle",
            name: "cms-content-recycle",
            component: () => import("@/views/cms/content/recycle.vue"),
            meta: {
              title: "cms-content-recycle",
              hide: true
            }
          },
          {
            path: "/cms/category",
            name: "cms-category",
            component: () => import("@/views/cms/category/list.vue"),
            meta: {
              title: "cms-category",
              icon: "icon-folder"
            }
          },
          {
            path: "/cms/tag",
            name: "cms-tag",
            component: () => import("@/views/cms/tag/list.vue"),
            meta: {
              title: "cms-tag",
              icon: "icon-tag"
            }
          },
          {
            path: "/cms/form-config",
            name: "cms-form-config",
            component: () => import("@/views/cms/form-config/list.vue"),
            meta: {
              title: "cms-form-config",
              icon: "icon-form"
            }
          },
          {
            path: "/cms/form-config/builder",
            name: "cms-form-config-builder",
            component: () => import("@/views/cms/form-config/builder.vue"),
            meta: {
              title: "cms-form-config-builder",
              hide: true
            }
          },
          {
            path: "/cms/table-config",
            name: "cms-table-config",
            component: () => import("@/views/cms/table-config/list.vue"),
            meta: {
              title: "cms-table-config",
              icon: "icon-table"
            }
          },
          {
            path: "/cms/table-config/builder",
            name: "cms-table-config-builder",
            component: () => import("@/views/cms/table-config/builder.vue"),
            meta: {
              title: "cms-table-config-builder",
              hide: true
            }
          },
          {
            path: "/cms/code-gen",
            name: "cms-code-gen",
            redirect: "/cms/code-gen/index",
            meta: {
              title: "cms-code-gen",
              icon: "icon-code"
            }
          },
          {
            path: "/cms/code-gen/index",
            name: "cms-code-gen-index",
            component: () => import("@/views/cms/code-gen/index.vue"),
            meta: {
              title: "cms-code-gen-index"
            }
          },
          {
            path: "/cms/code-gen/preview",
            name: "cms-code-gen-preview",
            component: () => import("@/views/cms/code-gen/preview.vue"),
            meta: {
              title: "cms-code-gen-preview",
              hide: true
            }
          }
        ]
      },
      {
        path: "/ecommerce",
        name: "ecommerce",
        redirect: "/ecommerce/dashboard",
        meta: {
          title: "电商平台",
          icon: "icon-shopping"
        },
        children: [
          {
            path: "/ecommerce/dashboard",
            name: "ecommerce-dashboard",
            component: () => import("@/views/ecommerce/dashboard.vue"),
            meta: {
              title: "电商仪表板",
              icon: "icon-dashboard"
            }
          },
          {
            path: "/ecommerce/platforms",
            name: "ecommerce-platforms",
            redirect: "/ecommerce/platforms/list",
            meta: {
              title: "平台管理",
              icon: "icon-earth"
            }
          },
          {
            path: "/ecommerce/platforms/list",
            name: "ecommerce-platforms-list",
            component: () => import("@/views/ecommerce/platforms/list.vue"),
            meta: {
              title: "平台列表",
              hide: true
            }
          },
          {
            path: "/ecommerce/platforms/create",
            name: "ecommerce-platforms-create",
            component: () => import("@/views/ecommerce/platforms/create.vue"),
            meta: {
              title: "新增平台",
              hide: true
            }
          },
          {
            path: "/ecommerce/platforms/edit/:id",
            name: "ecommerce-platforms-edit",
            component: () => import("@/views/ecommerce/platforms/edit.vue"),
            meta: {
              title: "编辑平台",
              hide: true
            }
          },
          {
            path: "/ecommerce/orders",
            name: "ecommerce-orders",
            component: () => import("@/views/ecommerce/orders/list.vue"),
            meta: {
              title: "订单管理",
              icon: "icon-data-queries"
            }
          },
          {
            path: "/ecommerce/products",
            name: "ecommerce-products",
            component: () => import("@/views/ecommerce/products/list.vue"),
            meta: {
              title: "商品管理",
              icon: "icon-goods"
            }
          }
        ]
      }
    ]
  }
  /**
   * 提示：写在这里的为全屏界面，不建议写在这里非全屏界面，请写在 layout.children 路由数组中
   *
   */
];

/**
 * 定义401、404、500界面
 * 401无权限
 * 404页面不存在
 * 500网络断开
 * @link 参考：https://router.vuejs.org/zh/guide/essentials/history-mode.html#netlify
 */
export const notFoundAndNoPower = [
  {
    path: "/401",
    name: "no-access",
    component: () => import("@/views/error/401.vue"),
    meta: {
      title: "no-access",
      hide: true
    }
  },
  {
    path: "/500",
    name: "no-network",
    component: () => import("@/views/error/500.vue"),
    meta: {
      title: "no-network",
      hide: true
    }
  },
  {
    path: "/:path(.*)*",
    name: "not-found",
    component: () => import("@/views/error/404.vue"),
    meta: {
      title: "not-found",
      hide: true
    }
  }
];
