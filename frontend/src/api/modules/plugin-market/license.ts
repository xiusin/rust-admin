export const license = {
  list: (params?: { pageNum?: number; pageSize?: number }) => axios.get("/plugin/license/list", { params }),
  detail: (id: number) => axios.get(`/plugin/license/detail/${id}`),
  bind: (data: {
    licenseId: number;
    deviceId: string;
    deviceName?: string;
    deviceType?: string;
    osVersion?: string;
    appVersion?: string;
    macAddress?: string;
    ipAddress?: string;
  }) => axios.post("/plugin/license/bind", data),
  unbind: (data: { licenseId: number; deviceId: string }) => axios.post("/plugin/license/unbind", data),
  renew: (data: { licenseId: number; extendDays: number }) => axios.post("/plugin/license/renew", data),
  revoke: (data: { licenseId: number }) => axios.post("/plugin/license/revoke", data),
  verify: (data: {
    licenseKey: string;
    deviceId: string;
    deviceInfo: {
      deviceName: string;
      deviceType: string;
      osVersion: string;
      appVersion: string;
      macAddress?: string;
    };
    timestamp: number;
    sign: string;
  }) => axios.post("/plugin/license/verify", data),
  heartbeat: (data: { licenseKey: string; deviceId: string; timestamp: number; sign: string }) =>
    axios.post("/plugin/license/heartbeat", data)
};

export const subscription = {
  list: (params?: { pageNum?: number; pageSize?: number }) => axios.get("/plugin/subscription/list", { params }),
  renew: (data: { subscriptionId: number; extendDays: number }) => axios.post("/plugin/subscription/renew", data),
  cancel: (id: number) => axios.post(`/plugin/subscription/cancel/${id}`)
};

export const device = {
  register: (data: {
    licenseId: number;
    deviceId: string;
    deviceInfo: {
      deviceName?: string;
      deviceType?: string;
      osVersion?: string;
      macAddress?: string;
    };
    ipAddress?: string;
  }) => axios.post("/plugin/verify/device/register", data)
};
