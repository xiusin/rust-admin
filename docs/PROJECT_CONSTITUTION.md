# 项目开发范式宪法

**版本**: v1.0.0
**制定日期**: 2026-03-19
**状态**: 正式生效

---

## 前言

本文档作为项目开发的最高指导纲领，定义了项目开发过程中的核心范式规范、角色职责及质量保障机制。所有开发人员必须严格遵守本文档的规定，确保项目开发过程的一致性、可维护性和高质量。

---

## 第一部分：项目开发范式规范

### 1. 技术栈规范

#### 1.1 技术栈总览

| 层级 | 技术选型 | 版本要求 |
|------|----------|----------|
| **后端语言** | Rust | >= 1.70.0 |
| **后端框架** | Axum | 0.8.x |
| **ORM框架** | Sea-ORM | 1.1.x |
| **异步运行时** | Tokio | 1.x |
| **数据库** | MySQL >= 8.0 / SQLite >= 3.35 | 最新稳定版 |
| **缓存** | Redis | >= 6.0 |
| **前端框架** | Vue | 3.4.x |
| **构建工具** | Vite | 5.2.x |
| **类型系统** | TypeScript | 5.4.x |
| **状态管理** | Pinia | 2.1.x |
| **路由管理** | Vue Router | 4.3.x |
| **UI组件库** | Element Plus | 2.7.x |

#### 1.2 依赖管理规范

**Rust 依赖管理**：
- 所有 Rust 依赖必须通过 `Cargo.toml` 声明
- 禁止在代码中直接使用未声明的 crate
- 依赖版本必须锁定到次版本号（如 `1.1.19`，禁止使用 `1.1` 或 `latest`）
- 新增依赖必须经过技术评审

**前端依赖管理**：
- 使用 pnpm 作为包管理器
- 所有前端依赖必须通过 `package.json` 声明
- 禁止使用 `npm` 或 `yarn` 混入项目
- 依赖版本锁定到精确版本

---

### 2. 代码风格规范

#### 2.1 Rust 代码风格

##### 2.1.1 命名规范

| 元素 | 命名规范 | 示例 |
|------|----------|------|
| 模块名 | snake_case | `sys_user`, `cache_manager` |
| 函数名 | snake_case | `get_user_info`, `delete_by_id` |
| 变量名 | snake_case | `user_name`, `dept_id` |
| 结构体名 | PascalCase | `SysUserModel`, `ApiResponse` |
| 枚举名 | PascalCase | `Error`, `Relation` |
| 枚举成员 | PascalCase | `Message`, `WithStatus` |
| 常量名 | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT`, `TOKEN_EXPIRY` |
| 文件名 | snake_case | `s_sys_user.rs`, `sys_controll.rs` |

##### 2.1.2 函数设计规范

```rust
// ✅ 正确示例：清晰的函数签名，参数命名规范
pub async fn get_user_info(uid: i64) -> Result<UserInfoDetail> {
    // 函数实现
}

// ✅ 正确示例：多参数使用结构体
pub async fn list_users(arg: UserSearchParams) -> Result<ListData<UserRes>> {
    // 函数实现
}

// ❌ 错误示例：参数过多且无结构化
pub async fn get_user_info(uid: i64, uname: String, did: i64, rid: i64) -> Result<UserInfoDetail>

// ✅ 正确示例：返回类型明确，使用 Result 包装
pub async fn find_by_id(id: i64) -> Result<Option<User>>

// ❌ 错误示例：使用 panic 代替错误处理
pub async fn get_user_info(uid: i64) -> UserInfoDetail {
    user.unwrap() // 禁止 unwrap
}
```

##### 2.1.3 错误处理规范

**必须使用自定义错误枚举**：
```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Message(String),
    WithStatus(StatusCode, String),
}

impl Error {
    pub fn bad_request(msg: impl Into<String>) -> Self
    pub fn unauthorized(msg: impl Into<String>) -> Self
    pub fn forbidden(msg: impl Into<String>) -> Self
    pub fn not_found(msg: impl Into<String>) -> Self
    pub fn validation_error(msg: impl Into<String>) -> Self
    pub fn internal_error(msg: impl Into<String>) -> Self
}
```

**必须使用 Result 别名**：
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

**禁止使用 unwrap/expect 进行错误处理**：
```rust
// ❌ 禁止
let user = user_result.unwrap();

// ✅ 推荐
let user = user_result?;
```

**必须实现标准错误转换**：
```rust
impl From<sea_orm::DbErr> for Error { ... }
impl From<std::io::Error> for Error { ... }
```

##### 2.1.4 异步编程规范

**必须使用 async/await**：
```rust
// ✅ 正确
pub async fn login(params: LoginParams) -> Result<TokenResponse> {
    let user = SysUserModel::find_by_username(&params.username).await?;
    // ...
}

// ❌ 禁止同步阻塞
pub fn login(params: LoginParams) -> Result<TokenResponse> {
    let user = blocking_call_to_db();
}
```

**Tokio 使用规范**：
- 使用 `#[tokio::main]` 作为入口
- 后台任务使用 `tokio::spawn`
- 避免在 hot path 中创建不必要的 task

##### 2.1.5 可见性规范

```rust
// ✅ 对外 API 使用 pub
pub async fn public_api() -> Result<Response> { ... }

// ✅ 内部实现使用默认可见性
async fn internal_helper() -> Result<()> { ... }

// ✅ 跨模块调用使用 pub(crate)
pub(crate) fn registry() -> &'static Registry { ... }
```

#### 2.2 前端代码风格（Vue/TypeScript）

##### 2.2.1 命名规范

| 元素 | 命名规范 | 示例 |
|------|----------|------|
| 组件名 | PascalCase | `UserList.vue`, `LoginForm.vue` |
| 变量名 | camelCase | `userName`, `deptId` |
| 函数名 | camelCase | `getUserInfo`, `deleteUser` |
| 常量名 | SCREAMING_SNAKE_CASE | `MAX_COUNT`, `API_BASE_URL` |
| 文件名 | kebab-case | `user-list.vue`, `login-form.vue` |
| 目录名 | kebab-case | `user-management`, `api-config` |

##### 2.2.2 TypeScript 类型规范

```typescript
// ✅ 必须显式声明类型
interface UserInfo {
  id: number;
  userName: string;
  email?: string;
  status: 'active' | 'inactive';
}

// ✅ 函数必须声明返回类型
async function getUserInfo(id: number): Promise<UserInfo> {
  return await api.get(`/user/${id}`);
}

// ❌ 禁止使用 any
function handleData(data: any) { }

// ✅ 优先使用 interface，复杂类型使用 type
interface UserProps {}
type UserCallback = (user: User) => void;
```

##### 2.2.3 Vue 组件规范

```vue
<!-- ✅ 正确：使用 Composition API -->
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useUserStore } from '@/store/user';

const userStore = useUserStore();
const userName = ref('');

const isAdmin = computed(() => userStore.role === 'admin');

onMounted(() => {
  loadUserData();
});
</script>

<!-- ❌ 禁止：Options API -->
<script>
export default {
  data() {
    return { userName: '' }
  }
}
</script>
```

---

### 3. 架构设计原则

#### 3.1 分层架构规范

项目采用经典的分层架构，各层职责明确：

```
┌─────────────────────────────────────────┐
│              API Layer                  │  路由与控制器层
│         (src/api/*.rs)                  │  请求接收与响应
├─────────────────────────────────────────┤
│            Service Layer                │  业务逻辑层
│       (src/service/**/*.rs)             │  业务规则处理
├─────────────────────────────────────────┤
│            Model Layer                  │  数据模型层
│   (src/model/**/{entity,model,args})   │  数据结构定义
├─────────────────────────────────────────┤
│            DB / Cache                   │  数据访问层
│      (src/db/*.rs, src/cache/*.rs)     │  数据库与缓存
└─────────────────────────────────────────┘
```

#### 3.2 模块划分标准

**后端模块划分**：

| 模块 | 职责 | 目录 |
|------|------|------|
| api | API路由与控制器 | `src/api/` |
| service | 业务逻辑服务 | `src/service/` |
| model | 数据模型定义 | `src/model/` |
| db | 数据库连接管理 | `src/db/` |
| cache | 缓存抽象层 | `src/cache/` |
| config | 配置管理 | `src/config/` |
| common | 公共工具 | `src/common/` |
| midle_ware | 中间件 | `src/midle_ware/` |
| worker | 后台任务处理 | `src/worker/` |

**前端模块划分**：

| 模块 | 职责 | 目录 |
|------|------|------|
| api | 接口定义 | `src/api/` |
| components | 通用组件 | `src/components/` |
| views | 页面视图 | `src/views/` |
| store | 状态管理 | `src/store/` |
| router | 路由配置 | `src/router/` |
| hooks | 组合式函数 | `src/hooks/` |
| utils | 工具函数 | `src/utils/` |
| constants | 常量定义 | `src/constants/` |

#### 3.3 依赖注入规范

**全局单例模式**（后端）：
```rust
// 数据库连接获取
pub async fn db() -> &'static DatabaseConnection {
    let reg = registry().await;
    &reg.conns[reg.default_idx]
}

// 缓存实例获取
pub async fn instance() -> Arc<Cache> {
    GLOBAL_CACHE.get().cloned().expect("Cache not initialized")
}
```

**Pinia Store 模式**（前端）：
```typescript
// store/user.ts
export const useUserStore = defineStore('user', () => {
  const userInfo = ref<UserInfo | null>(null);

  async function fetchUserInfo() {
    const res = await api.getUserInfo();
    userInfo.value = res.data;
  }

  return { userInfo, fetchUserInfo };
});
```

#### 3.4 配置管理规范

**配置文件位置**：`config/` 目录

**配置格式**：YAML

**环境隔离**：
- 开发环境：`.env` 或 `config/dev.yaml`
- 生产环境：`.env.pro` 或 `config/pro.yaml`

**敏感信息管理**：
- 数据库密码、API密钥等敏感信息禁止硬编码
- 必须通过环境变量或配置文件注入
- 敏感配置不纳入版本控制

---

### 4. 接口定义规范

#### 4.1 RESTful API 设计

##### 4.1.1 URL 规范

| 操作 | HTTP方法 | URL模式 | 示例 |
|------|----------|---------|------|
| 查询列表 | GET | /{resource}/list | GET /sys/user/list |
| 查询详情 | GET | /{resource}/:{id} | GET /sys/user/1 |
| 新增 | POST | /{resource}/add | POST /sys/user/add |
| 更新 | PUT | /{resource}/edit | PUT /sys/user/edit |
| 删除 | DELETE | /{resource}/del | DELETE /sys/user/del |

##### 4.1.2 请求格式

**查询参数**：
```rust
#[derive(Debug, Deserialize, Validate)]
pub struct UserSearchParams {
    pub dept_id: i64,
    pub user_name: Option<String>,
    pub phonenumber: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
}
```

**请求体**：
```rust
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserAddRequest {
    pub dept_id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    #[serde(default)]
    pub remark: Option<String>,
}
```

##### 4.1.3 响应格式

**统一响应结构**：
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub page_num: u32,
    pub page_size: u32,
}
```

**HTTP状态码使用**：
| 状态码 | 使用场景 |
|--------|----------|
| 200 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未认证/认证失败 |
| 403 | 无权限 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

#### 4.2 API 路由定义规范

```rust
// src/api/sys_controll.rs
pub fn router_sys() -> WebPath {
    WebPath::new().nest(
        "/sys",
        WebPath::new()
            .nest("/user", sys_user())
            .nest("/menu", menu())
            .nest("/dept", sys_dept())
    )
}

fn sys_user() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取用户列表"), get(s_sys_user::list))
        .route("/add", WebPathType::Post, Some("添加用户"), post(s_sys_user::add))
        .route("/edit", WebPathType::Put, Some("编辑用户"), put(s_sys_user::edit))
        .route("/del", WebPathType::Delete, Some("删除用户"), delete(s_sys_user::delete_users))
}
```

---

### 5. 数据模型规范

#### 5.1 Sea-ORM Entity 规范

```rust
// ✅ 标准 Entity 定义
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub dept_id: i64,
    pub role_id: i64,
    pub user_name: String,
    pub nick_name: String,
    #[sea_orm(nullable)]
    pub email: Option<String>,
    pub password: String,
    #[sea_orm(nullable)]
    pub status: Option<String>,
    #[sea_orm(nullable)]
    pub created_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub updated_at: Option<DateTime>,
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime>,
}

// ✅ 关系定义
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_user_dept::Entity")]
    SysUserDept,
    #[sea_orm(has_many = "super::sys_user_role::Entity")]
    SysUserRole,
}

// ✅ 关联实现
impl Related<super::sys_dept::Entity> for Entity {
    fn to() -> RelationDef {
        super::sys_user_dept::Relation::SysDept.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sys_user_dept::Relation::SysUser.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

#### 5.2 Model 层结构规范

每个业务模块的 Model 层必须包含三个子模块：

```
src/model/sys/
├── entity/          # Sea-ORM Entity 定义
│   ├── mod.rs
│   └── sys_user.rs
├── model/           # 业务 Model（查询、列表等）
│   ├── mod.rs
│   └── msys_user.rs
├── args/            # 请求参数结构体
│   ├── mod.rs
│   └── asys_user.rs
└── mod.rs
```

#### 5.3 数据库迁移规范

```rust
// migration/src/m20220101_000001_create_table.rs
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(SysUser::Table)
            .col(string(SysUser::UserId).primary_key().unique_key().char_len(32))
            .col(string(SysUser::UserName).char_len(50))
            .col(string(SysUser::Password).char_len(128))
            // ...
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum SysUser {
    Table,
    UserId,
    UserName,
    Password,
}
```

---

### 6. 版本控制策略

#### 6.1 Git 工作流

采用 **Gitflow** 工作流：

```
main (生产分支)
  └── develop (开发分支)
        ├── feature/* (功能分支)
        ├── bugfix/* (修复分支)
        └── release/* (发布分支)
```

#### 6.2 分支命名规范

| 分支类型 | 命名格式 | 示例 |
|----------|----------|------|
| 功能分支 | feature/{模块}-{功能名} | feature/user-auth |
| 修复分支 | bugfix/{模块}-{问题描述} | bugfix/login-crash |
| 发布分支 | release/v{版本号} | release/v1.0.0 |
| 热修复分支 | hotfix/{问题描述} | hotfix/security-patch |

#### 6.3 提交信息规范

**格式**：
```
<类型>(<模块>): <简短描述>

[可选的详细描述]

[可选的关联Issue]
```

**类型枚举**：
| 类型 | 描述 |
|------|------|
| feat | 新功能 |
| fix | 修复bug |
| docs | 文档更新 |
| style | 代码格式（不影响功能） |
| refactor | 重构 |
| perf | 性能优化 |
| test | 测试相关 |
| chore | 构建/工具相关 |

**示例**：
```
feat(user): 添加用户登录验证码功能

- 集成滑动验证码
- 添加验证码缓存机制
- 更新登录接口参数

Closes #123
```

#### 6.4 版本号规范

采用 **语义化版本** (SemVer)：

```
主版本.次版本.修订号
  MAJOR   MINOR   PATCH

- MAJOR: 不兼容的API变更
- MINOR: 向后兼容的功能新增
- PATCH: 向后兼容的问题修复
```

---

## 第二部分：开发专家角色定义

### 7. 角色职责与技能要求

#### 7.1 后端Rust专家工程师

##### 7.1.1 核心职责

1. **API 开发**
   - 设计和实现 RESTful API
   - 确保接口的安全性、性能和可扩展性
   - 编写接口文档

2. **业务逻辑实现**
   - 在 Service 层实现业务规则
   - 维护业务逻辑的一致性和完整性
   - 优化业务处理性能

3. **数据层开发**
   - 设计数据库表结构
   - 编写数据迁移脚本
   - 优化数据库查询性能

4. **系统架构维护**
   - 参与系统架构设计
   - 提出技术改进建议
   - 保障系统稳定性

##### 7.1.2 技能要求

**必备技能**：
- Rust 编程语言精通
- Axum 框架深入理解
- Sea-ORM 使用经验
- Toki 异步编程掌握
- MySQL/SQLite 数据库管理
- Redis 缓存机制理解
- Git 版本控制熟练

**推荐技能**：
- 数据库读写分离设计
- 分布式系统架构经验
- 性能调优经验
- 安全编码实践

##### 7.1.3 协作标准

- 每日同步开发进度
- 代码提交前必须通过本地测试
- Pull Request 必须经过代码 review
- 遇到阻塞问题时及时升级

#### 7.2 前端Vue/UI/UX专家工程师

##### 7.2.1 核心职责

1. **界面开发**
   - 根据设计稿实现页面
   - 维护组件库的规范使用
   - 确保页面兼容性和响应式设计

2. **状态管理**
   - 设计和实现 Pinia Store
   - 管理全局状态
   - 优化状态更新性能

3. **接口对接**
   - 与后端协商接口定义
   - 实现 API 调用层
   - 处理接口异常和错误

4. **性能优化**
   - 优化页面加载速度
   - 实现懒加载和代码分割
   - 减少 bundle size

##### 7.2.2 技能要求

**必备技能**：
- Vue 3 深入理解
- TypeScript 精通
- Vite 构建工具使用
- Element Plus 组件库
- HTTP 请求库（Axios）
- CSS/SCSS 样式开发

**推荐技能**：
- Composition API 精通
- 性能优化经验
- 单元测试编写（Vitest）
- 前端安全（XSS、CSRF 防护）

##### 7.2.3 协作标准

- 使用组件前查看现有组件库
- 复杂组件需编写使用文档
- 页面适配主流浏览器
- 确保移动端兼容性

#### 7.3 测试工程师

##### 7.3.1 核心职责

1. **测试用例设计**
   - 编写功能测试用例
   - 设计边界条件和异常场景
   - 维护测试用例库

2. **自动化测试**
   - 编写单元测试
   - 编写集成测试
   - 维护测试覆盖率

3. **质量保障**
   - 执行回归测试
   - 报告和跟踪缺陷
   - 验证缺陷修复

4. **性能测试**
   - 执行 API 性能测试
   - 评估系统负载能力
   - 提供性能优化建议

##### 7.3.2 技能要求

**必备技能**：
- 软件测试理论
- 测试用例设计方法
- 缺陷管理流程
- 后端 API 测试（Postman/Insomnia）

**推荐技能**：
- 单元测试框架（Rust: tokio-test, 前端: Vitest）
- 自动化测试工具
- 性能测试工具（JMeter）
- 安全测试基础

##### 7.3.3 协作标准

- 测试用例评审参与
- 测试报告及时同步
- 缺陷描述清晰准确
- 优先测试核心业务功能

#### 7.4 全栈开发工程师

同时承担后端和前端开发职责，需满足双方所有技能要求。

#### 7.5 技术负责人

##### 7.5.1 核心职责

1. **技术决策**
   - 确定技术选型
   - 制定技术规范
   - 评审架构设计

2. **代码质量**
   - 执行代码 Review
   - 把控代码质量
   - 优化开发流程

3. **团队协作**
   - 分配开发任务
   - 协调进度
   - 培养团队能力

4. **技术债务**
   - 识别技术债务
   - 制定偿还计划
   - 预防技术风险

---

## 第三部分：防止幻觉专项规范

### 8. 信息准确性保障机制

#### 8.1 验证机制

##### 8.1.1 输入验证规范

**后端验证**（必须实现）：
```rust
// 使用 validator crate 进行参数验证
#[derive(Debug, Deserialize, Validate)]
pub struct UserAddRequest {
    #[validate(length(min = 2, max = 50, message = "用户名长度2-50"))]
    pub user_name: String,

    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,

    #[validate(regex(path = "*PHONE_REGEX", message = "手机号格式不正确"))]
    pub phonenumber: Option<String>,
}

// 自定义验证提取器
impl<T, S> FromRequest<S> for VJson<T>
where
    T: DeserializeOwned + Validate,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;  // 必须调用 validate
        Ok(VJson(value))
    }
}
```

**前端验证**：
```typescript
// 使用 vee-validate 或 zod
import { z } from 'zod';

const userSchema = z.object({
  userName: z.string().min(2).max(50),
  email: z.string().email().optional(),
  phonenumber: z.string().regex(/^1[3-9]\d{9}$/).optional(),
});

// 验证函数
function validateUser(data: unknown) {
  return userSchema.safeParse(data);
}
```

##### 8.1.2 输出验证规范

**API 响应必须包含**：
```rust
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

// 禁止直接返回数据库模型
// 必须转换为 DTO 后返回
```

**前端响应处理**：
```typescript
// 统一响应处理
axios.interceptors.response.use(
  (response) => {
    if (response.data.message !== 'success') {
      Message.error(response.data.message);
    }
    return response.data;
  },
  (error) => {
    Message.error(error.response?.data?.message || '网络错误');
    return Promise.reject(error);
  }
);
```

##### 8.1.3 类型安全验证

**后端**：
- 所有数据库字段必须声明类型
- API 请求/响应必须使用强类型
- 禁止使用 `serde_json::Value` 作为长期类型

**前端**：
- 所有 API 响应必须定义 TypeScript 接口
- 禁止使用 `any` 类型
- 使用类型守卫确保类型安全

#### 8.2 文档核实流程

##### 8.2.1 API 文档规范

**必须包含的信息**：
```
### 接口名称

**URL**: /api/sys/user/list
**方法**: GET
**描述**: 获取用户列表

**请求参数**:
| 参数名 | 类型 | 必填 | 描述 |
|--------|------|------|------|

**响应示例**:
```json
{
  "message": "success",
  "data": {
    "list": [],
    "total": 0
  }
}
```

**错误码**:
| 状态码 | 描述 |
|--------|------|
| 400 | 参数错误 |
| 401 | 未认证 |
```

##### 8.2.2 代码文档规范

**Rust 文档注释**：
```rust
/// 获取用户详情信息
///
/// # Arguments
/// * `uid` - 用户ID
///
/// # Returns
/// * `Result<UserInfoDetail>` - 用户详情
///
/// # Errors
/// * `Error::NotFound` - 用户不存在
pub async fn get_user_info(uid: i64) -> Result<UserInfoDetail> {
    // ...
}
```

**前端组件文档**：
```typescript
/**
 * UserCard 组件
 * 用于展示用户信息卡片
 *
 * @prop user - 用户信息对象
 * @prop showActions - 是否显示操作按钮
 *
 * @example
 * ```vue
 * <UserCard :user="userData" show-actions @edit="handleEdit" />
 * ```
 */
```

##### 8.2.3 文档更新流程

```
文档变更 → 关联代码变更 → Pull Request → Code Review → 合并后同步文档
```

- 文档必须与代码同步更新
- 文档更新必须经过 Review
- 过时文档必须及时更新

#### 8.3 知识管理规范

##### 8.3.1 知识库建设

**必须维护的文档**：
- 项目技术架构文档
- API 接口文档
- 数据库设计文档
- 部署文档
- 常见问题解答 (FAQ)

**文档存放位置**：
```
docs/
├── architecture/     # 架构文档
├── api/             # 接口文档
├── database/        # 数据库设计
├── deployment/      # 部署文档
└── faq/            # 常见问题
```

##### 8.3.2 代码知识共享

**Code Review 要点**：
1. 功能实现是否正确
2. 是否存在安全风险
3. 是否有性能问题
4. 代码是否遵循规范
5. 文档是否同步更新

**知识分享机制**：
- 每周技术分享会
- 技术方案评审
- 最佳实践沉淀

##### 8.3.3 信息核实准则

**开发人员在实现功能前必须核实**：

| 信息类型 | 核实方法 |
|----------|----------|
| API 接口规范 | 查看接口文档 + 与前端确认 |
| 数据库字段 | 查看 Entity 定义 + 迁移脚本 |
| 业务规则 | 查看需求文档 + 与产品确认 |
| 第三方依赖 | 查看官方文档 + 版本兼容性 |

**禁止行为**：
- 禁止基于假设进行开发
- 禁止跳过核实直接实现
- 禁止假设接口未变化
- 禁止猜测字段含义

---

## 第五部分：多代理/多子代理Agent调用规范

### 10. Agent 系统架构

#### 10.1 Agent 定义与分类

本项目采用 **多代理（Multi-Agent）** 系统架构，定义以下 Agent 类型：

| Agent类型 | 代号 | 职责描述 | 并发数限制 |
|-----------|------|----------|-----------|
| **主代理（Master Agent）** | `master` | 任务分解、子代理调度、结果聚合 | 1 |
| **搜索代理（Search Agent）** | `search` | 代码库搜索、信息检索、知识查询 | 3 |
| **代码审查代理（Review Agent）** | `review` | 代码质量检查、规范验证 | 2 |
| **开发代理（Code Agent）** | `code` | 代码编写、重构、修复 | 2 |
| **测试代理（Test Agent）** | `test` | 测试用例生成、测试执行 | 2 |
| **文档代理（Doc Agent）** | `doc` | 文档生成、注释检查 | 1 |

#### 10.2 Agent 层级结构

```
┌─────────────────────────────────────────────────────┐
│                  Master Agent                       │
│              (任务分解与调度中心)                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  任务队列管理                                 │   │
│  │  - 任务分解策略                               │   │
│  │  - 子代理调度                                 │   │
│  │  - 结果聚合与验证                             │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
   ┌──────────┐    ┌──────────┐    ┌──────────┐
   │ Search   │    │  Review  │    │   Code   │
   │  Agent   │    │  Agent   │    │  Agent   │
   └──────────┘    └──────────┘    └──────────┘
         │               │               │
         └───────────────┼───────────────┘
                         ▼
                   ┌──────────┐
                   │  Test    │
                   │  Agent   │
                   └──────────┘
```

### 11. Agent 通信协议

#### 11.1 消息格式定义

所有 Agent 间通信必须遵循以下消息格式：

```json
{
  "message_id": "uuid-v4",
  "timestamp": "ISO8601",
  "sender": "agent-name",
  "receiver": "agent-name | broadcast",
  "message_type": "request | response | notification | error",
  "content": {
    "task_id": "task-uuid",
    "action": "action-name",
    "payload": {},
    "metadata": {}
  },
  "status": {
    "code": 0,
    "message": "success"
  }
}
```

#### 11.2 消息类型规范

| 消息类型 | 用途 | 必须包含字段 |
|----------|------|--------------|
| `request` | 请求执行任务 | `task_id`, `action`, `payload` |
| `response` | 任务响应 | `task_id`, `result`, `status` |
| `notification` | 状态通知 | `task_id`, `event`, `data` |
| `error` | 错误报告 | `task_id`, `error_code`, `error_message` |

#### 11.3 响应状态码

| 状态码 | 含义 | 处理建议 |
|--------|------|----------|
| 0 | 成功 | 返回结果 |
| 1000 | 参数错误 | 检查输入 |
| 2000 | 执行超时 | 重试或跳过 |
| 3000 | 资源不足 | 等待或降级 |
| 4000 | 权限错误 | 检查权限配置 |
| 5000 | 系统错误 | 记录日志并告警 |

### 12. Agent 调用规范

#### 12.1 主代理调度流程

```
任务输入 → 任务分析 → 子任务分解 → 并发调度 → 结果验证 → 结果聚合 → 任务完成
```

**调度决策规则**：

1. **串行执行**：存在依赖关系的子任务
2. **并发执行**：独立的子任务（最多 4 个并发）
3. **降级策略**：子代理不可用时，使用备选代理

#### 12.2 子代理调用约束

**必须遵守的约束**：

```yaml
调用约束:
  超时限制:
    搜索代理: 30s
    代码审查: 60s
    开发代理: 300s
    测试代理: 180s
    文档代理: 60s

  重试策略:
    最大重试次数: 3
    重试间隔: 5s (指数退避)
    重试条件: 超时、网络错误

  熔断机制:
    失败阈值: 5次/分钟
    熔断恢复: 60s
```

#### 12.3 代理调用示例

**正确调用模式**：

```rust
// ✅ 正确的 Agent 调用方式
let result = agent_dispatch(AgentRequest {
    agent_type: AgentType::Search,
    task: SearchTask {
        query: "user authentication".to_string(),
        scope: SearchScope::Codebase,
        max_results: 10,
    },
    timeout: Duration::from_secs(30),
    retry_policy: RetryPolicy::default(),
}).await?;

// ✅ 结果验证
if !result.is_valid() {
    return Err(AgentError::InvalidResult);
}
```

**错误调用模式**：

```rust
// ❌ 禁止：直接忽略超时设置
agent_dispatch(request); // 缺少 timeout

// ❌ 禁止：无限重试
agent_dispatch(request.with_retry(999)); // 危险

// ❌ 禁止：缺少结果验证
let result = agent_dispatch(request)?;
process(result); // 未验证结果有效性
```

### 13. 防止 Agent 幻觉规范

#### 13.1 信息核实机制

**三级核实制度**：

| 级别 | 核实内容 | 执行者 | 时机 |
|------|----------|--------|------|
| L1 | 任务参数核实 | Master Agent | 任务分发前 |
| L2 | 执行结果核实 | 子代理 | 结果返回前 |
| L3 | 聚合结果核实 | Master Agent | 结果聚合后 |

**核实检查清单**：

- [ ] 任务参数与原始需求一致性
- [ ] 返回数据结构完整性
- [ ] 返回内容与代码库实际状态一致性
- [ ] 多个子代理结果交叉验证
- [ ] 不确定信息的置信度标记

#### 13.2 置信度评分

每个 Agent 返回结果必须包含置信度评分：

```json
{
  "result": {},
  "confidence": {
    "score": 0.85,
    "verified": true,
    "uncertainty": ["unverified_field_1"],
    "sources": ["file:///path/to/source"]
  }
}
```

**置信度等级**：

| 评分范围 | 等级 | 处理方式 |
|----------|------|----------|
| 0.9-1.0 | 高 | 可直接使用 |
| 0.7-0.9 | 中 | 建议复核 |
| 0.5-0.7 | 低 | 必须复核 |
| < 0.5 | 不可信 | 重新执行或标记失败 |

#### 13.3 知识库同步

**同步机制**：

```yaml
知识同步:
  触发条件:
    - 代码库变更后
    - 文档更新后
    - 规范变更后

  同步内容:
    - 实体定义（Entity、Model）
    - API 接口定义
    - 代码规范变更
    - 依赖版本信息

  同步验证:
    - 完整性检查
    - 一致性验证
    - 版本兼容性检查
```

### 14. Agent 协作流程规范

#### 14.1 典型工作流程

**代码审查工作流**：

```
Master Agent
    │
    ├─→ [Search Agent] 搜索相关代码
    │       │
    │       └─→ [Review Agent] 代码审查
    │               │
    │               ├─→ [Code Agent] 修复建议
    │               │       │
    │               └─→ [Test Agent] 验证修复
    │                       │
    └───────────────────────┴─→ 结果聚合 → 用户输出
```

**代码开发工作流**：

```
Master Agent
    │
    ├─→ [Search Agent] 搜索现有实现参考
    │       │
    ├─→ [Doc Agent] 检查相关文档
    │       │
    └─→ [Code Agent] 代码实现
            │
            ├─→ [Review Agent] 代码审查
            │       │
            └─→ [Test Agent] 测试生成
                    │
                    └─→ 结果验证 → 代码提交
```

#### 14.2 并发控制规范

**并发任务数限制**：

```rust
// 任务调度器配置
pub struct SchedulerConfig {
    // 最大并发任务数
    pub max_concurrent_tasks: usize = 4,

    // 单个 Agent 最大并发
    pub max_concurrent_per_agent: usize = 2,

    // 任务队列大小
    pub task_queue_size: usize = 100,

    // 任务超时默认时间
    pub default_timeout: Duration = Duration::from_secs(300),
}
```

**资源隔离策略**：

- 每个子代理独立内存空间
- 共享状态通过消息传递
- 禁止跨代理直接调用

### 15. Agent 监控与日志

#### 15.1 日志规范

**必须记录的日志**：

| 事件类型 | 记录内容 | 日志级别 |
|----------|----------|----------|
| Agent 启动/停止 | 启动时间、版本 | INFO |
| 任务分配 | 任务ID、子代理类型、参数 | DEBUG |
| 任务完成 | 任务ID、执行时长、结果摘要 | INFO |
| 任务失败 | 任务ID、错误类型、错误信息 | ERROR |
| 重试 | 任务ID、重试次数、原因 | WARN |
| 熔断触发 | Agent类型、失败次数、时间 | WARN |

**日志格式**：

```json
{
  "timestamp": "ISO8601",
  "level": "INFO",
  "agent": "agent-name",
  "task_id": "task-uuid",
  "event": "event-type",
  "duration_ms": 123,
  "metadata": {}
}
```

#### 15.2 监控指标

**关键性能指标**：

```yaml
监控指标:
  响应时间:
    - P50: < 5s
    - P90: < 30s
    - P99: < 60s

  成功率:
    - 整体成功率: > 95%
    - 单 Agent 成功率: > 90%

  资源使用:
    - CPU 使用率: < 80%
    - 内存使用率: < 70%
    - 并发任务数: 实时监控
```

### 16. Agent 安全管理

#### 16.1 权限控制

```rust
// Agent 权限级别
pub enum AgentPermission {
    ReadOnly,      // 只读（搜索、文档）
    ReadWrite,     // 读写（代码、测试）
    Admin,         // 管理（配置、调度）
}

// Agent 权限矩阵
pub struct PermissionMatrix {
    // Agent -> 允许的操作
    pub search: vec![ReadOnly],
    pub review: vec![ReadOnly],
    pub code: vec![ReadWrite],
    pub test: vec![ReadWrite],
    pub doc: vec![ReadOnly],
    pub master: vec![Admin],
}
```

#### 16.2 敏感操作审计

**必须审计的操作**：

- 文件写入操作
- 代码修改操作
- 配置变更操作
- 外部网络请求
- 数据库修改操作

### 17. Agent 配置文件

#### 17.1 配置文件位置

```
config/
├── agents/
│   ├── master.yaml          # 主代理配置
│   ├── search.yaml           # 搜索代理配置
│   ├── review.yaml           # 审查代理配置
│   ├── code.yaml             # 代码代理配置
│   ├── test.yaml             # 测试代理配置
│   └── doc.yaml              # 文档代理配置
└── agent_config.yaml         # 全局代理配置
```

#### 17.2 配置文件格式

```yaml
# config/agents/master.yaml
agent:
  name: "master"
  version: "1.0.0"
  max_child_agents: 6
  timeout: 600s

scheduler:
  strategy: "priority_fifo"
  max_concurrent: 4
  queue_size: 100

retry:
  max_attempts: 3
  base_delay: 5s
  max_delay: 60s
  exponential_base: 2

circuit_breaker:
  failure_threshold: 5
  recovery_timeout: 60s
```

---

## 第六部分：实施与执行

### 9. 规范执行

#### 9.1 规范培训

- 新成员入职必须阅读本宪法
- 技术负责人负责答疑解惑
- 定期组织规范学习

#### 9.2 工具支持

**代码检查**：
- Rust: `cargo clippy`, `cargo fmt`
- 前端: ESLint, Prettier

**提交检查**：
- Commit Message 格式检查
- 关联 Issue 检查

**测试覆盖**：
- 核心业务单元测试覆盖率 >= 80%
- API 接口测试覆盖

#### 9.3 违规处理

| 违规类型 | 处理方式 |
|----------|----------|
| 代码风格不合规 | 要求整改后方可合并 |
| 安全问题 | 一票否决，立即修复 |
| 测试不通过 | 要求补充测试后方可合并 |
| 文档未同步 | 要求补充文档后方可合并 |

---

## 附录

### 附录A：参考资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Axum 框架文档](https://docs.rs/axum/)
- [Sea-ORM 文档](https://www.sea-ql.org/SeaORM/)
- [Vue 3 文档](https://vuejs.org/)
- [TypeScript 手册](https://www.typescriptlang.org/docs/)

### 附录B：版本历史

| 版本 | 日期 | 变更说明 |
|------|------|----------|
| v1.0.0 | 2026-03-19 | 初始版本发布 |

---

**本宪法自发布之日起正式生效，所有开发人员必须严格遵守。**
