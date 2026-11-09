#!/usr/bin/env bash
# html-ppt-editor macOS 构建脚本
# 用法：
#   ./build-macos.sh              # 默认构建当前架构 dmg
#   ./build-macos.sh universal    # 构建通用二进制（Intel + Apple Silicon）
#   ./build-macos.sh arm64        # 仅 Apple Silicon
#   ./build-macos.sh x86_64       # 仅 Intel
#   SIGN=1 ./build-macos.sh ...   # 使用环境变量中的签名证书签名
#   NOTARIZE=1 SIGN=1 APPLE_ID=xxx@xxx.com APPLE_TEAM_ID=XXXXXXXXXX APPLE_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx ./build-macos.sh ...
#     签名 + 公证后再打包 dmg

set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

ARCH="${1:-native}"
BUNDLE_DIR="src-tauri/target/release/bundle/dmg"

echo "=========================================="
echo " HTML PPT Editor — macOS 构建"
echo " 架构: $ARCH"
echo "=========================================="

# --- 目标架构处理 ---
case "$ARCH" in
  native)
    TARGET_FLAG=""
    ;;
  universal)
    rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true
    TARGET_FLAG="--target universal-apple-darwin"
    ;;
  arm64|aarch64)
    rustup target add aarch64-apple-darwin 2>/dev/null || true
    TARGET_FLAG="--target aarch64-apple-darwin"
    ;;
  x86_64|x64)
    rustup target add x86_64-apple-darwin 2>/dev/null || true
    TARGET_FLAG="--target x86_64-apple-darwin"
    ;;
  *)
    echo "未知架构: $ARCH，应为 native / universal / arm64 / x86_64"
    exit 1
    ;;
esac

# --- 签名与公证配置 ---
SIGN_ARGS=""
if [[ "${SIGN:-0}" == "1" ]]; then
  if [[ -z "${TAURI_SIGNING_PRIVATE_KEY:-}" ]]; then
    echo "警告: 已启用签名 (SIGN=1) 但未设置 TAURI_SIGNING_PRIVATE_KEY"
    echo "  请参考 BUILD_MACOS.md 生成密钥并设置环境变量"
  fi
fi

# --- 构建 ---
echo ">>> 执行 tauri build ..."
# shellcheck disable=SC2086
npx tauri build $TARGET_FLAG --config src-tauri/tauri.conf.macos.json

# --- 公证（可选）---
DMG_PATH=$(find "$BUNDLE_DIR" -name "*.dmg" -print -quit 2>/dev/null || true)
if [[ -z "$DMG_PATH" ]]; then
  echo "错误: 未找到 dmg 产物"
  exit 1
fi

if [[ "${NOTARIZE:-0}" == "1" ]]; then
  echo ">>> 提交公证 ..."
  if [[ -z "${APPLE_ID:-}" || -z "${APPLE_TEAM_ID:-}" || -z "${APPLE_APP_PASSWORD:-}" ]]; then
    echo "错误: 公证需要 APPLE_ID / APPLE_TEAM_ID / APPLE_APP_PASSWORD"
    exit 1
  fi
  xcrun notarytool submit "$DMG_PATH" \
    --apple-id "$APPLE_ID" \
    --team-id "$APPLE_TEAM_ID" \
    --password "$APPLE_APP_PASSWORD" \
    --wait
  echo ">>> 公证完成，为 dmg 打钉 (staple) ..."
  xcrun stapler staple "$DMG_PATH"
fi

echo ""
echo "=========================================="
echo " 构建完成: $DMG_PATH"
ls -lh "$DMG_PATH"
echo "=========================================="
