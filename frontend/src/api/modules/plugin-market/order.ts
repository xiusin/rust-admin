export const cart = {
  list: (userId?: number) => axios.get("/plugin/cart/list", { params: { userId } }),
  add: (data: { pluginId: number; planId: number }) => axios.post("/plugin/cart/add", data),
  remove: (data: { ids: number[] }) => axios.delete("/plugin/cart/remove", { data }),
  clear: (userId?: number) => axios.delete("/plugin/cart/clear", { params: { userId } })
};

export const order = {
  create: (data: { pluginId: number; planId: number; couponId?: number; paymentMethod: number }) =>
    axios.post("/plugin/order/create", data),
  list: (params?: any) => axios.get("/plugin/order/list", { params }),
  detail: (id: number) => axios.get(`/plugin/order/detail/${id}`),
  cancel: (id: number) => axios.post(`/plugin/order/cancel/${id}`),
  pay: (id: number) => axios.post(`/plugin/order/pay/${id}`),
  payCallback: (data: { orderId: number; paymentMethod: number }) => axios.post("/plugin/order/payCallback", data)
};
