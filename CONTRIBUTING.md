# 贡献指南

感谢你对 ApiHub 的关注！我们欢迎所有形式的贡献。

---

## 📋 行为准则

- 尊重所有贡献者
- 接受建设性批评
- 关注对社区最有利的事情
- 对新手保持耐心和欢迎

---

## 🤔 我可以如何贡献？

### 报告 Bug

如果你发现了 bug，请创建 [Issue](https://github.com/yourusername/apihub/issues)，包含：

1. **清晰的标题**
2. **复现步骤**
3. **预期行为**
4. **实际行为**
5. **环境信息**（操作系统、Rust 版本等）
6. **日志/截图**（如果有）

### 建议新功能

创建 [Issue](https://github.com/yourusername/apihub/issues)，标记为 `enhancement`：

1. **描述功能**
2. **说明为什么需要**
3. **可能的实现方案**（可选）

### 改进文档

文档改进是极好的贡献：

- 修复拼写/语法错误
- 改进说明清晰度
- 添加缺失的文档
- 翻译文档

### 提交代码

---

## 🔧 开发流程

### 1. Fork 并克隆

```bash
git clone https://github.com/YOUR_USERNAME/apihub.git
cd apihub
git remote add upstream https://github.com/yourusername/apihub.git
```

### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
```

分支命名规范：
- `feature/` - 新功能
- `fix/` - bug 修复
- `docs/` - 文档更新
- `refactor/` - 代码重构

### 3. 开发

```bash
# 安装依赖
cd frontend && npm install && cd ..

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 4. 提交

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
feat: 添加用户头像上传功能
fix: 修复积分计算错误
docs: 更新部署文档
style: 格式化代码
refactor: 重构调度器
test: 添加用户测试
chore: 更新依赖
```

### 5. 推送

```bash
git push origin feature/your-feature-name
```

### 6. 创建 Pull Request

1. 访问你 fork 的 GitHub 页面
2. 点击 "New Pull Request"
3. 填写 PR 模板：
   - 变更说明
   - 关联的 Issue
   - 测试说明
   - 截图（如果适用）

---

## ✅ 代码规范

### Rust

```bash
# 格式化
cargo fmt

# 检查
cargo clippy -- -D warnings
```

- 遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- 添加文档注释
- 编写单元测试

### TypeScript/React

```bash
# 格式化
cd frontend && npm run format

# 检查
npm run lint
```

- 使用 TypeScript 严格模式
- 遵循 ESLint 规则
- 组件使用函数式写法

---

## 🧪 测试规范

### 后端测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_user_creation

# 运行并显示输出
cargo test -- --nocapture
```

### 前端测试

```bash
cd frontend
npm run test
```

### 测试覆盖

- 新功能必须包含测试
- Bug 修复应包含回归测试
- 保持测试覆盖率

---

## 📚 文档规范

- 使用清晰的中文/英文
- 代码示例使用代码块
- 保持文档更新

---

## 🔍 Review 流程

1. 至少需要 1 个批准
2. 通过所有 CI 检查
3. 解决所有 review 意见
4. Squash merge 到 main

---

## 🙏 感谢

每一位贡献者都会出现在我们的贡献者列表中。感谢你让 ApiHub 变得更好！

---

*如有问题，请在 [Discussions](https://github.com/yourusername/apihub/discussions) 中提问。*
