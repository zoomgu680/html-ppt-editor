# 第三方组件版权申明

HTML PPT Editor（以下简称"本软件"）在开发、构建与分发过程中使用了以下第三方组件。本文件列出各组件的名称、版本、许可证类型与版权归属，以履行相应的署名与许可证声明义务。

---

## 一、运行时组件

### 1. Tauri

- 版本：2.11.6
- 许可证：MIT OR Apache-2.0
- 版权：Copyright © 2017 - 2024 Tauri Contributors
- 项目地址：https://github.com/tauri-apps/tauri

Tauri 提供跨平台桌面应用框架，负责窗口管理、IPC 通信与本软件的全部原生能力桥接。

### 2. tauri-plugin-dialog

- 版本：2.7.3
- 许可证：MIT OR Apache-2.0
- 版权：Copyright © 2017 - 2024 Tauri Contributors
- 项目地址：https://github.com/tauri-apps/plugins-workspace

提供原生文件夹选择、文件打开与另存为对话框。

### 3. tauri-plugin-opener

- 版本：2.5.5
- 许可证：MIT OR Apache-2.0
- 版权：Copyright © 2017 - 2024 Tauri Contributors
- 项目地址：https://github.com/tauri-apps/plugins-workspace

提供以系统默认程序打开文件或链接的能力。

### 4. Microsoft Edge WebView2 Runtime

- 许可证：Microsoft 软件许可条款（专有，可再分发）
- 版权：Copyright © Microsoft Corporation
- 项目地址：https://developer.microsoft.com/microsoft-edge/webview2/

本软件界面基于 WebView2 渲染。离线安装包内嵌官方 Evergreen Standalone Installer，安装时按需部署到系统公共目录；在线安装包在检测到系统缺失时自动下载官方引导程序安装。WebView2 的安装与使用受 Microsoft 相应许可条款约束。

---

## 二、Rust 依赖库

以下库均以 MIT 或 Apache-2.0 双许可发布，本软件按 MIT 条款使用。完整许可证文本见各项目仓库。

| 组件 | 版本 | 许可证 | 用途 |
|---|---|---|---|
| serde | 1.0.229 | MIT OR Apache-2.0 | 序列化 / 反序列化框架 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | JSON 处理 |
| base64 | 0.22.1 | MIT OR Apache-2.0 | 二进制与 base64 互转 |
| walkdir | 2.5.0 | MIT OR Unlicense | 递归目录遍历 |
| rfd | 0.16.1 | MIT | 原生文件对话框（备用实现） |
| open | 5.4.5 | MIT | 调用系统默认程序打开路径 |
| windows-sys | 0.60.2 | MIT OR Apache-2.0 | Windows 注册表与系统 API 绑定 |

### 传递依赖

Tauri 及其插件通过 Cargo 引入大量传递依赖（如 `wry`、`tao`、`webkit2gtk`（Linux）、`windows`、`tokio`、`serde`、`syn`、`quote`、`proc-macro2` 等）。这些组件绝大多数以 **MIT OR Apache-2.0** 许可发布，少量采用 BSD-3-Clause、ISC、Unlicense 或 Zlib 许可。完整清单可通过以下命令生成：

```bash
cargo install cargo-license
cd src-tauri && cargo license
```

---

## 三、构建与打包工具

### 1. NSIS（Nullsoft Scriptable Install System）

- 版本：由 tauri-bundler 自动下载的 3.x 系列
- 许可证：Zlib / libpng
- 版权：Copyright © 1999-2024 Contributors to the NSIS project
- 项目地址：https://nsis.sourceforge.io/

用于生成 Windows 安装包。NSIS 自身的代码不会被嵌入本软件，仅在安装包头部包含其运行时代码（按其许可证允许的方式）。

### 2. @tauri-apps/cli

- 版本：2.12.4
- 许可证：MIT OR Apache-2.0
- 版权：Copyright © 2017 - 2024 Tauri Contributors

仅作为开发依赖用于构建，不随软件分发。

### 3. Rust 工具链

- 版本：1.99.1
- 许可证：MIT OR Apache-2.0
- 版权：Copyright © The Rust Project Developers

---

## 四、前端与模板说明

本软件前端（`src/index.html`）为原生 HTML / CSS / JavaScript 实现，**不包含任何第三方前端框架或 UI 组件库**。

用户通过本软件打开、编辑的 HTML 演示稿（含其中引用的 `reveal.js`、字体、图标、图片等资源）版权归演示稿原作者或其提供方所有，不属于本软件的组成部分，本软件不对其主张任何权利，也不对其许可证合规性负责。

---

## 五、许可证全文

MIT License 与 Apache License 2.0 的完整文本可从以下地址获取：

- MIT：https://opensource.org/licenses/MIT
- Apache-2.0：https://www.apache.org/licenses/LICENSE-2.0
- Unlicense：https://unlicense.org/
- Zlib：https://opensource.org/licenses/Zlib

---

如对本申明有疑问，或认为本软件的使用侵犯了您的权利，请通过项目 Issue 联系维护者。
