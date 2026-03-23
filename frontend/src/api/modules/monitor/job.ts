import axios from "@/api";

// ==================== 定时任务 API ====================

// 获取定时任务列表
export const getJobListAPI = (params?: {
  job_name?: string;
  job_group?: string;
  status?: string;
  page_num?: number;
  page_size?: number;
}) => {
  return axios({
    url: "/sys/job/list",
    method: "get",
    params
  });
};

// 添加定时任务
export const addJobAPI = (data: {
  job_name: string;
  job_group: string;
  task_type: string;
  task_count: number;
  run_count: number;
  cron_expression: string;
  job_params?: string;
  status: string;
  remark: string;
}) => {
  return axios({
    url: "/sys/job/add",
    method: "post",
    data
  });
};

// 编辑定时任务
export const editJobAPI = (data: {
  job_id: string;
  job_name: string;
  job_group: string;
  task_type: string;
  task_count: number;
  run_count: number;
  cron_expression: string;
  job_params?: string;
  status: string;
  remark: string;
}) => {
  return axios({
    url: "/sys/job/edit",
    method: "put",
    data
  });
};

// 删除定时任务
export const deleteJobAPI = (params: { job_id: string }) => {
  return axios({
    url: "/sys/job/del",
    method: "delete",
    params
  });
};

// 验证cron表达式
export const validateCronAPI = (data: { cron_expression: string }) => {
  return axios({
    url: "/sys/job/validate_cron",
    method: "post",
    data
  });
};

// 手动执行定时任务
export const handExecuteJobAPI = (data: { job_id: string }) => {
  return axios({
    url: "/sys/job/hand_execute_job",
    method: "post",
    data
  });
};

// ==================== 定时任务日志 API ====================

// 获取定时任务日志列表
export const getJobLogListAPI = (params?: {
  job_id?: string;
  page_num?: number;
  page_size?: number;
}) => {
  return axios({
    url: "/sys/jobinfo/list",
    method: "get",
    params
  });
};
