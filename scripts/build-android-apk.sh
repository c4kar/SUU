#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "==> [1/5] Rust Android hedefleri kontrol ediliyor..."
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Android Gradle Plugin 8.x uyumluluğu için JDK 21 kontrolü (mise varsa otomatik kurar)
if command -v mise >/dev/null 2>&1; then
  echo "==> [2/5] Android Gradle uyumu için Java 21 (temurin-21) hazırlanıyor..."
  mise install java@temurin-21 >/dev/null 2>&1 || true
  JAVA_21_HOME="$(mise where java@temurin-21 2>/dev/null || true)"
  if [[ -n "$JAVA_21_HOME" ]]; then
    export JAVA_HOME="$JAVA_21_HOME"
    export PATH="$JAVA_HOME/bin:$PATH"
  fi
fi

# Kullanıcı dizininde yazılabilir Android SDK + NDK kurulumu
export ANDROID_HOME="${ANDROID_HOME:-$HOME/.local/share/android-sdk}"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
mkdir -p "$ANDROID_HOME/cmdline-tools"

if [[ ! -x "$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager" ]]; then
  echo "==> [3/5] Android Command-Line Tools indiriliyor ($ANDROID_HOME)..."
  TMP_ZIP="$(mktemp /tmp/cmdline-tools-XXXXXX.zip)"
  curl -fL "https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip" -o "$TMP_ZIP"
  rm -rf "$ANDROID_HOME/cmdline-tools/latest" "$ANDROID_HOME/cmdline-tools/cmdline-tools"
  unzip -q "$TMP_ZIP" -d "$ANDROID_HOME/cmdline-tools"
  mv "$ANDROID_HOME/cmdline-tools/cmdline-tools" "$ANDROID_HOME/cmdline-tools/latest"
  rm -f "$TMP_ZIP"
fi

SDKMANAGER="$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager"
echo "==> [3/5] Android SDK 35, Build-Tools ve NDK kontrol ediliyor..."
yes | "$SDKMANAGER" --licenses >/dev/null 2>&1 || true
"$SDKMANAGER" --install \
  "platform-tools" \
  "platforms;android-35" \
  "build-tools;35.0.0" \
  "ndk;27.2.12479018"

export NDK_HOME="$ANDROID_HOME/ndk/27.2.12479018"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"

if [[ ! -d "$ROOT_DIR/src-tauri/gen/android" ]]; then
  echo "==> [4/5] Tauri Android projesi oluşturuluyor..."
  bun run tauri android init
fi

echo "==> [5/5] Samsung (ARM64) APK derleniyor..."
bun run tauri android build --apk --target aarch64

APK_UNSIGNED="$ROOT_DIR/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk"
APK_OUT="$ROOT_DIR/suu-samsung-arm64.apk"

# Samsung telefona doğrudan kurulabilmesi için imzalama (debug/local keystore)
KEYSTORE="$HOME/.android/debug.keystore"
mkdir -p "$HOME/.android"
if [[ ! -f "$KEYSTORE" ]]; then
  keytool -genkey -v \
    -keystore "$KEYSTORE" \
    -storepass android \
    -alias androiddebugkey \
    -keypass android \
    -keyalg RSA \
    -keysize 2048 \
    -validity 10000 \
    -dname "CN=SUU Local,O=Android,C=TR"
fi

if [[ -f "$APK_UNSIGNED" ]]; then
  "$ANDROID_HOME/build-tools/35.0.0/apksigner" sign \
    --ks "$KEYSTORE" \
    --ks-pass pass:android \
    --key-pass pass:android \
    --out "$APK_OUT" \
    "$APK_UNSIGNED"
  echo ""
  echo "✅ Samsung telefonlara kurulmaya hazır imzalı APK üretildi:"
  echo "   👉 $APK_OUT"
fi
