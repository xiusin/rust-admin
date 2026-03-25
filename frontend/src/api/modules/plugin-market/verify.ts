export const verify = {
  sendCode: (data: {
    licenseId: number;
    pluginId: number;
    purpose: number;
    deviceHash?: string;
  }) => axios.post('/plugin/verify/code/send', data),
  checkCode: (data: {
    licenseId: number;
    code: string;
    deviceHash?: string;
  }) => axios.post('/plugin/verify/code/check', data),
  obfuscationConfig: (pluginId: number) =>
    axios.get('/plugin/verify/obfuscation/config', { params: { pluginId } }),
};