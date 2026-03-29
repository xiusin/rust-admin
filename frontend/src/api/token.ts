const TOKEN_KEY = "x-token";

export const getToken = () => {
  return localStorage.getItem(TOKEN_KEY) || "";
};

export const setToken = (token: string) => {
  localStorage.setItem(TOKEN_KEY, token);
};

export const removeToken = () => {
  localStorage.removeItem(TOKEN_KEY);
};

export const getUserInfo = () => {
  const info = localStorage.getItem("user-info");
  return info ? JSON.parse(info) : null;
};

export const setUserInfo = (info: any) => {
  localStorage.setItem("user-info", JSON.stringify(info));
};

export const removeUserInfo = () => {
  localStorage.removeItem("user-info");
};
