export const market = {
  list: (params?: any) => axios.get('/plugin/market/list', { params }),
  detail: (id: number) => axios.get(`/plugin/market/detail/${id}`),
  search: (params?: any) => axios.get('/plugin/market/search', { params }),
  recommend: (limit?: number) => axios.get('/plugin/market/recommend', { params: { limit } }),
  hot: (limit?: number) => axios.get('/plugin/market/hot', { params: { limit } }),
  categories: () => axios.get('/plugin/market/categories'),
};

export const category = {
  list: () => axios.get('/plugin/category/list'),
  tree: () => axios.get('/plugin/category/tree'),
  detail: (id: number) => axios.get(`/plugin/category/${id}`),
  add: (data: any) => axios.post('/plugin/category/add', data),
  edit: (data: any) => axios.put('/plugin/category/edit', data),
  delete: (id: number) => axios.delete(`/plugin/category/${id}`),
};

export const developer = {
  list: (params?: any) => axios.get('/plugin/developer/list', { params }),
  add: (data: any) => axios.post('/plugin/developer/add', data),
  edit: (data: any) => axios.put('/plugin/developer/edit', data),
  delete: (id: number) => axios.delete(`/plugin/developer/delete/${id}`),
  audit: (data: any) => axios.put('/plugin/developer/audit', data),
  versionAdd: (data: any) => axios.post('/plugin/developer/version/add', data),
  stats: () => axios.get('/plugin/developer/stats'),
  register: (data: any) => axios.post('/plugin/developer/register', data),
  profile: () => axios.get('/plugin/developer/profile'),
  update: (data: any) => axios.put('/plugin/developer/update', data),
};

export const version = {
  list: (pluginId: number) => axios.get(`/plugin/version/list/${pluginId}`),
  detail: (id: number) => axios.get(`/plugin/version/detail/${id}`),
  latest: (pluginId: number) => axios.get(`/plugin/version/latest/${pluginId}`),
};

export const plan = {
  list: (pluginId: number) => axios.get(`/plugin/plan/list/${pluginId}`),
  detail: (id: number) => axios.get(`/plugin/plan/detail/${id}`),
  add: (data: any) => axios.post('/plugin/plan/add', data),
  edit: (data: any) => axios.put('/plugin/plan/edit', data),
  delete: (id: number) => axios.delete(`/plugin/plan/delete/${id}`),
};

export const review = {
  list: (pluginId: number, params?: any) => axios.get(`/plugin/review/list/${pluginId}`, { params }),
  create: (data: any) => axios.post('/plugin/review/create', data),
  reply: (data: any) => axios.post('/plugin/review/reply', data),
  stats: (pluginId: number) => axios.get(`/plugin/review/stats/${pluginId}`),
};