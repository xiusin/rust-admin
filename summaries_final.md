| 文件 | 变更 |
|------|---------|
| .trae/specs/optimize-and-integrate-system/checklist.md | - 新增了项目重构与集成的检查清单。 |
| .trae/specs/optimize-and-integrate-system/spec.md | - 新增了项目重构与集成的需求说明文档。 |
| .trae/specs/optimize-and-integrate-system/tasks.md | - 新增了项目重构与集成的任务列表。 |
| frontend/build/vite-plugin.ts | - 移除了 `vite-plugin-mock` 插件配置。 |
| frontend/package.json | - 移除了 `mockjs` 及相关依赖。 |
| frontend/pnpm-lock.yaml | - 移除了 `mockjs` 及相关依赖。 |
| frontend/src/App.vue | - 引入了 WebSocket 初始化逻辑。 |
| frontend/src/api/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/api/modules/consumer/freight.ts | - 更新了后端接口请求地址或类型定义。 |
| frontend/src/api/modules/file/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/api/modules/monitor/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/api/modules/plugin-market/card.ts | - 更新了后端接口请求地址或类型定义。 |
| frontend/src/api/modules/plugin-market/market.ts | - 更新了后端接口请求地址或类型定义。 |
| frontend/src/api/modules/table/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/api/modules/test/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/api/modules/user/index.ts | - 更新了 axios 请求拦截器和响应处理逻辑。<br>- 移除了 mock 相关的 API 引入。 |
| frontend/src/components/cms/code-gen/CodeViewer.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/dynamic-form/DynamicForm.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/dynamic-form/fields/FileField.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/dynamic-form/fields/ImageField.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/dynamic-table/DynamicTable.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/model-designer/FieldList.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/components/cms/model-designer/ModelDesigner.vue | - 优化了组件内部状态管理或方法调用。 |
| frontend/src/hooks/useWebSocket.ts | - 新增了 `useWebSocket` Hook 用于封装 WebSocket 实时通信逻辑。 |
| frontend/src/mock/_data/monitor_data.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/_data/system_data.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/_data/system_menu.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/_utils.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/category.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/codeGen.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/content.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/form.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/model.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/table.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/cms/tag.mock.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/file/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/monitor/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/plugin-market/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/system/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/system/menu.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/system/system.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/table/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/test/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/mock/user/index.ts | - 删除了前端 Mock 数据文件。 |
| frontend/src/store/config/index.ts | - 更新了状态管理中的配置或数据结构。 |
| frontend/src/store/modules/system.ts | - 更新了状态管理中的配置或数据结构。 |
| frontend/src/style/model/global-style.scss | - 增加了全局样式或过渡动画。 |
| frontend/src/style/model/global-transition.scss | - 增加了全局样式或过渡动画。 |
| frontend/src/views/cms/code-gen/index.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/cms/code-gen/preview.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/consumer/freight.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/monitor/onlineuser/index.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-developer/sales.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-market/detail.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-market/index.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-market/list.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-market/search.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-my/license.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-my/purchased.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/plugin-my/subscription.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/system/account/account.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/system/division/division.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/table/common-table/common-table.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/table/custom-table/custom-table.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| frontend/src/views/verify/card-redeem.vue | - 移除了页面组件中的 mock 数据依赖。<br>- 接入了真实的后端 API 接口。 |
| patch_developer_service.sh | - 新增了 `patch_developer_service.sh` 修复脚本。 |
| src/api.rs | - 新增了文件。 |
| src/api/consumer/freight.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/monitor/mod.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/monitor/ws.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/plugin_market/plugin_market_handler.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/plugin_market/router.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/sys/mod.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/api/sys/upload.rs | - 更新了 Rust 后端的路由注册与处理器逻辑。 |
| src/application/cms/cms_content_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/application/consumer/freight_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/application/plugin_market/card_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/application/plugin_market/developer_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/application/product/product_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/application/sys/upload_service.rs | - 完善了应用层的业务逻辑实现。<br>- 增加了数据库查询或数据处理逻辑。 |
| src/domain/args/a_freight.rs | - 更新了领域模型或请求参数结构。 |
| src/domain/model/m_freight.rs | - 更新了领域模型或请求参数结构。 |
| src/worker/requesturl.rs | - 增强了 worker 中的请求处理逻辑。 |
| test-expand.vue | - 新增了文件。 |
