# HTML PPT Editor

基于 **Tauri 2 + Rust + WebView2** 的 HTML 演示稿可视化编辑器。直接打开磁盘上的 HTML 演示稿目录，在真实渲染效果上点选、编辑、增删元素，改完写回原文件。

> 版本：v0.1.0 ｜ 平台：Windows（macOS 需在 macOS 上自行构建）

## 功能

### 载入

- **打开文件夹**：选择演示稿目录，自动定位入口 HTML 与 `assets/` 资源，图片、样式、脚本按原相对路径解析
- **打开 HTML**：只载入单个 HTML 文件
- **补充资源**：把目录之外的图片目录挂进来
- **拖拽打开**：文件或文件夹直接拖进窗口
- 载入后左侧列出全部幻灯片缩略图与文件树，点击即可跳页

### 可视化编辑

- **点选元素**：预览区内悬停高亮、单击选中，选框跟随滚动与缩放
- **文本编辑**：双击进入原地编辑，右侧面板可改字号、颜色、行高、对齐等
- **尺寸与样式**：宽度、高度、内边距、外边距、背景、边框，写入内联样式并带 `!important` 覆盖模板
- **删除元素**：`Delete` / `Backspace`，或右侧面板「删除元素」
- **图片替换**：选中图片后重新选图，或以 data URL 内联插入
- **插入元素**：文本块、图片、列表、新幻灯片
  - 有选中元素时插入到该元素内部，未选中时插入到当前页
- **撤销 / 重做**：完整快照栈，`Ctrl+Z` / `Ctrl+Shift+Z`；撤销后停留在原页

### 源码与导出

- **源码**：打开完整 HTML 源码编辑，保存后重新渲染；打开时自动定位到当前页
- **打印**：调用系统打印预览，一张幻灯片一页，按画布原始尺寸出图，不裁切、不丢线条
- **保存 / 另存为**：写回原文件或导出副本，导出的 HTML 图片自动还原为相对路径，可离线分发

### 桌面集成

- 原生菜单：文件（打开文件夹 / 打开 HTML / 保存 / 另存为 / 退出）、编辑（撤销 / 重做 / 全选）
- 原生文件夹、文件、保存对话框
- **退出保护**：有未保存改动时提示「保存并退出 / 不保存 / 取消」

## 下载

| 安装包 | 体积 | 适用场景 |
|---|---|---|
| `html-ppt-editor_0.1.0_x64-online-setup.exe` | ~4.5 MB | 联网环境。安装时检测系统 WebView2，缺失则自动下载安装 |
| `html-ppt-editor_0.1.0_x64-offline-setup.exe` | ~210 MB | 内网 / 离线环境。自带 WebView2 运行时，同样先检测再用，已安装则跳过 |

两者安装时都会**先检测系统是否已安装 WebView2**，已安装则直接装本软件；WebView2 始终安装到系统公共目录，可被其他程序复用。

便携版：`src-tauri/target/release/html-ppt-editor.exe`（构建后生成，约 5.7 MB，依赖系统已安装的 WebView2）

## 开发

环境要求：Node.js 18+、Rust 1.99.1（由 `src-tauri/rust-toolchain.toml` 固定）、WebView2 Runtime。

```bash
npm install
npm run tauri dev
```

### 打包

```bash
npm run build:online    # 只出在线包
npm run build:offline   # 只出离线包
npm run build:all       # 同时出两个（推荐）
```

产物统一输出到 `src-tauri/target/release/bundle/nsis/`，并自动重命名为 `-online-setup.exe` / `-offline-setup.exe`。

两种模式的差异只在 `webviewInstallMode`：

| 配置文件 | 模式 | 说明 |
|---|---|---|
| `src-tauri/tauri.conf.json` | `offlineInstaller` | 内嵌完整 WebView2 运行时（205 MB） |
| `src-tauri/tauri.conf.online.json` | `embedBootstrapper` | 内嵌 1.8 MB 引导程序，安装时在线下载 |

## 目录结构

```
html-ppt-editor/
├── src/index.html              # 编辑器前端（单文件，含全部 UI 与逻辑）
├── src-tauri/
│   ├── src/lib.rs              # Rust 后端：文件读写命令 + 原生菜单
│   ├── capabilities/           # 权限配置
│   ├── tauri.conf.json         # 离线包配置
│   ├── tauri.conf.online.json  # 在线包配置
│   └── rust-toolchain.toml     # 固定 Rust 版本
├── build-all.ps1               # 双模式打包脚本
└── THIRD-PARTY-NOTICES.md      # 第三方组件版权申明
```

## 技术要点

- 预览使用 `iframe` 加载原始演示稿，编辑器注入的选框、标记均带 `data-ved` 属性，序列化时自动剔除，不会污染导出文件
- 动态生成的相对路径图片（如 `data-logo`）在渲染后改写为 blob URL，导出时还原
- 演示稿运行时通过方向键事件驱动翻页，重渲染后用 `#/N` 深链接恢复所在页

## 第三方组件

本项目使用的开源组件及其许可证见 [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md)。
