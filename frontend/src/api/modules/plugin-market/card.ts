export const card = {
  generate: (data: { pluginId: number; planId: number; count: number; price: number; expireDays: number }) =>
    axios.post("/plugin/card/generate", data),
  batchList: (params?: { pluginId?: number; pageNum?: number; pageSize?: number }) =>
    axios.get("/plugin/card/batch/list", { params }),
  export: (batchId: number) =>
    axios.get(`/plugin/card/export/${batchId}`, {
      responseType: "blob"
    }),
  preview: (data: { cardNo: string; cardPwd: string }) => axios.post("/plugin/card/preview", data),
  redeem: (data: { cardNo: string; cardPwd: string }) => axios.post("/plugin/card/redeem", data),
  freeze: (data: { cardId: number }) => axios.post("/plugin/card/freeze", data),
  unfreeze: (data: { cardId: number }) => axios.post("/plugin/card/unfreeze", data)
};
