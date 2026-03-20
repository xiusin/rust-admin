# 注解式路由系统 - 验证清单

## 核心模块验证

- [ ] `src/core/` 目录结构正确
- [ ] `src/core/route/mod.rs` 模块声明正确
- [ ] `src/core/macros.rs` 宏定义正确

## 注解定义验证

- [ ] `#[route]` 注解可正确标记 Handler
- [ ] `#[auth]` 注解可正确配置鉴权规则
- [ ] `#[log]` 注解可正确配置日志记录
- [ ] `#[rate_limit]` 注解可正确配置限流规则

## 路由收集验证

- [ ] RouteCollector 可收集所有带注解的 Handler
- [ ] Handler 元信息（路径、方法、注解）正确提取
- [ ] 路由元数据存储结构正确

## 路由注册验证

- [ ] RouteRegistry 正确实现 Axum Router 集成
- [ ] 路由路径参数正确解析（如 `/user/{id}`）
- [ ] 中间件按注解正确应用

## 鉴权验证

- [ ] `#[auth(required = true)]` 正确拦截未登录请求
- [ ] `#[auth(roles = ["admin"])]` 正确检查角色
- [ ] `#[auth(permissions = ["user:delete"])]` 正确检查权限

## 日志验证

- [ ] `#[log(operation = "xxx")]` 正确记录操作日志
- [ ] 日志包含正确的操作人、时间、结果信息

## 示例验证

- [ ] UserHandler 示例编译通过
- [ ] 路由正确注册到路由器
- [ ] 中间件按预期执行

## 编译验证

- [ ] cargo build 编译通过
- [ ] 无未解析的类型或模块
- [ ] 无循环依赖

## 文档验证

- [ ] 注解使用文档完整
- [ ] AGENTS.md 已更新