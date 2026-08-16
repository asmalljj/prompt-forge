# PromptForge

提示词优化工具 — 输入大白话，输出 AI 更能理解的高质量提示词。

有好的功能建议请发送邮箱至：653595478@qq.com

## 功能特性

- ✨ **一键优化**：调用 DeepSeek API，把大白话提示词优化成高质量提示词
- 🎨 **三种优化风格**：通用 / 简洁 / 详细，按需选择
- 📋 **一键复制**：复制优化结果到剪贴板
- 🖥️ **苹果风格 UI**：极简设计，大量留白，微动画交互

## 技术栈

| 层级    | 技术                                             |
| ----- | ---------------------------------------------- |
| 桌面框架  | [Tauri](https://tauri.app/) v2                 |
| 前端框架  | [Vue 3](https://vuejs.org/) + TypeScript       |
| 构建工具  | Vite                                           |
| AI 后端 | [DeepSeek API](https://platform.deepseek.com/) |

## 环境要求

- [Rust](https://www.rust-lang.org/) ≥ 1.85
- [Node.js](https://nodejs.org/) ≥ 18
- Windows: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

## 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 配置 API Key

复制 `.env.example` 为 `.env`，填入你的 DeepSeek API Key（在 [platform.deepseek.com](https://platform.deepseek.com/) 申请）：

```bash
DEEPSEEK_API_KEY=your_api_key_here
DEEPSEEK_BASE_URL=https://api.deepseek.com
DEEPSEEK_MODEL=deepseek-v4-flash
```

### 3. 启动开发模式

```bash
npm run tauri dev
```

### 4. 构建安装包

```bash
npm run tauri build
```

构建产物在 `src-tauri/target/release/bundle/` 目录。

## 使用说明

1. 在输入框输入你的大白话提示词（例如：`帮我写一个介绍 Vue 的文案`）
2. 选择优化风格（通用 / 简洁 / 详细）
3. 点击 **「优化提示词」** 按钮
4. 等待优化结果，点击 **「复制」** 复制到剪贴板

## 项目结构

```
prompt-forge/
├── src-tauri/                    # Tauri 后端（Rust）
│   ├── src/
│   │   ├── main.rs              # 入口文件
│   │   ├── lib.rs               # 核心逻辑（Builder 配置、命令注册）
│   │   ├── commands/            # 命令分组（路由分组）
│   │   │   ├── health.rs        # 健康检查接口
│   │   │   ├── optimize.rs      # 提示词优化接口
│   │   │   └── hello.rs         # 示例命令
│   │   ├── services/            # 业务逻辑层
│   │   │   └── deepseek.rs      # DeepSeek API 调用
│   │   ├── config.rs            # 环境变量配置管理
│   │   ├── error.rs             # 统一错误处理 + 统一响应格式
│   │   └── state.rs             # 应用状态管理
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/
│       └── default.json         # 权限能力配置
├── src/                          # Vue 前端
│   ├── components/              # 通用组件
│   │   ├── PromptInput.vue      # 输入框组件
│   │   ├── StyleSelector.vue    # 风格选择组件
│   │   └── ResultDisplay.vue    # 结果展示组件
│   ├── styles/                  # 全局样式
│   │   └── global.css           # 苹果风格 CSS 变量
│   ├── App.vue
│   └── main.ts
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 开发说明

- 日志输出到 `logs/` 目录（`tauri-plugin-log`）
- API Key 通过 `.env` 文件配置，不硬编码在代码中
- 所有命令统一返回 `ApiResponse<T>` 格式
- 代码规范：ESLint + Prettier（`npm run lint` / `npm run format`）

## License

MIT
