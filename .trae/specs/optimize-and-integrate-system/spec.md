# 系统深度优化与全量 API 对接 (Optimize and Integrate System) Spec

## Why
目前项目核心业务虽然已接入真实后端，但仍存在部分模块（如监控、文件、表格演示等）依赖 Mock 数据，且登录存在不安全的 Mock 兜底机制。此外，前端在动态表单/表格的大规模数据渲染时存在性能瓶颈，交互缺乏细节反馈；后端在复杂动态数据查询中存在 N+1 性能陷阱，后台 Worker 任务缺乏针对第三方平台的并发控制。为了交付一个高性能、逻辑自洽、交互丰富且完全脱离 Mock（不打桩）的生产级应用，必须进行深度优化与全量联调。

## What Changes
- **移除所有 Mock 依赖并全量对接 API**：移除 `vite-plugin-mock` 相关逻辑，清理遗留 Mock 文件；重构登录与用户信息接口，移除失败时自动 fallback 到 Mock 的机制。
- **引入双 Token 机制**：实现 Access Token + Refresh Token 无感刷新机制，提升鉴权体验与安全性。
- **前端性能优化**：对 CMS 模块中的 `DynamicForm`、`ModelDesigner` 实施组件懒加载；在 `DynamicTable` 和数据量大的表格中引入虚拟滚动（Virtual List）；对字典配置数据引入 Pinia + sessionStorage 缓存与防抖机制。
- **前端交互与 UI 丰富化**：强化 CMS 设计器（拖拽高亮、撤销/重做提示等）微交互；为代码生成器（`code-gen`）及耗时操作添加终端风格打字机动画或骨架屏；统一全局状态切换的过渡动画，确保符合项目统一设计风格。
- **后端性能优化与并发控制**：优化 `cms` 和 `product` 模块的 ORM 查询，消除动态属性查询中的 N+1 陷阱；为 `worker` 模块中的电商平台同步任务增加指数退避（Exponential Backoff）重试与连接池并发控制。
- **BREAKING**: 完全移除 `/mock/*` 路由前缀和对应的拦截机制，所有接口强制走 `/api/*`。

## Impact
- Affected specs: 认证鉴权体系、CMS/Product 模型渲染性能、全局网络请求与拦截器、第三方开放平台 Worker 任务调度。
- Affected code: 
  - 前端：`frontend/src/api/*`, `frontend/src/mock/*`, `frontend/src/components/*`, `frontend/src/store/*`
  - 后端：`src/application/cms/*`, `src/application/product/*`, `src/worker/*`

## ADDED Requirements
### Requirement: 双 Token 无感刷新
系统应当在 Access Token 过期时（如 401 状态码），自动挂起当前请求队列，使用 Refresh Token 获取新 Token 后无缝重放请求。

#### Scenario: 令牌过期刷新成功
- **WHEN** 用户在页面操作触发 API 请求，且后端返回 401 Unauthorized
- **THEN** 前端拦截器暂停抛出错误，静默调用 `/api/sys/auth/refresh` 换取新 Token，并使用新 Token 重试原请求，用户无感知。

### Requirement: 拖拽微交互与加载状态增强
系统应当在进行复杂模型设计和代码生成时，提供明确的视觉反馈与中间状态。

#### Scenario: 耗时生成操作
- **WHEN** 用户点击“生成代码”按钮
- **THEN** 界面展示带有代码生成意象的骨架屏或打字机加载动画，直至后端真实返回文件流。

## MODIFIED Requirements
### Requirement: 登录与接口调用纯净度
完全剥离前端开发与 Mock 数据的耦合。登录、用户信息获取、系统监控等所有接口强制请求后端真实服务。

**修改点**: 删除 `loginAPI` 中的 `try-catch` Mock 兜底代码，若后端服务异常，直接通过 UI 组件向用户抛出真实的网络错误（如“网络超时”或“服务器无响应”）。

## REMOVED Requirements
### Requirement: Mock 数据拦截
**Reason**: 系统即将进入真实生产和全量联调阶段，不打桩是核心需求。
**Migration**: 废弃并删除 `frontend/src/mock/` 目录及其在 Vite 中的配置。
