# 项目重构任务清单

## 重构目标

将现有项目从扁平的 `src/service/sys/s_sys_*.rs` 结构迁移到清晰的 `DDD-Like` 分层架构：
- `domain/` - 领域层（Entity、Model、Args）
- `application/` - 应用服务层
- `infrastructure/` - 基础设施层（DB、Cache、Mailer）
- `api/` - 路由层
- `middleware/` - 中间件层
- `worker/` - 后台任务层

## 任务列表

- [x] Task 1: 创建新目录结构骨架
  - [x] SubTask 1.1: 创建 `domain/entity/`、`domain/model/`、`domain/args/` 目录
  - [x] SubTask 1.2: 创建 `application/sys/`、`application/monitor/`、`application/system/` 目录
  - [x] SubTask 1.3: 创建 `infrastructure/db/`、`infrastructure/cache/`、`infrastructure/mailer/`、`infrastructure/storage/` 目录
  - [x] SubTask 1.4: 重命名 `midle_ware/` 为 `middleware/`

- [x] Task 2: 迁移 domain 层文件
  - [x] SubTask 2.1: 将 `model/sys/entity/*.rs` 移动到 `domain/entity/`
  - [x] SubTask 2.2: 将 `model/sys/model/*.rs` 移动到 `domain/model/`
  - [x] SubTask 2.3: 将 `model/sys/args/*.rs` 移动到 `domain/args/`
  - [x] SubTask 2.4: 更新 `domain/entity/prelude.rs` 和 `domain/model/prelude.rs`
  - [x] SubTask 2.5: 删除原 `model/sys/` 目录

- [x] Task 3: 迁移并重构 application 层
  - [x] SubTask 3.1: 将 `service/sys/s_sys_user.rs` 重命名为 `application/sys/user_service.rs`
  - [x] SubTask 3.2: 将 `service/sys/s_sys_role.rs` 重命名为 `application/sys/role_service.rs`
  - [x] SubTask 3.3: 将 `service/sys/s_sys_menu.rs` 重命名为 `application/sys/menu_service.rs`
  - [x] SubTask 3.4: 将 `service/sys/s_sys_dept.rs` 重命名为 `application/sys/dept_service.rs`
  - [x] SubTask 3.5: 将 `service/sys/s_sys_auth.rs`（或相关）重命名为 `application/sys/auth_service.rs`
  - [x] SubTask 3.6: 将 `service/sys/s_sys_job.rs` 移动到 `application/monitor/job_service.rs`
  - [x] SubTask 3.7: 将 `service/sys/s_sys_job_log.rs` 移动到 `application/monitor/job_log_service.rs`
  - [x] SubTask 3.8: 将 `service/sys/s_sys_operation_log.rs` 移动到 `application/monitor/operation_log_service.rs`
  - [x] SubTask 3.9: 将 `service/sys/s_sys_dict_*.rs` 移动到 `application/system/dict_service.rs`
  - [x] SubTask 3.10: 将 `service/sys/s_sys_api_permission.rs` 移动到 `application/system/api_permission_service.rs`
  - [x] SubTask 3.11: 更新 `application/sys/mod.rs`、`application/monitor/mod.rs`、`application/system/mod.rs`

- [x] Task 4: 重构 infrastructure 层
  - [x] SubTask 4.1: 将 `cache/*.rs` 移动到 `infrastructure/cache/`
  - [x] SubTask 4.2: 将 `worker/mailer/*.rs` 移动到 `infrastructure/mailer/`
  - [x] SubTask 4.3: 创建 `infrastructure/storage/` 目录用于文件上传
  - [x] SubTask 4.4: 更新 `infrastructure/` 模块的 `mod.rs`

- [x] Task 5: 重构 API 路由层
  - [x] SubTask 5.1: 将 `api/sys_controll.rs` 拆分为 `api/sys/user.rs`、`api/sys/role.rs` 等
  - [x] SubTask 5.2: 创建 `api/monitor/`、`api/system/` 目录结构
  - [x] SubTask 5.3: 更新 `api/web_path.rs` 以适配新结构
  - [x] SubTask 5.4: 更新 `api/mod.rs` 引入新模块

- [x] Task 6: 重构 middleware 层
  - [x] SubTask 6.1: 重命名 `midle_ware/` 为 `middleware/`
  - [x] SubTask 6.2: 更新所有 `use` 引用从 `crate::midle_ware` 到 `crate::middleware`
  - [x] SubTask 6.3: 更新 `lib.rs` 中的模块声明

- [x] Task 7: 更新 lib.rs 和 main.rs
  - [x] SubTask 7.1: 更新 `lib.rs` 中的模块声明顺序
  - [x] SubTask 7.2: 更新 `app.rs` 中的引用路径
  - [x] SubTask 7.3: 更新所有内部 `use` 语句

- [x] Task 8: 更新 Cargo.toml 依赖路径（如需要）

- [x] Task 9: 运行 cargo build 验证编译通过

- [ ] Task 10: 更新 AGENTS.md 文档以反映新的目录结构

## Task Dependencies

- Task 2 依赖 Task 1
- Task 3 依赖 Task 1 和 Task 2
- Task 4 依赖 Task 1
- Task 5 依赖 Task 1、Task 2、Task 3
- Task 6 依赖 Task 1
- Task 7 依赖 Task 2、Task 3、Task 4、Task 5、Task 6
- Task 8 依赖 Task 7
- Task 9 依赖 Task 8
- Task 10 可以在 Task 9 完成后进行

## 并行化建议

以下任务可以并行执行：
- Task 2、Task 4、Task 6 可以并行（都在 Task 1 之后）
- Task 3 的各个 SubTask 可以并行
- Task 5 的各个 SubTask 可以并行