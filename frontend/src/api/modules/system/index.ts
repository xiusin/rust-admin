import axios from "@/api";

// 获取菜单数据
export const getRoutersAPI = () => {
  return axios({
    url: "/sys/menu/getRouters",
    method: "get"
  });
};

// 获取字典数据
export const getDictAPI = () => {
  return axios({
    url: "/sys/dicttype/list",
    method: "get"
  });
};

// 获取部门数据
export const getDivisionAPI = (params?: { dept_name?: string; status?: string }) => {
  return axios({
    url: "/sys/dept/tree",
    method: "get",
    params
  });
};

// 添加部门
export const addDivisionAPI = (data: any) => {
  return axios({
    url: "/sys/dept/add",
    method: "post",
    data
  });
};

// 编辑部门
export const editDivisionAPI = (data: any) => {
  return axios({
    url: "/sys/dept/edit",
    method: "put",
    data
  });
};

// 删除部门
export const deleteDivisionAPI = (params: { dept_id: string }) => {
  return axios({
    url: "/sys/dept/del",
    method: "delete",
    params
  });
};

// 获取角色数据
export const getRoleAPI = (params?: {
  role_name?: string;
  role_key?: string;
  status?: string;
  start_time?: string;
  end_time?: string;
}) => {
  return axios({
    url: "/sys/role/list",
    method: "get",
    params
  });
};

// 获取账户数据
export const getAccountAPI = (params?: {
  dept_id?: number;
  user_name?: string;
  phonenumber?: string;
  status?: string;
  start_time?: string;
  end_time?: string;
}) => {
  return axios({
    url: "/sys/user/list",
    method: "get",
    params
  });
};

// 获取菜单管理列表
export const getMenuListAPI = (params?: { title?: string; hidden?: string; status?: string }) => {
  return axios({
    url: "/sys/menu/getMenuList",
    method: "get",
    params
  });
};

// 添加菜单
export const addMenuAPI = (data: any) => {
  return axios({
    url: "/sys/menu/add",
    method: "post",
    data
  });
};

// 编辑菜单
export const editMenuAPI = (data: any) => {
  return axios({
    url: "/sys/menu/edit",
    method: "put",
    data
  });
};

// 删除菜单
export const deleteMenuAPI = (data: any) => {
  return axios({
    url: "/sys/menu/del",
    method: "delete",
    data
  });
};

// 根据角色获取权限数据
export const getUserPermissionAPI = (params: { role: string }) => {
  return axios({
    url: "/sys/menu/getUserPermission",
    method: "get",
    params
  });
};

// 登录 - 真实后端
export const loginAPI = (data: any) => {
  return axios({
    url: "/sys/auth/login",
    method: "post",
    data
  });
};

// 获取用户信息 - 真实后端
export const getUserInfoAPI = () => {
  return axios({
    url: "/sys/user/getUserInfo",
    method: "get"
  });
};

// 添加角色
export const addRoleAPI = (data: any) => {
  return axios({
    url: "/sys/role/add",
    method: "post",
    data
  });
};

// 编辑角色
export const editRoleAPI = (data: any) => {
  return axios({
    url: "/sys/role/edit",
    method: "put",
    data
  });
};

// 删除角色
export const deleteRoleAPI = (data: any) => {
  return axios({
    url: "/sys/role/del",
    method: "delete",
    data
  });
};

// 分配菜单权限
export const assignMenuAPI = (data: { role_id: number; menu_ids: number[] }) => {
  return axios({
    url: "/sys/role/menu",
    method: "post",
    data: {
      role_id: data.role_id,
      menu_ids: data.menu_ids.map(id => Number(id))
    }
  });
};
