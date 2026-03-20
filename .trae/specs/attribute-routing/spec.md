# 注解式路由注册系统规范

## Why

当前路由注册方式存在以下问题：

1. **手动注册繁琐** - 每个路由都需要在 `api/sys/*.rs` 中手动编写 `.route()` 调用
2. **中间件配置分散** - 鉴权中间件在 `app.rs` 中统一配置，无法精细化控制
3. **路由与业务逻辑耦合** - 路由路径字符串散落在各处
4. **缺乏统一规范** - 不同模块的路由注册方式可能不一致

## What Changes

### 核心设计

引入 **注解（Attribute）驱动的路由注册**，通过在 Handler 函数上添加注解来实现：

```rust
#[utoipa::path(
    get,
    path = "/api/sys/user/list",
    params(PathParams),  // OpenAPI 参数
    responses(
        (status = 200, description = "用户列表", body = ApiResponse<ListData<User>>)
    ),
    tag = "sys_user"
)]
#[auth(required = true, roles = ["admin"])]      // 鉴权注解
#[log(operation = "查询用户列表")]              // 操作日志注解
pub async fn list(/* ... */) -> impl IntoResponse { /* ... */ }
```

### 新增模块

```
src/
├── core/                                  # 核心模块（新增）
│   ├── route/                             # 路由注册系统
│   │   ├── mod.rs
│   │   ├── attributes.rs                 # 路由注解定义
│   │   ├── collector.rs                  # 路由收集器
│   │   ├── registry.rs                   # 路由注册器
│   │   └── openapi.rs                    # OpenAPI 集成
│   ├── middleware/                        # 注解中间件
│   │   ├── mod.rs
│   │   ├── auth.rs                       # 鉴权注解处理
│   │   ├── log.rs                        # 日志注解处理
│   │   └── rate_limit.rs                 # 限流注解处理
│   └── macros.rs                         # 过程宏定义
```

### 注解类型

#### 1. 路由注解 `#[route]`

```rust
#[route(method = "GET", path = "/user/{id}", tag = "user")]
// method: GET | POST | PUT | DELETE
// path: 路由路径，支持路径参数 {id}
// tag: OpenAPI 标签分组
```

#### 2. 鉴权注解 `#[auth]`

```rust
#[auth(required = true)]                    // 必须登录
#[auth(required = false)]                   // 可选登录
#[auth(roles = ["admin", "user"])]          // 指定角色
#[auth(permissions = ["user:read"])]        // 指定权限
```

#### 3. 日志注解 `#[log]`

```rust
#[log(operation = "创建用户")]               // 操作名称
#[log(operation = "删除用户", log_args = true)]  // 是否记录参数
```

#### 4. 限流注解 `#[rate_limit]`

```rust
#[rate_limit(requests = 100, period = "1m")]   // 100次/分钟
#[rate_limit(requests = 10, period = "1s")]    // 10次/秒
```

### 路由收集与注册流程

```
1. 编译时：过程宏扫描所有 handler 函数，收集注解元数据
                      ↓
2. 生成：生成路由注册代码和 OpenAPI 规范
                      ↓
3. 运行前：路由注册器收集所有路由信息
                      ↓
4. 运行：Axum 路由器根据元数据自动注册
```

### Handler 函数签名规范

```rust
// 标准 Handler
pub async fn list(
    Query(params): Query<ListParams>,
    Path(id): Path<i64>,
    Json(body): Json<Request>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse, AppError>

// 带用户信息的 Handler（自动注入）
pub async fn list(
    Query(params): Query<ListParams>,
    Auth(user): AuthenticatedUser,        // 自动从请求中提取用户
) -> Result<impl IntoResponse, AppError>
```

### 中间件执行顺序

```
请求 → [限流] → [鉴权] → [日志] → [业务逻辑] → [日志响应] → 响应
```

## Impact

- **受影响代码**: API 路由层需要迁移到新注解方式
- **依赖变更**: 需要引入 `utoipa` 用于 OpenAPI，proc_macro2, quote 等用于过程宏
- **学习成本**: 开发人员需要了解新注解语法

## ADDED Requirements

### Requirement: 路由注解系统

系统 SHALL 提供基于注解的路由注册机制，允许开发者通过函数注解声明路由信息。

#### Scenario: 基本路由注册
- **WHEN** 开发者为 Handler 函数添加 `#[route(method = "GET", path = "/user/list")]`
- **THEN** 系统自动注册 `GET /user/list` 路由

#### Scenario: 路由分组
- **WHEN** 多个路由属于同一 tag（如 "user"）
- **THEN** 系统自动在 OpenAPI 文档中分组展示

### Requirement: 鉴权注解

系统 SHALL 提供基于注解的精细化鉴权机制。

#### Scenario: 角色鉴权
- **WHEN** Handler 添加 `#[auth(roles = ["admin"])]`
- **THEN** 只有 admin 角色可以访问此接口

#### Scenario: 权限鉴权
- **WHEN** Handler 添加 `#[auth(permissions = ["user:delete"])]`
- **THEN** 只有拥有 user:delete 权限的用户可以访问

### Requirement: 操作日志注解

系统 SHALL 提供基于注解的操作日志记录。

#### Scenario: 自动日志记录
- **WHEN** Handler 添加 `#[log(operation = "删除用户")]`
- **THEN** 系统自动记录操作日志，包含操作人、时间、结果

## MODIFIED Requirements

### Requirement: 现有 WebPath 兼容

**Modified**: 保留现有 `WebPath` 系统作为迁移过渡，新系统应能与之共存。

#### Scenario: 渐进式迁移
- **WHEN** 团队逐步迁移到注解系统
- **THEN** 两种注册方式可以共存，逐步替代

## 技术选型

| 方案 | 优点 | 缺点 |
|------|------|------|
| **Proc Macro (推荐)** | 编译时展开，功能强大 | 实现复杂 |
| Trait + PhantomData | 简单直观 | 需手动实现 trait |
| 宏规则 | 实现中等 | 灵活性较低 |

### 推荐技术栈

- `proc_macro2` - 代码生成基础
- `quote` - 生成 Rust 代码
- `syn` - 解析 Rust 代码
- `utoipa` - OpenAPI 集成
- `axum` - 框架集成