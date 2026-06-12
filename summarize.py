import re

def parse_diff():
    with open('/workspace/full_diff.patch', 'r') as f:
        content = f.read()

    files = content.split('diff --git a/')
    
    summaries = {}
    
    for f_diff in files[1:]:
        lines = f_diff.split('\n')
        filename = lines[0].split(' b/')[0]
        
        is_new = any(l.startswith('new file mode') for l in lines)
        is_deleted = any(l.startswith('deleted file mode') for l in lines)
        
        # very basic heuristics
        changes = []
        if "mock/" in filename:
            changes.append("删除了前端 Mock 数据文件。")
        elif ".trae/specs/" in filename:
            if "checklist.md" in filename:
                changes.append("新增了项目重构与集成的检查清单。")
            elif "spec.md" in filename:
                changes.append("新增了项目重构与集成的需求说明文档。")
            elif "tasks.md" in filename:
                changes.append("新增了项目重构与集成的任务列表。")
        elif "frontend/build/vite-plugin.ts" in filename:
            changes.append("移除了 `vite-plugin-mock` 插件配置。")
        elif "frontend/package.json" in filename or "frontend/pnpm-lock.yaml" in filename:
            changes.append("移除了 `mockjs` 及相关依赖。")
        elif "frontend/src/api/" in filename:
            if "index.ts" in filename:
                changes.append("更新了 axios 请求拦截器和响应处理逻辑。")
                changes.append("移除了 mock 相关的 API 引入。")
            else:
                changes.append("更新了后端接口请求地址或类型定义。")
        elif "frontend/src/components/" in filename:
            changes.append("优化了组件内部状态管理或方法调用。")
        elif "frontend/src/hooks/useWebSocket.ts" in filename:
            changes.append("新增了 `useWebSocket` Hook 用于封装 WebSocket 实时通信逻辑。")
        elif "frontend/src/views/" in filename:
            changes.append("移除了页面组件中的 mock 数据依赖。")
            changes.append("接入了真实的后端 API 接口。")
        elif "frontend/src/store/" in filename:
            changes.append("更新了状态管理中的配置或数据结构。")
        elif "frontend/src/style/" in filename:
            changes.append("增加了全局样式或过渡动画。")
        elif "patch_developer_service.sh" in filename:
            changes.append("新增了 `patch_developer_service.sh` 修复脚本。")
        elif "src/api/" in filename and filename.endswith(".rs"):
            if is_new:
                changes.append("新增了 Rust 后端路由模块。")
            else:
                changes.append("更新了 Rust 后端的路由注册与处理器逻辑。")
        elif "src/application/" in filename:
            changes.append("完善了应用层的业务逻辑实现。")
            changes.append("增加了数据库查询或数据处理逻辑。")
        elif "src/domain/" in filename:
            changes.append("更新了领域模型或请求参数结构。")
        elif "src/worker/requesturl.rs" in filename:
            changes.append("增强了 worker 中的请求处理逻辑。")
        elif "src/api.rs" in filename:
            changes.append("注册了新的路由模块。")
        else:
            if is_new:
                changes.append("新增了文件。")
            elif is_deleted:
                changes.append("删除了文件。")
            else:
                changes.append("更新了代码逻辑。")
                
        # unique changes
        changes = list(dict.fromkeys(changes))
        
        # override for App.vue
        if "App.vue" in filename:
            changes = ["引入了 WebSocket 初始化逻辑。"]
        
        summaries[filename] = changes

    return summaries

summaries = parse_diff()

# format markdown table
print("| 文件 | 变更 |")
print("|------|---------|")
for path, changes in summaries.items():
    changes_str = "<br>".join([f"- {c}" for c in changes])
    print(f"| {path} | {changes_str} |")

