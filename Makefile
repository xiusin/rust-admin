.PHONY: help build build-release check clean test lint fmt fmt-check run dev release migrate db-reset docker-build docker-run clean-all info

# 项目名称和版本
PROJECT_NAME := qiluo
CARGO_HOME := ~/.cargo
TARGET_DIR := target
BINARY_NAME := $(PROJECT_NAME)

# 默认目标
.DEFAULT_GOAL := help

## ========== 信息显示 ==========

info:
	@echo "=========================================="
	@echo "  $(PROJECT_NAME) Build System"
	@echo "=========================================="
	@echo "  项目名称:    $(PROJECT_NAME)"
	@echo "  目标目录:    $(TARGET_DIR)"
	@echo "  Rust版本:    $$(rustc --version)"
	@echo "  Cargo版本:   $$(cargo --version)"
	@echo "=========================================="

## ========== 帮助信息 ==========

help: info
	@echo ""
	@echo "  可用命令:"
	@echo ""
	@echo "  开发模式:"
	@echo "    make dev              - 开发模式运行（cargo run）"
	@echo "    make run              - 运行应用程序"
	@echo ""
	@echo "  编译构建:"
	@echo "    make build            - Debug 模式编译"
	@echo "    make build-release    - Release 模式编译（优化）"
	@echo "    make release          - Release 模式编译（优化）"
	@echo ""
	@echo "  代码质量:"
	@echo "    make check            - 代码检查（cargo check）"
	@echo "    make lint             - 代码检查和修复（cargo clippy）"
	@echo "    make fmt              - 代码格式化"
	@echo "    make fmt-check        - 检查代码格式（不修复）"
	@echo ""
	@echo "  测试:"
	@echo "    make test             - 运行所有测试"
	@echo "    make test-verbose     - 运行测试（详细输出）"
	@echo "    make test-release     - Release 模式下测试"
	@echo ""
	@echo "  数据库:"
	@echo "    make migrate          - 运行数据库迁移"
	@echo "    make db-reset         - 重置数据库（删除并重新创建）"
	@echo ""
	@echo "  清理:"
	@echo "    make clean            - 清理编译产物"
	@echo "    make clean-all        - 清理所有（包括 cargo cache）"
	@echo ""
	@echo "  Docker:"
	@echo "    make docker-build     - 构建 Docker 镜像"
	@echo "    make docker-run       - 运行 Docker 容器"
	@echo ""
	@echo "=========================================="

## ========== 开发模式 ==========

### 开发模式运行
dev run:
	@echo "🚀 启动开发模式..."
	cargo run

## ========== 编译构建 ==========

### Debug 模式编译
build:
	@echo "🔨 编译项目 (Debug 模式)..."
	cargo build
	@echo "✅ 编译完成: $(TARGET_DIR)/debug/$(BINARY_NAME)"

### Release 模式编译
build-release release:
	@echo "🔨 编译项目 (Release 模式)..."
	cargo build --release
	@echo "✅ 编译完成: $(TARGET_DIR)/release/$(BINARY_NAME)"

## ========== 代码质量 ==========

### 代码检查
check:
	@echo "🔍 运行代码检查..."
	cargo check
	@echo "✅ 检查完成"

### Clippy 代码检查和修复建议
lint:
	@echo "🔍 运行 Clippy 检查..."
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Lint 完成"

### Clippy 检查（仅报告，不阻止编译）
lint-warn:
	@echo "🔍 运行 Clippy 检查（警告模式）..."
	cargo clippy --all-targets --all-features
	@echo "✅ Lint 完成"

### 代码格式化
fmt:
	@echo "📝 格式化代码..."
	cargo fmt --all
	@echo "✅ 格式化完成"

### 检查代码格式（不修复）
fmt-check:
	@echo "📝 检查代码格式..."
	cargo fmt --all -- --check
	@echo "✅ 格式检查完成"

## ========== 测试 ==========

### 运行所有测试
test:
	@echo "🧪 运行测试..."
	cargo test --all
	@echo "✅ 测试完成"

### 运行测试（详细输出）
test-verbose:
	@echo "🧪 运行测试（详细模式）..."
	cargo test --all -- --nocapture --test-threads=1
	@echo "✅ 测试完成"

### 运行单个测试
test-single:
	@echo "🧪 运行单个测试..."
	cargo test $(TEST_NAME)
	@echo "✅ 测试完成"

### Release 模式下测试
test-release:
	@echo "🧪 运行测试 (Release 模式)..."
	cargo test --release --all
	@echo "✅ 测试完成"

### 运行文档测试
test-doc:
	@echo "🧪 运行文档测试..."
	cargo test --doc
	@echo "✅ 文档测试完成"

## ========== 数据库 ==========

### 运行数据库迁移
migrate:
	@echo "📦 运行数据库迁移..."
	cd migration && cargo run
	@echo "✅ 迁移完成"

### 重置数据库
db-reset:
	@echo "⚠️  准备重置数据库..."
	@echo "请手动执行以下步骤:"
	@echo "  1. 删除现有数据库"
	@echo "  2. 创建新数据库"
	@echo "  3. 运行: make migrate"

## ========== 清理 ==========

### 清理编译产物
clean:
	@echo "🧹 清理编译产物..."
	cargo clean
	@echo "✅ 清理完成"

### 清理所有（包括 cargo cache）
clean-all:
	@echo "🧹 清理所有（包括依赖缓存）..."
	rm -rf $(TARGET_DIR)
	@echo "✅ 清理完成"

## ========== Docker ==========

### 构建 Docker 镜像
docker-build:
	@echo "🐳 构建 Docker 镜像..."
	docker build -t $(PROJECT_NAME):latest .
	@echo "✅ Docker 镜像构建完成: $(PROJECT_NAME):latest"

### 构建 Docker 镜像（多平台）
docker-build-multi:
	@echo "🐳 构建 Docker 镜像（多平台）..."
	docker buildx build --platform linux/amd64,linux/arm64 -t $(PROJECT_NAME):latest --push .
	@echo "✅ Docker 镜像构建完成"

### 运行 Docker 容器
docker-run:
	@echo "🐳 启动 Docker 容器..."
	docker run -d --name $(PROJECT_NAME) -p 8080:8080 $(PROJECT_NAME):latest
	@echo "✅ 容器已启动: $(PROJECT_NAME)"

### 停止 Docker 容器
docker-stop:
	@echo "🛑 停止 Docker 容器..."
	docker stop $(PROJECT_NAME) || true
	docker rm $(PROJECT_NAME) || true
	@echo "✅ 容器已停止"

### 查看 Docker 容器日志
docker-logs:
	docker logs -f $(PROJECT_NAME)

## ========== 依赖管理 ==========

### 更新依赖
update:
	@echo "📜 更新依赖..."
	cargo update
	@echo "✅ 依赖更新完成"

### 添加依赖
add:
	@echo "📜 添加依赖..."
	@if [ -z "$(DEP)" ]; then \
		echo "❌ 请指定依赖名称: make add DEP=crate_name"; \
	else \
		cargo add $(DEP); \
		echo "✅ 依赖 $(DEP) 添加完成"; \
	fi

### 移除依赖
remove:
	@echo "📜 移除依赖..."
	@if [ -z "$(DEP)" ]; then \
		echo "❌ 请指定依赖名称: make remove DEP=crate_name"; \
	else \
		cargo remove $(DEP); \
		echo "✅ 依赖 $(DEP) 移除完成"; \
	fi

### 锁定依赖版本
lock:
	@echo "📜 锁定依赖版本..."
	cargo generate-lockfile
	@echo "✅ 依赖版本已锁定"

## ========== 文档 ==========

### 生成文档
doc:
	@echo "📚 生成文档..."
	cargo doc --no-deps
	@echo "✅ 文档生成完成"

### 打开文档（浏览器）
doc-open:
	@echo "📚 在浏览器中打开文档..."
	cargo doc --no-deps --open
	@echo "✅ 文档已打开"

## ========== 高级 ==========

### 仅构建库
build-lib:
	@echo "🔨 编译库..."
	cargo build --lib
	@echo "✅ 库编译完成"

### 仅构建指定包
build-package:
	@echo "🔨 编译指定包..."
	cargo build --package $(PACKAGE)
	@echo "✅ 包编译完成"

### 分析依赖
tree:
	@echo "📦 显示依赖树..."
	cargo tree
	@echo "✅ 依赖树显示完成"

### 分析包大小
size:
	@echo "📊 分析二进制文件大小..."
	@if [ -f "$(TARGET_DIR)/release/$(BINARY_NAME)" ]; then \
		size $(TARGET_DIR)/release/$(BINARY_NAME); \
	else \
		echo "❌ 未找到 release 二进制文件，请先运行 make build-release"; \
	fi

### 生成 shell 补全脚本
completion-bash:
	@echo "📜 生成 Bash 补全脚本..."
	cargo generate-lockfile --print-hash > /dev/null 2>&1 || true
	@echo "source <(cargo completions bash)" >> ~/.bashrc
	@echo "✅ 请执行: source ~/.bashrc"

completion-zsh:
	@echo "📜 生成 Zsh 补全脚本..."
	mkdir -p ~/.zsh/completion
	cargo completions zsh > ~/.zsh/completion/_cargo
	@echo "✅ 补全脚本已生成到 ~/.zsh/completion/_cargo"

## ========== CI/CD ==========

### CI 检查（用于 GitHub Actions 等）
ci-check:
	@echo "🔍 CI 检查..."
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all
	@echo "✅ CI 检查完成"

### 发布检查
publish-check:
	@echo "🔍 发布前检查..."
	cargo publish --dry-run --allow-dirty
	@echo "✅ 发布检查完成"

## ========== 安装 ==========

### 安装二进制文件到 ~/.cargo/bin
install:
	@echo "📦 安装二进制文件..."
	cargo install --path .
	@echo "✅ 安装完成"
	@echo "请确保 ~/.cargo/bin 在你的 PATH 中"

### 卸载二进制文件
uninstall:
	@echo "🗑️  卸载二进制文件..."
	cargo uninstall $(PROJECT_NAME)
	@echo "✅ 卸载完成"
