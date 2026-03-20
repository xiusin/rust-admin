# 项目重构验证清单

## 目录结构验证

- [ ] 新目录 `domain/` 存在且包含 `entity/`、`model/`、`args/` 子目录
- [ ] 新目录 `application/` 存在且包含 `sys/`、`monitor/`、`system/` 子目录
- [ ] 新目录 `infrastructure/` 存在且包含 `db/`、`cache/`、`mailer/`、`storage/` 子目录
- [ ] 新目录 `middleware/` 存在（由 `midle_ware/` 重命名）
- [ ] 旧目录 `service/sys/` 已移除
- [ ] 旧目录 `model/sys/` 已移除
- [ ] 旧目录 `midle_ware/` 已移除

## 文件命名验证

- [ ] `service/sys/s_sys_user.rs` 已重命名为 `application/sys/user_service.rs`
- [ ] `service/sys/s_sys_role.rs` 已重命名为 `application/sys/role_service.rs`
- [ ] `service/sys/s_sys_menu.rs` 已重命名为 `application/sys/menu_service.rs`
- [ ] `service/sys/s_sys_dept.rs` 已重命名为 `application/sys/dept_service.rs`
- [ ] `service/sys/s_sys_job.rs` 已移动到 `application/monitor/job_service.rs`
- [ ] `model/sys/entity/*.rs` 已移动到 `domain/entity/`
- [ ] `model/sys/model/*.rs` 已移动到 `domain/model/`
- [ ] `model/sys/args/*.rs` 已移动到 `domain/args/`

## 编译验证

- [ ] `cargo build` 编译通过，无错误
- [ ] 无未解析的模块引用
- [ ] 无循环依赖警告

## 运行时验证

- [ ] 应用启动成功
- [ ] 数据库连接正常
- [ ] 缓存初始化成功
- [ ] 定时任务调度器启动成功
- [ ] API 路由注册成功

## 功能验证

- [ ] 用户登录功能正常
- [ ] 用户列表查询正常
- [ ] 角色管理功能正常
- [ ] 菜单管理功能正常
- [ ] 定时任务功能正常
- [ ] 操作日志记录正常

## 文档更新验证

- [ ] AGENTS.md 已更新反映新的目录结构
- [ ] 新的模块职责说明已添加到文档