# 任务列表 (Tasks)

- [x] Task 1: 移除 Mock 依赖与鉴权机制重构
  - [x] SubTask 1.1: 移除 `frontend/src/api/modules/user/index.ts` 中的 Mock 兜底逻辑，确保登录和用户信息接口仅请求真实后端。
  - [x] SubTask 1.2: 清理 `frontend/src/mock/` 目录并移除 Vite 配置中的 `vite-plugin-mock` 插件。
  - [x] SubTask 1.3: 改造 `frontend/src/api/index.ts` 的 Axios 拦截器，实现基于 Refresh Token 的 401 静默无感刷新队列机制。

- [x] Task 2: 剩余模块 API 全量对接 (不打桩)
  - [x] SubTask 2.1: 将 `frontend/src/api/modules/monitor/index.ts` 中的在线用户、定时任务监控等接口对接到真实后端的 `api/monitor/*` 路由。
  - [x] SubTask 2.2: 将文件管理 (`file/index.ts`) 和通用表格 (`table/index.ts`) 演示页面切换至真实的后端接口，并确保前后端字段对齐。

- [x] Task 3: 前端渲染性能深度优化
  - [x] SubTask 3.1: 在 `DynamicTable` 和数据量较大的业务表格中，引入并适配 `virtual-list` 虚拟滚动组件，优化长列表渲染性能。
  - [x] SubTask 3.2: 改造 CMS 模块的 `DynamicForm` 与 `ModelDesigner`，对表单子组件实施异步懒加载 (Async Component)。
  - [x] SubTask 3.3: 结合 Pinia 和 `sessionStorage`，为全局字典数据请求 (`dict.ts`) 添加缓存与防抖机制，避免组件重复挂载导致多次请求。

- [x] Task 4: 后端性能优化与并发控制
  - [x] SubTask 4.1: 分析并优化 `src/application/cms/` 和 `src/application/product/` 中的 ORM 查询，通过批处理或缓存机制解决动态模型数据加载的 N+1 查询陷阱。
  - [x] SubTask 4.2: 优化 `src/worker/` 下电商平台等外部调用的并发模型，加入指数退避 (Exponential Backoff) 重试机制和熔断保护。

- [x] Task 5: 前端页面交互体验与内容丰富化
  - [x] SubTask 5.1: 强化 `ModelDesigner` (表单/模型设计器) 的拖拽微交互，添加拖拽放置热区高亮、明确的占位符及平滑过渡动画。
  - [x] SubTask 5.2: 为 `code-gen` 模块的代码生成与预览过程，设计并实现终端打字机风格的加载动画或骨架屏。
  - [x] SubTask 5.3: 统一全局按钮悬停、页面切换过渡路由动画、全局 Loading 状态，确保符合项目整体的高级感与统一风格。

# 任务依赖 (Task Dependencies)
- [Task 2] 依赖 [Task 1] (必须先清理 Mock 环境并确保鉴权可靠)
- [Task 3] 与 [Task 4] 可并行执行 (前后端性能优化相互独立)
- [Task 5] 依赖 [Task 3] (在性能保障的前提下增加交互细节)
