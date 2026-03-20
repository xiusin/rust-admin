# 项目深度架构重构 - 完整修改报告

**日期**: 2026-03-20
**版本**: v2.0.0
**状态**: ✅ 已完成

---

## 一、重构概述

### 1.1 重构目标

将项目从扁平的 `src/service/sys/s_sys_*.rs` 结构迁移到清晰的 **DDD-Like 分层架构**，提升代码的可维护性、可扩展性和工程化水平。

### 1.2 重构后的目录结构

```
src/
├── api/                          # API 路由层
│   └── sys/                      # 系统管理路由（已拆分）
│       ├── user.rs               # 用户路由
│       ├── role.rs               # 角色路由
│       ├── menu.rs               # 菜单路由
│       ├── dept.rs               # 部门路由
│       ├── auth.rs               # 认证路由
│       ├── dict.rs               # 字典路由
│       ├── job.rs                # 定时任务路由
│       ├── monitor.rs            # 监控路由
│       ├── dashboard.rs          # 仪表盘路由
│       ├── permission.rs         # 权限路由
│       ├── server.rs             # 服务器路由
│       ├── setting.rs             # 设置路由
│       └── mod.rs
│
├── application/                  # 应用服务层（新增）
│   ├── sys/                      # 系统管理服务
│   │   ├── user_service.rs       # 用户服务
│   │   ├── role_service.rs        # 角色服务
│   │   ├── menu_service.rs        # 菜单服务
│   │   ├── dept_service.rs        # 部门服务
│   │   ├── captcha_service.rs      # 验证码服务
│   │   ├── auth_service.rs        # 认证服务（整合）
│   │   ├── user_role_service.rs   # 用户角色服务
│   │   ├── user_dept_service.rs   # 用户部门服务
│   │   ├── role_api_service.rs    # 角色API服务
│   │   ├── white_jwt_service.rs   # 白名单JWT服务
│   │   ├── api_permission_service.rs  # API权限服务
│   │   ├── dashboard_service.rs   # 仪表盘服务
│   │   ├── cache_service.rs       # 缓存服务
│   │   ├── upload_service.rs       # 上传服务
│   │   ├── test_service.rs         # 测试服务
│   │   └── mod.rs
│   ├── monitor/                  # 监控服务
│   │   ├── job_service.rs         # 定时任务服务
│   │   ├── job_log_service.rs      # 任务日志服务
│   │   ├── operation_log_service.rs  # 操作日志服务
│   │   ├── login_info_service.rs    # 登录日志服务
│   │   ├── server_info_service.rs   # 服务器信息服务
│   │   └── mod.rs
│   └── system/                  # 系统配置服务
│       ├── dict_type_service.rs     # 字典类型服务
│       ├── dict_data_service.rs     # 字典数据服务
│       └── mod.rs
│
├── domain/                       # 领域实体层（新增）
│   ├── entity/                   # Sea-ORM Entity
│   │   ├── sys_user.rs
│   │   ├── sys_role.rs
│   │   ├── sys_menu.rs
│   │   ├── sys_dept.rs
│   │   └── ... (20个entity文件)
│   ├── model/                    # 领域模型
│   │   ├── m_user.rs             # 原 msys_user.rs
│   │   ├── m_role.rs
│   │   ├── m_menu.rs
│   │   └── ... (18个model文件)
│   └── args/                     # 请求参数
│       ├── a_user.rs              # 原 asys_user.rs
│       ├── a_role.rs
│       ├── a_menu.rs
│       └── ... (19个args文件)
│
├── infrastructure/               # 基础设施层（重构）
│   ├── db/                       # 数据库
│   │   ├── registry.rs
│   │   ├── router.rs
│   │   ├── id.rs
│   │   └── mod.rs
│   ├── cache/                    # 缓存
│   │   ├── memory.rs
│   │   ├── redis.rs
│   │   ├── traits.rs
│   │   └── mod.rs
│   └── mailer/                   # 邮件
│       ├── email_sender.rs
│       ├── template.rs
│       └── mod.rs
│
├── middleware/                    # 中间件层（由 midle_ware 重命名）
│   ├── auth.rs
│   ├── jwt.rs
│   └── operate_log.rs
│
├── worker/                       # 后台任务层
│   ├── common/                    # 任务调度器
│   │   ├── job.rs
│   │   ├── processor.rs
│   │   ├── periodic.rs
│   │   ├── scheduled.rs
│   │   ├── enqueue_opts.rs
│   │   ├── unit_of_work.rs
│   │   ├── worker.rs
│   │   └── worker_opts.rs
│   └── ... (其他worker文件)
│
├── config/                       # 配置层
│   └── appconfig.rs
│
├── common/                       # 公共工具层
│   ├── error.rs
│   ├── result.rs
│   ├── ser.rs
│   ├── snowflakeid.rs
│   ├── tera.rs
│   ├── util.rs
│   ├── validatedform.rs
│   ├── validatedjson.rs
│   └── validatedquery.rs
│
└── lib.rs, main.rs, app.rs...
```

---

## 二、详细变更清单

### 2.1 目录变更

| 操作 | 原路径 | 新路径 |
|------|--------|--------|
| 新增 | - | `src/domain/` |
| 新增 | - | `src/application/` |
| 重命名 | `src/midle_ware/` | `src/middleware/` |
| 重命名 | `src/cache/` | `src/infrastructure/cache/` |
| 移动 | `src/worker/mailer/` | `src/infrastructure/mailer/` |
| 移动 | `src/db/` | `src/infrastructure/db/` |
| 拆分 | `src/api/sys_controll.rs` | `src/api/sys/*.rs` (12个文件) |

### 2.2 文件命名变更

| 原文件名 | 新文件名 |
|----------|----------|
| `s_sys_user.rs` | `user_service.rs` |
| `s_sys_role.rs` | `role_service.rs` |
| `s_sys_menu.rs` | `menu_service.rs` |
| `s_sys_dept.rs` | `dept_service.rs` |
| `s_sys_job.rs` | `job_service.rs` |
| `s_sys_job_log.rs` | `job_log_service.rs` |
| `s_sys_operation_log.rs` | `operation_log_service.rs` |
| `s_sys_login_info.rs` | `login_info_service.rs` |
| `s_sys_dict_type.rs` | `dict_type_service.rs` |
| `s_sys_dict_data.rs` | `dict_data_service.rs` |
| `msys_user.rs` | `m_user.rs` |
| `msys_role.rs` | `m_role.rs` |
| `msys_menu.rs` | `m_menu.rs` |
| `msys_*.rs` | `m_*.rs` (统一前缀) |
| `asys_user.rs` | `a_user.rs` |
| `asys_role.rs` | `a_role.rs` |
| `asys_*.rs` | `a_*.rs` (统一前缀) |

---

## 三、配置文件变更

### 3.1 数据库密码环境变量支持

**问题**: 数据库密码硬编码在 `config/development.yaml` 中，存在安全风险。

**解决方案**: 支持 `${ENV_VAR}` 格式的环境变量展开。

**修改文件**:
- `src/config/appconfig.rs` - 新增 `expand_env_vars()` 函数
- `config/development.yaml` - 数据库 URI 改用环境变量

**使用方式**:
```bash
# 设置环境变量
export DB_PASSWORD="Qiluo123"

# 或在运行前导出
export DB_PASSWORD="your_password"
```

**YAML 配置示例**:
```yaml
databases:
  - name: primary
    uri: "mysql://qiluo:${DB_PASSWORD}@127.0.0.1:3306/qiluoopen"
```

---

## 四、编译验证

### 4.1 编译状态

```
✅ cargo build - 编译通过
⚠️  无错误
⚠️  10 个 warnings (未使用的 Router 导入，已清理)
```

---

## 五、AGENTS.md 文档更新

已更新以下章节以反映新架构：

1. **2.1.1 命名规范** - 更新文件名示例
2. **3.1 分层架构规范** - 更新分层图表
3. **3.2 模块划分标准** - 更新后端模块表格
4. **5.2 Domain 层结构规范** - 新增
5. **5.3 Application 层结构规范** - 新增
6. **4.2 API 路由定义规范** - 更新代码示例

---

## 六、Task 清单

| Task | 描述 | 状态 |
|------|------|------|
| Task 1 | 创建新目录结构骨架 | ✅ 完成 |
| Task 2 | 迁移 domain 层文件 | ✅ 完成 |
| Task 3 | 迁移并重构 application 层 | ✅ 完成 |
| Task 4 | 重构 infrastructure 层 | ✅ 完成 |
| Task 5 | 重构 API 路由层 | ✅ 完成 |
| Task 6 | 重构 middleware 层 | ✅ 完成 |
| Task 7 | 更新 lib.rs 和 main.rs | ✅ 完成 |
| Task 8 | 更新 Cargo.toml 依赖路径 | ✅ 完成 |
| Task 9 | 运行 cargo build 验证编译 | ✅ 完成 |
| Task 10 | 更新 AGENTS.md 文档 | ✅ 完成 |
| Task 11 | 数据库密码环境变量支持 | ✅ 完成 |

---

## 七、后续建议

1. **删除旧 service 目录**: 确认所有功能正常后，可删除 `src/service/` 目录
2. **删除旧 model/sys 目录**: 确认 domain 层工作正常后，可删除原 `src/model/sys/` 目录
3. **配置加密**: 生产环境建议对敏感配置进行加密存储
4. **环境变量文档**: 建议在 `docs/` 目录下创建环境变量配置文档

---

## 八、变更统计

| 类别 | 数量 |
|------|------|
| 新增目录 | 6 个 |
| 重命名目录 | 1 个 |
| 移动文件 | 60+ 个 |
| 重命名文件 | 40+ 个 |
| 修改文件 | 20+ 个 |
| 编译错误 | 0 个 |

---

**报告生成时间**: 2026-03-20