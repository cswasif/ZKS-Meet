# Android Build Errors Documentation

This document provides comprehensive troubleshooting for Android build issues in the ZKS-Meet Edge Hive Tauri application.

## 🚨 Common Build Errors

### 1. MODULE_NOT_FOUND Error for "src-tauri/tauri"

**Error:**
```
Error: Cannot find module 'src-tauri/tauri'
```

**Root Cause:** Missing Tauri dependencies in package.json

**Solution:**
```bash
npm install @tauri-apps/api @tauri-apps/cli
```

**Prevention:** Ensure package.json includes:
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.9.1",
    "@tauri-apps/cli": "^2.9.6"
  }
}
```

### 2. Ring Crate Cross-Compilation Errors

**Error:**
```
error: failed to run custom build command for `ring v0.16.20`
caused by: process didn't exit successfully: `failed to find clang.exe`
```

**Root Cause:** Ring crate requires C compiler for cryptographic operations

**Solutions:**

#### A. Environment Variable Approach
```bash
export RING_PREGENERATE_ASM=1
export ANDROID_NDK_HOME=/path/to/android-ndk
```

#### B. Cargo Configuration Approach
Create `.cargo/config.toml`:
```toml
[target.aarch64-linux-android]
linker = "path/to/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "path/to/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
```

#### C. Rust Version Pinning
Use specific Rust version compatible with ring:
```bash
rustup install 1.77.2
rustup default 1.77.2
```

### 3. File Creation Error (Os code 183)

**Error:**
```
error: failed to run custom build command for `ring v0.16.20`
caused by: could not create file: Os { code: 183, kind: Uncategorized, message: "Cannot create a file when that file already exists." }
```

**Root Cause:** Concurrent file access during ring compilation

**Solutions:**

#### A. Clean Build
```bash
cargo clean
cd src-tauri
cargo tauri android build --apk
```

#### B. Remove Target Directory
```bash
rm -rf target/
rm -rf src-tauri/target/
```

#### C. Use Pre-built Ring
```bash
cd ~/.cargo/registry/src/index.crates.io-*/ring-*/
cargo build --target aarch64-linux-android --release
```

### 4. Android NDK Path Issues

**Error:**
```
error: linker `aarch64-linux-android21-clang` not found
```

**Root Cause:** Incorrect Android NDK path configuration

**Solution:**
```bash
# Verify NDK installation
ls $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/

# Set correct environment variables
export ANDROID_SDK_ROOT=/path/to/android-sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/25.2.9519653
```

## 🔧 GitHub Actions Solutions

### Standard Build Workflow
The project includes two GitHub Actions workflows:

1. **Standard Build** (`.github/workflows/build-android.yml`)
   - Multi-target builds
   - Dependency caching
   - Security scanning

2. **Ring Fix Build** (`.github/workflows/build-android-ring-fix.yml`)
   - Specialized ring compilation handling
   - Rust version pinning
   - Pre-build ring compilation

### Workflow Triggers
- Push to `main` branch
- Push to `android-fix` branch
- Pull requests to `main`
- Manual workflow dispatch

### Build Matrix
```yaml
strategy:
  matrix:
    target:
      - aarch64-linux-android
      - armv7-linux-androideabi
      - x86_64-linux-android
      - i686-linux-android
```

## 🛠️ Development Environment Setup

### Prerequisites
1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add aarch64-linux-android
   ```

2. **Android SDK/NDK**
   ```bash
   # Install Android Studio or SDK command line tools
   # Set ANDROID_SDK_ROOT and ANDROID_NDK_HOME
   ```

3. **Node.js**
   ```bash
   # Install Node.js 18+
   npm install
   ```

### Local Build Commands
```bash
# Initialize Tauri Android project
cd src-tauri
cargo tauri android init

# Build debug APK
cargo tauri android build --apk --debug

# Build release APK
cargo tauri android build --apk --release

# Run development server
npm run dev
```

## 📱 libp2p Configuration

### Dependencies
```toml
[dependencies]
libp2p = { version = "0.52", features = ["tcp", "noise", "yamux", "mdns", "ping", "macros", "tokio", "websocket"] }
tokio = { version = "1.0", features = ["full"] }
```

### Key Features
- **TCP Transport**: Reliable peer connections
- **Noise Protocol**: Secure communication
- **Yamux**: Stream multiplexing
- **mDNS**: Local peer discovery
- **WebSocket**: Browser compatibility

### Network Configuration
```rust
let transport = tcp::tokio::Transport::default()
    .upgrade(upgrade::Version::V1Lazy)
    .authenticate(noise::Config::new(&local_key).expect("signing libp2p-noise static keypair"))
    .multiplex(yamux::Config::default())
    .boxed();
```

## 🔍 Debugging Tips

### 1. Verbose Build Output
```bash
RUST_BACKTRACE=1 cargo tauri android build --apk --verbose
```

### 2. Check Environment Variables
```bash
echo $ANDROID_SDK_ROOT
echo $ANDROID_NDK_HOME
echo $RING_PREGENERATE_ASM
```

### 3. Verify Toolchain Installation
```bash
rustup show
rustc --version
cargo --version
```

### 4. Android SDK Verification
```bash
sdkmanager --list
adb devices
```

## 🚀 Performance Optimization

### 1. Dependency Caching
GitHub Actions workflows include comprehensive caching:
- Cargo registry cache
- Cargo index cache
- Cargo build cache

### 2. Build Matrix Optimization
- Parallel builds for different architectures
- Matrix strategy for multiple targets
- Artifact retention policies

### 3. Ring Compilation Optimization
- Pre-generated assembly (`RING_PREGENERATE_ASM=1`)
- Rust version pinning
- Alternative crypto libraries (rustls)

## 📊 Build Status Monitoring

### GitHub Actions Status
Check build status at:
- Standard build: `.github/workflows/build-android.yml`
- Ring fix build: `.github/workflows/build-android-ring-fix.yml`

### Artifact Downloads
Successful builds produce:
- Debug APK files
- Release APK files
- Build logs (on failure)

## 🔗 Related Resources

- [Tauri Android Documentation](https://tauri.app/mobile/)
- [libp2p Rust Documentation](https://docs.rs/libp2p/)
- [Ring Crate Issues](https://github.com/briansmith/ring/issues)
- [Android NDK Guide](https://developer.android.com/ndk/guides)

## 📝 Contributing

When encountering new build errors:
1. Document the error and solution
2. Update this documentation
3. Test the solution in CI/CD
4. Share findings with the team

## 📞 Support

For build issues:
1. Check this documentation
2. Review GitHub Actions logs
3. Test with provided workflows
4. Create issue with detailed error logs