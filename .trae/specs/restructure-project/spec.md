# 项目深度架构重构规范

## Why

当前项目存在以下架构问题：

1. **模块职责边界模糊** - `src/common` 承载过多工具模块（snowflakeid、tera、util等），不符合单一职责原则
2. **命名不规范** - 文件命名使用 `s_sys_user`（前缀s表示service），违反 AGENTS.md 中 `snake_case` 命名规范
3. **目录结构过深** - `src/service/sys/s_sys_user.rs` 层次过深，不够扁平化
4. **模块分组不合理** - 未按业务领域（system、monitor、workflow）清晰划分
5. **缺乏工程化分层** - 缺少 `domain`（领域）、`application`（应用层）概念

## What Changes

### 重构后目录结构

```
src/
├── api/                          # API 路由层
│   ├── sys/                      # 系统管理模块（用户、角色、菜单等）
│   │   ├── user.rs               # 用户路由定义
│   │   ├── role.rs               # 角色路由定义
│   │   ├── menu.rs               # 菜单路由定义
│   │   ├── dept.rs               # 部门路由定义
│   │   ├── auth.rs               # 认证路由定义
│   │   └── ...
│   ├── monitor/                  # 监控模块（日志、定时任务）
│   │   ├── job.rs
│   │   ├── job_log.rs
│   │   └── operation_log.rs
│   ├── system/                   # 系统配置模块（字典、API权限）
│   │   ├── dict.rs
│   │   └── api_permission.rs
│   └── web_path.rs               # 路由工具
│
├── domain/                       # 领域实体层（新增）
│   ├── entity/                   # Sea-ORM Entity 定义
│   │   ├── prelude.rs
│   │   ├── sys_user.rs
│   │   ├── sys_role.rs
│   │   ├── sys_menu.rs
│   │   └── ...
│   ├── model/                    # 领域模型（查询、列表）
│   │   ├── prelude.rs
│   │   ├── sys_user.rs
│   │   └── ...
│   └── args/                     # 领域参数（请求参数）
│       ├── prelude.rs
│       ├── sys_user.rs
│       └── ...
│
├── application/                  # 应用服务层（新增）
│   ├── sys/                      # 系统管理应用服务
│   │   ├── user_service.rs      # 用户服务（合并原有 s_sys_user）
│   │   ├── role_service.rs      # 角色服务
│   │   ├── menu_service.rs       # 菜单服务
│   │   ├── auth_service.rs       # 认证服务
│   │   └── ...
│   ├── monitor/                  # 监控应用服务
│   │   ├── job_service.rs
│   │   ├── job_log_service.rs
│   │   └── operation_log_service.rs
│   └── system/                  # 系统配置应用服务
│       ├── dict_service.rs
│       └── api_permission_service.rs
│
├── infrastructure/               # 基础设施层（重构）
│   ├── db/                       # 数据库
│   │   ├── mod.rs
│   │   ├── registry.rs
│   │   ├── router.rs
│   │   └── id.rs
│   ├── cache/                    # 缓存
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── memory.rs
│   │   └── redis.rs
│   ├── mailer/                  # 邮件（从 worker 提取）
│   │   ├── mod.rs
│   │   ├── email_sender.rs
│   │   └── template.rs
│   └── storage/                  # 文件存储（新增）
│       └── uploader.rs
│
├── middleware/                   # 中间件层（从 midle_ware 重命名）
│   ├── mod.rs
│   ├── auth.rs
│   ├── jwt.rs
│   ├── request_log.rs
│   └── operate_log.rs
│
├── worker/                      # 后台任务层
│   ├── mod.rs
│   ├── scheduler/                # 调度器
│   │   ├── job.rs
│   │   ├── processor.rs
│   │   ├── periodic.rs
│   │   └── enqueue_opts.rs
│   ├── processor/                # 处理器
│   │   ├── logininfo.rs
│   │   ├── requesturl.rs
│   │   └── invokefunction.rs
│   └── mailer/                  # 邮件任务
│       └── ...
│
├── config/                       # 配置层
│   ├── mod.rs
│   └── app_config.rs
│
├── common/                       # 公共工具层
│   ├── mod.rs
│   ├── error.rs                  # 统一错误定义
│   ├── result.rs                 # 统一响应结构
│   ├── snowflake.rs              # 雪花ID生成器
│   ├── pagination.rs             # 分页工具
│   └── validation.rs              # 验证工具
│
├── lib.rs
└── main.rs
```

### 命名规范修正

| 原命名 | 重构后 | 说明 |
|--------|--------|------|
| `s_sys_user.rs` | `user_service.rs` | 去除前缀s，使用完整单词 |
| `sys_user()` | `user_service()` | 路由处理函数命名 |
| `asys_user.rs` | `user_args.rs` | args 模块命名 |
| `msys_user.rs` | `user_model.rs` | model 模块命名 |
| `midle_ware/` | `middleware/` | 目录命名修正 |
| `worker/common/` | `worker/scheduler/` | 任务调度器目录 |

### 模块职责重新划分

| 模块 | 职责 |
|------|------|
| `domain` | 数据模型、实体定义、业务规则约束 |
| `application` | 业务用例编排、跨领域协调 |
| `infrastructure` | 数据库访问、缓存、文件存储、外部服务 |
| `api` | HTTP 路由、请求处理、响应格式化 |
| `middleware` | 认证、授权、日志、限流 |
| `worker` | 异步任务、定时任务、消息队列 |

## Impact

- **受影响规范**: AGENTS.md 中模块划分标准（需同步更新）
- **受影响代码**: 约 200+ 文件需要重命名和移动
- **迁移策略**: 分阶段执行，保持向后兼容

## ADDED Requirements

### Requirement: 新增 domain 层抽象

系统 SHALL 提供清晰的领域层，用于隔离数据模型和业务逻辑。

#### Scenario: 领域实体定义
- **WHEN** 定义新的系统用户实体
- **THEN** 应在 `domain/entity/` 下创建 `sys_user.rs`，包含完整的 Sea-ORM Entity 定义和关联关系

### Requirement: 应用服务层解耦

系统 SHALL 提供应用服务层，用于编排业务用例。

#### Scenario: 用户注册流程
- **WHEN** 执行用户注册业务用例
- **THEN** 应在 `application/sys/user_service.rs` 中实现，调用 domain 层和 infrastructure 层

## MODIFIED Requirements

### Requirement: 模块命名规范

AGENTS.md 中规定的命名规范需增加以下条款：
- Service 层文件命名：`{业务名称}_service.rs`（如 `user_service.rs`，非 `s_sys_user.rs`）
- 中间件目录：`middleware/`（非 `midle_ware/`）
- 应用服务：`application/` 目录替代部分 `service/` 职责

## REMOVED Requirements

### Requirement: 旧目录结构

**Reason**: 原 `service/sys/s_sys_*.rs` 结构层次过深，不符合工程化标准

**Migration**: 重构为 `application/{domain}/{entity}_service.rs` 扁平结构