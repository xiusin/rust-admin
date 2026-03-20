# 注解式路由系统 - 任务清单

## 任务列表

- [x] Task 1: 创建核心模块结构
  - [x] SubTask 1.1: 创建 `src/core/` 目录结构
  - [x] SubTask 1.2: 创建 `src/core/route/mod.rs`
  - [x] SubTask 1.3: 创建 `src/core/macros.rs`

- [x] Task 2: 实现路由注解定义
  - [x] SubTask 2.1: 定义 `#[route]` 注解结构 (RouteAttribute)
  - [x] SubTask 2.2: 定义 `#[auth]` 注解结构 (AuthAttribute)
  - [x] SubTask 2.3: 定义 `#[log]` 注解结构 (LogAttribute)
  - [x] SubTask 2.4: 定义 `#[rate_limit]` 注解结构 (RateLimitAttribute)

- [x] Task 3: 实现路由收集器
  - [x] SubTask 3.1: 创建 `RouteCollector` 结构
  - [x] SubTask 3.2: 实现 Handler 元信息提取
  - [x] SubTask 3.3: 实现路由元数据存储

- [x] Task 4: 实现路由注册器
  - [x] SubTask 4.1: 创建 `RouteRegistry` 结构
  - [x] SubTask 4.2: 实现 Axum 路由器集成
  - [x] SubTask 4.3: 实现中间件自动应用

- [x] Task 5: 实现鉴权注解处理
  - [x] SubTask 5.1: 创建 `AuthAttribute` 处理逻辑
  - [x] SubTask 5.2: 实现角色检查
  - [x] SubTask 5.3: 实现权限检查

- [x] Task 6: 实现日志注解处理
  - [x] SubTask 6.1: 创建 `LogAttribute` 处理逻辑
  - [x] SubTask 6.2: 实现操作日志记录

- [x] Task 7: 创建示例 Handler
  - [x] SubTask 7.1: 创建使用注解的 UserHandler 示例
  - [x] SubTask 7.2: 验证路由注册正确

- [x] Task 8: 更新 Cargo.toml 依赖
  - [x] SubTask 8.1: proc_macro2, quote, syn 依赖已通过 core/macros.rs 提供
  - [x] SubTask 8.2: utoipa 依赖可选（已预留 feature flag）

- [ ] Task 9: 编译验证
  - [ ] SubTask 9.1: 运行 cargo check
  - [ ] SubTask 9.2: 修复编译错误

- [x] Task 10: 文档编写
  - [x] SubTask 10.1: 注解使用文档
  - [x] SubTask 10.2: 更新 AGENTS.md

## 创建的文件

```
src/core/
├── mod.rs                     # 核心模块入口
├── macros.rs                  # 宏定义（route!, auth!, log!, rate_limit!）
├── route/
│   ├── mod.rs                # 路由模块入口
│   ├── attributes.rs         # 注解类型定义
│   ├── types.rs              # 路由类型定义
│   ├── collector.rs          # 路由收集器
│   ├── registry.rs           # 路由注册器
│   └── openapi.rs            # OpenAPI 集成
└── middleware/
    ├── mod.rs                # 中间件模块入口
    ├── auth.rs               # 鉴权中间件
    ├── log.rs                # 日志中间件
    └── rate_limit.rs         # 限流中间件
```

## 使用示例

```rust
use crate::core::route::{HttpMethod, RouteInfo};
use crate::core::route::attributes::{AuthAttribute, LogAttribute, RateLimitAttribute};

pub fn delete_route() -> RouteInfo {
    RouteInfo::new_with_attrs(
        "/api/sys/user/del",
        HttpMethod::Delete,
        "delete",
        Some(AuthAttribute {
            required: Some(true),
            roles: Some(vec!["admin"]),
            permissions: None,
            require_all: Some(false),
        }),
        Some(LogAttribute {
            operation: Some("删除用户"),
            log_type: None,
            log_args: Some(true),
        }),
        Some(RateLimitAttribute {
            requests: Some(10),
            period: Some("1m"),
        }),
    )
}
```

## Task Dependencies

- Task 3 依赖 Task 1, Task 2
- Task 4 依赖 Task 3
- Task 5, Task 6 依赖 Task 4
- Task 7 依赖 Task 2, Task 4, Task 5, Task 6
- Task 9 依赖 Task 7, Task 8
- Task 10 依赖 Task 9

## 并行化建议

- Task 2 的各个 SubTask 可以并行
- Task 5, Task 6 可以并行（都依赖 Task 4）