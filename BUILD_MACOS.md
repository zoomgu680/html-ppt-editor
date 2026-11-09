# macOS 构建指南

本文档说明如何在 macOS 上构建 **HTML PPT Editor** 桌面应用。

> 本项目的核心代码（Rust 后端 + 前端编辑器）是跨平台的，macOS 构建与 Windows 的主要差异在于：打包目标从 NSIS 变为 DMG、WebView 改用系统内置的 WKWebView、以及 Apple 平台特有的签名与公证流程。

---

## 一、系统与环境要求

| 项目 | 要求 |
|---|---|
| 操作系统 | macOS 10.15 (Catalina) 及以上（推荐 macOS 12+） |
| 架构 | Intel (x86_64) 或 Apple Silicon (arm64)，均可构建 |
| Xcode Command Line Tools | 必需（提供 clang、make、SDK） |
| Rust | 1.99.1（由 `rust-toolchain.toml` 自动锁定） |
| Node.js | 18+（推荐 LTS） |
| 磁盘空间 | 至少 5 GB（含 cargo 依赖与构建产物） |

### 1.1 安装 Xcode Command Line Tools

```bash
xcode-select --install
```

若已安装完整 Xcode，确保已接受许可协议：

```bash
sudo xcodebuild -license accept
```

### 1.2 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后重新打开终端，`rust-toolchain.toml` 会在首次构建时自动下载并切换到 1.99.1。

### 1.3 安装 Node.js

推荐使用 [nvm](https://github.com/nvm-sh/nvm)：

```bash
nvm install --lts
nvm use --lts
```

或从 [Node.js 官网](https://nodejs.org/) 安装。

---

## 二、快速构建（本地测试）

```bash
cd html-ppt-editor

# 1. 安装前端依赖
npm install

# 2. 构建 macOS 安装包（DMG）
npm run build:macos
# 等价于：npx tauri build --config src-tauri/tauri.conf.macos.json
```

产物位置：

```
src-tauri/target/release/bundle/dmg/html-ppt-editor_0.1.0_aarch64.dmg
```

双击 dmg 即可安装。首次运行时，若系统提示「无法打开，因为无法验证开发者」，右键 → 打开，或在「系统设置 → 隐私与安全性」中允许。

### 使用构建脚本（推荐）

项目提供了 `build-macos.sh`，支持指定架构：

```bash
chmod +x build-macos.sh

./build-macos.sh              # 当前架构
./build-macos.sh universal    # 通用二进制（Intel + Apple Silicon）
./build-macos.sh arm64        # 仅 Apple Silicon
./build-macos.sh x86_64       # 仅 Intel
```

---

## 三、配置说明

### 3.1 `tauri.conf.macos.json`

macOS 构建使用独立配置文件，与 Windows 的 `tauri.conf.json` 隔离：

| 字段 | 值 | 说明 |
|---|---|---|
| `bundle.targets` | `["dmg"]` | macOS 安装包格式。也可改为 `["app"]` 只生成 `.app` |
| `bundle.icon` | 含 `icon.icns` | macOS 应用图标，**必须存在**，已提供 |
| `bundle.macOS.signingIdentity` | `null` | 本地构建为 ad-hoc 签名；分发时填入 Developer ID |
| `bundle.macOS.minimumSystemVersion` | `"10.15"` | 最低系统版本 |
| `bundle.windows.webviewInstallMode` | 无 | macOS 不需要，该字段被忽略 |

> **不需要 WebView2**：macOS 应用使用系统内置的 WKWebView，无需随安装包分发任何 WebView 运行时。`lib.rs` 中检测 WebView2 的代码已用 `#[cfg(target_os = "windows")]` 隔离，在 macOS 上不会编译。

### 3.2 Rust 平台条件编译

`Cargo.toml` 中 `windows-sys` 仅在 Windows 上引入：

```toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.60", features = ["Win32_System_Registry", "Win32_Foundation"] }
```

其余依赖（tauri、serde、walkdir、rfd、open）均为跨平台，无需改动。

---

## 四、跨架构构建

### 4.1 构建当前架构

```bash
npm run build:macos
```

### 4.2 构建通用二进制（Universal）

通用二进制可同时在 Intel 和 Apple Silicon 上原生运行，体积约为单架构的 2 倍。

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npx tauri build --target universal-apple-darwin --config src-tauri/tauri.conf.macos.json
```

或用脚本：

```bash
./build-macos.sh universal
```

### 4.3 单独构建某一架构

```bash
# Apple Silicon
npx tauri build --target aarch64-apple-darwin --config src-tauri/tauri.conf.macos.json

# Intel
npx tauri build --target x86_64-apple-darwin --config src-tauri/tauri.conf.macos.json
```

产物路径会带上架构后缀，如 `html-ppt-editor_0.1.0_aarch64.dmg`、`html-ppt-editor_0.1.0_x64.dmg`。

---

## 五、签名与公证（分发必需）

本地测试用 ad-hoc 签名即可。若要**分发给其他用户**（通过网站、邮件等非 App Store 渠道），必须进行 **Developer ID 签名 + Apple 公证**，否则用户打开时会被 Gatekeeper 拦截。

### 5.1 准备 Apple 开发者账号

需加入 [Apple Developer Program](https://developer.apple.com/programs/)（年费 $99），并在证书管理中创建：

- **Developer ID Application** 证书（用于签名 .app）
- **Developer ID Installer** 证书（仅 .pkg 需要，dmg 不需要）

在 Mac 上安装证书后，用以下命令查看可用签名身份：

```bash
security find-identity -v -p codesigning
```

记下 `Developer ID Application: XXX (TEAMID)` 中的团队 ID。

### 5.2 生成 Tauri 签名密钥

```bash
# 生成密钥对（会提示设置密码）
npx tauri signer generate -w ~/.tauri/html-ppt-editor.key

# 输出类似：
#   Public key: ...
#   Private key saved to: /Users/you/.tauri/html-ppt-editor.key
```

将公钥（以 `dW50cnVzdGVkIGNvbW1lbnQ:` 开头的字符串）记录下来，放入 `tauri.conf.json` 的 `bundle.macOS.signingIdentity` 或通过环境变量传入。

### 5.3 配置环境变量

```bash
# 私钥路径
export TAURI_SIGNING_PRIVATE_KEY="$HOME/.tauri/html-ppt-editor.key"
# 私钥密码（生成时设置的）
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="你的密码"
```

将 `tauri.conf.macos.json` 中的 `signingIdentity` 改为你的 `Developer ID Application` 证书名，或留空让 Tauri 自动选择。

### 5.4 签名构建

```bash
SIGN=1 ./build-macos.sh universal
```

### 5.5 提交公证

公证需三个凭证：Apple ID、Team ID、App 专用密码（在 https://appleid.apple.com 生成）。

```bash
export APPLE_ID="your@apple.com"
export APPLE_TEAM_ID="XXXXXXXXXX"
export APPLE_APP_PASSWORD="xxxx-xxxx-xxxx-xxxx"

NOTARIZE=1 SIGN=1 ./build-macos.sh universal
```

脚本会依次执行：

1. `xcrun notarytool submit --wait` — 上传并等待公证结果（通常 1–5 分钟）
2. `xcrun stapler staple` — 将公证凭证打钉到 dmg

公证成功后，用户下载 dmg 双击即可打开，不再有「无法验证开发者」提示。

---

## 六、App Store 分发（可选）

若计划上架 Mac App Store，需要：

1. 使用 **Mac App Distribution** 证书签名（而非 Developer ID）
2. 启用 App Sandbox（entitlements 中 `com.apple.security.app-sandbox = true`）
3. 配置文件访问权限 entitlement（本应用需要读写用户选择的文件夹）：
   ```xml
   <key>com.apple.security.files.user-selected.read-write</key>
   <true/>
   ```
4. 通过 Xcode Organizer 或 `xcrun altool` 上传

> 本应用的文件读写能力来自 `tauri-plugin-dialog` 的原生文件对话框，App Store 审核时需说明用途。

---

## 七、产物与目录

```
src-tauri/target/release/
├── html-ppt-editor                    # 可执行文件（仅 native 架构构建时）
└── bundle/
    ├── dmg/
    │   └── html-ppt-editor_0.1.0_aarch64.dmg    # 安装包
    └── macos/
        └── html-ppt-editor.app/                  # 应用包（右键显示包内容可查看）
            └── Contents/
                ├── MacOS/html-ppt-editor
                ├── Resources/
                └── Info.plist
```

便携版即为 `bundle/macos/html-ppt-editor.app`，可直接拷贝运行。

---

## 八、macOS 与 Windows 构建差异对照

| 项目 | Windows | macOS |
|---|---|---|
| 打包目标 | NSIS (`.exe`) | DMG (`.dmg`) / APP (`.app`) |
| WebView | WebView2 Runtime（需检测/安装） | WKWebView（系统内置，无需分发） |
| 配置文件 | `tauri.conf.json` (offline) / `tauri.conf.online.json` | `tauri.conf.macos.json` |
| 构建命令 | `npm run build:all` | `npm run build:macos` 或 `./build-macos.sh` |
| 签名 | 可选（Authenticode） | 本地 ad-hoc；分发必需 Developer ID |
| 公证 | 无 | 分发必需（notarytool + stapler） |
| Rust 依赖 | `windows-sys`（检测 WebView2） | 无平台特定依赖 |
| 最低系统 | Windows 10 1803+ | macOS 10.15+ |
| 产物路径 | `bundle/nsis/` | `bundle/dmg/` + `bundle/macos/` |

---

## 九、常见问题

### Q1：构建时报 `linking with `cc` failed` / `clang` 找不到

未安装 Xcode Command Line Tools。执行 `xcode-select --install`，或 `sudo xcode-select -s /Applications/Xcode.app/Contents/Developer`。

### Q2：`error: failed to run custom build command for ...` 且提示 SDK 版本低

macOS 版本过低，升级到 macOS 11+ 或在 `tauri.conf.macos.json` 中提高 `minimumSystemVersion`。

### Q3：打开 app 提示「已损坏，无法打开」

通常是未签名或未公证的 app 被 Gatekeeper 拦截。本地测试可执行：

```bash
xattr -cr /Applications/html-ppt-editor.app
```

分发则必须走签名 + 公证流程（见第五节）。

### Q4：构建时 `cargo` 编译极慢

首次构建需编译 Tauri 全部依赖（约 5–10 分钟），后续增量构建仅几十秒。可启用 sccache 加速：

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Q5：想离线构建

参考 `README.md` 的「离线构建」章节，在一台联网 Mac 上执行 `cargo vendor`，把 `vendor/` 和 `.cargo/config.toml` 拷到目标机器即可。

### Q6：universal 构建时报 `target not found`

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

---

## 十、参考链接

- [Tauri v2 macOS 构建文档](https://v2.tauri.app/start/prerequisites/#macos)
- [Tauri macOS 签名与公证](https://v2.tauri.app/distribute/sign/macos/)
- [Apple Notary Tool 文档](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
