# 验收清单 (Checklist)

- [x] 移除鉴权中的 Mock 兜底逻辑，登录与用户信息 100% 请求真实后端
- [x] 完全卸载 `vite-plugin-mock` 插件并清理了 `frontend/src/mock/` 目录
- [x] 成功实现了 Axios 拦截器中的 Access/Refresh Token 双令牌无感静默刷新机制
- [x] `frontend/src/api/modules/monitor/` 中的所有监控接口（在线用户、系统任务等）成功对接真实后端
- [x] `file` 和 `table` 演示页面不再请求 `/mock/*`，且前后端数据结构联调通过
- [x] `DynamicTable` 与长列表成功应用了虚拟滚动组件，滚动无明显卡顿
- [x] `DynamicForm` 与 `ModelDesigner` 等庞大组件成功实现异步按需懒加载
- [x] 字典与配置数据已在 Pinia 和 `sessionStorage` 层面实现持久化缓存与防抖
- [x] 后端 `cms` 和 `product` 模块的动态模型数据获取已优化，不再出现 N+1 的慢查询日志
- [x] 后端 `worker` 电商同步任务实现了指数退避重试和外部调用的并发限流保护
- [x] CMS 模型设计器 (`ModelDesigner`) 拖拽时具有清晰的高亮、热区占位符等微交互反馈
- [x] 代码生成器 (`code-gen`) 在点击生成时，具有明确的终端打字机风格加载动效或骨架屏
- [x] 全局按钮悬停、路由切换过渡动画、Loading 状态样式风格高度统一，操作流畅自洽
