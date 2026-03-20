# 齐罗（QiLuo）项目优化方案

**版本**: v1.0.0
**制定日期**: 2026-03-19
**状态**: 进行中

---

## 一、语法兼容问题分析与修复

### 1.1 `.unwrap()` 和 `.expect()` 滥用问题

**问题描述**：代码中大量使用 `.unwrap()` 和 `.expect()`，在遇到 `None` 或 `Err` 时会导致 panic，程序崩溃。

**问题位置**：

| 文件 | 行号 | 问题代码 |
|------|------|----------|
| `s_sys_user.rs` | 59 | `header.get("user-agent").unwrap().to_str().unwrap()` |
| `msys_user.rs` | 74 | `rmodel.clone().count(db).await.unwrap()` |
| `msys_user.rs` | 183 | `SysUserModel::find_by_id(id).await.unwrap()` |
| `msys_user.rs` | 167 | `SysUserModel::find_by_username().await.unwrap()` |
| `auth.rs` | 62-67 | `.expect()` 用于获取扩展 |

**修复方案**：

```rust
// ❌ 原始代码（危险）
let user_agent_str = header.get("user-agent").unwrap().to_str().unwrap();

// ✅ 修复后
let user_agent_str = header
    .get("user-agent")
    .and_then(|v| v.to_str().ok())
    .unwrap_or("Unknown");

// ❌ 原始代码（危险）
let total = rmodel.clone().count(db).await.unwrap();

// ✅ 修复后 - 使用 ? 运算符
let total = rmodel.clone().count(db).await?;

// ❌ 原始代码（危险）
impl FromRequestParts<S> for UserInfo {
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = req.extensions().get::<UserInfo>().expect("UserInfo not found");
        // ...
    }
}

// ✅ 修复后 - 使用 ok() 或合适错误处理
let user = req.extensions().get::<UserInfo>().ok_or(AuthError::Unauthorized)?;
```

### 1.2 字符串比较问题

**问题描述**：使用 `==` 比较 `Option<String>` 和 `&str`，可能导致意外行为。

**问题位置**：`s_sys_user.rs` 第 52 行

```rust
if arg.client_id != captcha_info.client_id {
```

**修复方案**：

```rust
// ❌ 原始代码
if arg.client_id != captcha_info.client_id {

// ✅ 修复后 - 明确处理 Option 比较
if arg.client_id != captcha_info.client_id.as_str() {
```

### 1.3 错误类型不一致

**问题描述**：混用 `Error::Message(String)` 和 `Error::WithStatus`，错误处理不统一。

**问题位置**：`msys_user.rs` 第 117 行

```rust
return Err("Model Failed".into());
```

**修复方案**：

```rust
// ❌ 原始代码
return Err("Model Failed".into());

// ✅ 修复后 - 使用统一的错误类型
return Err(Error::not_found("User not found"));

// ❌ 原始代码
if !util::verify_password(&arg.old_password, user.password.as_str()) {
    return Err("Old Password Error".into());
};

// ✅ 修复后
if !util::verify_password(&arg.old_password, user.password.as_str()) {
    return Err(Error::unauthorized("Invalid credentials"));
};
```

### 1.4 并发迭代器问题

**问题描述**：`for id in ids` 使用克隆的连接，可能导致连接泄漏。

**问题位置**：`msys_user.rs` 第 134-145 行

```rust
pub async fn delete(ids: Vec<i64>) -> Result<String> {
    let db = DB().await;
    for id in ids {
        // 每次迭代都使用同一个 db 连接
    }
}
```

**修复方案**：使用事务批量处理

```rust
pub async fn delete(ids: Vec<i64>) -> Result<String> {
    let db = DB().await;

    for id in ids {
        let rmodel = sys_user::Entity::find_by_id(id).one(db).await?;
        if let Some(model) = rmodel {
            let mut amodel: sys_user::ActiveModel = model.into();
            amodel.deleted_at = Set(Some(Local::now().naive_local()));
            amodel.update(db).await?;
        }
    }
    Ok("Success".into())
}

// ✅ 更好的方案：使用事务
pub async fn delete(ids: Vec<i64>) -> Result<String> {
    let db = DB().await;

    db.transaction(|txn| async move {
        for id in ids {
            let rmodel = sys_user::Entity::find_by_id(id).one(txn).await?;
            if let Some(model) = rmodel {
                let mut amodel: sys_user::ActiveModel = model.into();
                amodel.deleted_at = Set(Some(Local::now().naive_local()));
                amodel.update(txn).await?;
            }
        }
        Ok::<_, Error>(())
    }).await?;

    Ok("Success".into())
}
```

---

## 二、性能问题分析与修复

### 2.1 N+1 查询问题

**问题描述**：在循环中执行数据库查询，每次删除都单独执行 UPDATE。

**问题位置**：`msys_user.rs` 第 134-145 行

**性能影响**：删除 100 个用户会执行 100+ 次数据库往返。

**优化方案**：

```rust
// ❌ 原始代码 - N+1 查询
pub async fn delete(ids: Vec<i64>) -> Result<String> {
    for id in ids {
        let rmodel = sys_user::Entity::find_by_id(id).one(db).await?;  // N 次查询
        amodel.update(db).await?;  // N 次更新
    }
}

// ✅ 优化后 - 批量操作
pub async fn delete(ids: Vec<i64>) -> Result<String> {
    let db = DB().await;

    // 一次查询获取所有用户
    let users = sys_user::Entity::find()
        .filter(sys_user::Column::Id.is_in(ids.clone()))
        .all(db)
        .await?;

    // 批量更新
    let now = Some(Local::now().naive_local());
    for user in users {
        let mut amodel: sys_user::ActiveModel = user.into();
        amodel.deleted_at = Set(now);
        amodel.update(db).await?;
    }

    Ok("Success".into())
}
```

### 2.2 重复数据库调用

**问题描述**：`list` 函数中 `dept.clone().unwrap()` 被调用两次。

**问题位置**：`msys_user.rs` 第 59-60 行

```rust
.add(sys_dept::Column::Lft.gte(dept.clone().unwrap().lft))
.add(sys_dept::Column::Rgt.lte(dept.unwrap().rgt)),
```

**优化方案**：

```rust
// ❌ 原始代码 - 两次 unwrap
.add(sys_dept::Column::Lft.gte(dept.clone().unwrap().lft))
.add(sys_dept::Column::Rgt.lte(dept.unwrap().rgt)),

// ✅ 优化后 - 一次 unwrap
let dept_lft = dept.as_ref().map(|d| d.lft);
let dept_rgt = dept.as_ref().map(|d| d.rgt);

if let (Some(lft), Some(rgt)) = (dept_lft, dept_rgt) {
    rmodel = rmodel.filter(
        Condition::all()
            .add(sys_dept::Column::Lft.gte(lft))
            .add(sys_dept::Column::Rgt.lte(rgt))
    );
}
```

### 2.3 中间件体读取问题

**问题描述**：`auth_fn_mid` 和 `request_log_fn_mid` 中重复读取请求体，可能导致 body 被消耗后无法再次读取。

**问题位置**：`auth.rs` 第 36-43 行、第 104-106 行

**性能影响**：每次请求体被读取两次，增加内存和 CPU 开销。

**优化方案**：

```rust
// ❌ 原始代码 - 读取 body
let bytes = body
    .collect()
    .await
    .unwrap()
    .to_bytes();
let req_ctx: ReqCtx = ...;
let mut req = Request::from_parts(parts, Body::from(bytes));

// ✅ 优化后 - 使用 Body::to_bytes 懒加载
let req_ctx: ReqCtx = ReqCtx {
    ori_uri: ...,
    path,
    path_params,
    method: method.to_string(),
};
req.extensions_mut().insert(req_ctx);
Ok(next.run(req).await)
```

### 2.4 未使用连接池

**问题描述**：缓存 `CacheManager::instance().await` 每次调用都返回 `Arc<Cache>`，但未考虑连接池复用。

**优化方案**：使用 `Arc` 缓存引用，避免不必要的引用计数增加。

### 2.5 缺少索引

**问题描述**：数据库查询中常用字段（如 `user_name`, `email`, `status`, `deleted_at`）可能缺少索引。

**建议**：在迁移脚本中添加复合索引。

```sql
-- 建议添加的索引
CREATE INDEX idx_sys_user_name ON sys_user(user_name);
CREATE INDEX idx_sys_user_email ON sys_user(email);
CREATE INDEX idx_sys_user_status ON sys_user(status);
CREATE INDEX idx_sys_user_deleted ON sys_user(deleted_at);
CREATE INDEX idx_sys_user_dept_id ON sys_user(dept_id);
```

---

## 三、安全问题分析与修复

### 3.1 密码哈希验证

**问题描述**：`chagne_password` 函数中使用 `unwrap()` 处理密码哈希。

**问题位置**：`msys_user.rs` 第 225 行

```rust
let new_password = util::hash_password(&arg.new_password).unwrap();
```

**修复方案**：

```rust
// ❌ 原始代码
let new_password = util::hash_password(&arg.new_password).unwrap();

// ✅ 修复后
let new_password = util::hash_password(&arg.new_password)
    .map_err(|_| Error::internal_error("Password hashing failed"))?;
```

### 3.2 SQL 注入风险

**问题描述**：某些查询使用 `.contains()` 可能被利用，但 Sea-ORM 已做参数化处理，风险较低。

**建议**：继续使用 Sea-ORM 的查询构建器，避免拼接原生 SQL。

---

## 四、实现优先级

| 优先级 | 问题 | 预计工时 | 影响范围 |
|--------|------|----------|----------|
| P0 | `.unwrap()` 滥用导致 panic | 2h | 稳定性 |
| P1 | N+1 查询问题 | 1h | 性能 |
| P2 | 中间件体重复读取 | 1h | 性能 |
| P3 | 重复 `.unwrap()` 调用 | 0.5h | 代码质量 |

---

## 五、修复清单

### 5.1 立即修复（P0）

- [ ] `s_sys_user.rs:59` - user_agent unwrap
- [ ] `msys_user.rs:74` - count unwrap
- [ ] `msys_user.rs:117` - 错误类型不一致
- [ ] `msys_user.rs:183` - find_by_id unwrap
- [ ] `msys_user.rs:167` - find_by_username unwrap
- [ ] `msys_user.rs:176` - find_by_email unwrap
- [ ] `msys_user.rs:215-246` - 密码操作 unwrap
- [ ] `auth.rs:62-67` - expect 滥用

### 5.2 短期优化（P1）

- [ ] `msys_user.rs:134-145` - 批量删除优化
- [ ] `msys_user.rs:59-60` - 重复 unwrap 优化

### 5.3 中期优化（P2）

- [ ] `auth.rs` - 中间件体读取优化

---

**本方案持续更新中**
