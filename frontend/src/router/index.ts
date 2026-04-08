import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/About.vue'),
    children: [
      {
        path: 'team',
        name: 'Team',
        component: () => import('../views/About.vue')
      }
    ]
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/users',
    name: 'UserList',
    component: () => import('../views/UserList.vue')
  },
  {
    path: '/depts',
    name: 'DeptList',
    component: () => import('../views/DeptList.vue')
  },
  {
    path: '/roles',
    name: 'RoleList',
    component: () => import('../views/RoleList.vue')
  },
  {
    path: '/api-permissions',
    name: 'ApiPermissionList',
    component: () => import('../views/ApiPermissionList.vue')
  },
  {
    path: '/permission-categories',
    name: 'PermissionCategoryList',
    component: () => import('../views/PermissionCategoryList.vue')
  },
  {
    path: '/menus',
    name: 'MenuList',
    component: () => import('../views/MenuList.vue')
  },
  {
    path: '/login-logs',
    name: 'LoginLog',
    component: () => import('../views/LoginLog.vue')
  },
  {
    path: '/operation-logs',
    name: 'OperationLog',
    component: () => import('../views/OperationLog.vue')
  },
  {
    path: '/server-info',
    name: 'ServerInfo',
    component: () => import('../views/ServerInfo.vue')
  },
  {
    path: '/jobs',
    name: 'JobList',
    component: () => import('../views/JobList.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router