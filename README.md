# ZKS-Meet

ZKS Protocol Meet - Edge Hive Tauri 2.0 Android Application with libp2p P2P discovery

## 🚀 Features

- **P2P Communication**: Direct peer-to-peer messaging using libp2p
- **Android Support**: Native Android APK build with Tauri 2.0
- **Cross-Platform**: Works on desktop and mobile devices
- **Real-time Messaging**: Instant message delivery between peers
- **Peer Discovery**: Automatic peer discovery on local networks
- **Modern UI**: Beautiful gradient interface with real-time status updates

## 📱 Android APK Build

This project includes comprehensive GitHub Actions workflows for building Android APKs, specifically designed to handle the complex libp2p/ring cross-compilation challenges.

### Build Status
[![Build Android APK](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android.yml/badge.svg)](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android.yml)
[![Build Android APK Ring Fix](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android-ring-fix.yml/badge.svg)](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android-ring-fix.yml)

## 🛠️ Technology Stack

- **Frontend**: HTML5, CSS3, JavaScript
- **Backend**: Rust with Tauri 2.0
- **P2P Networking**: libp2p with TCP, Noise, Yamux, mDNS
- **Build System**: Tauri CLI with Cargo
- **CI/CD**: GitHub Actions with Android NDK cross-compilation

## 📦 Installation

### Prerequisites
- Node.js 18+
- Rust stable toolchain
- Android SDK/NDK (for Android builds)

### Local Development
```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build Tauri application
cargo tauri build
```

### Android APK Build
The project includes automated GitHub Actions workflows that handle the complex Android cross-compilation:

1. **Standard Build**: Multi-target builds with caching
2. **Ring Fix Build**: Specialized workflow for libp2p/ring compilation issues

## 🔧 Configuration

### Android Build Configuration
The project includes optimized Cargo configuration for Android cross-compilation:

```toml
[target.aarch64-linux-android]
linker = "path/to/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
```

### Environment Variables
- `RING_PREGENERATE_ASM=1` - Optimizes ring compilation
- `ANDROID_SDK_ROOT` - Android SDK path
- `ANDROID_NDK_HOME` - Android NDK path

## 🚀 GitHub Actions

The project includes two comprehensive GitHub Actions workflows:

### 1. Standard Android Build (`.github/workflows/build-android.yml`)
- Multi-target builds (ARM64, ARMv7, x86_64, x86)
- Dependency caching for faster builds
- Security scanning with cargo audit
- APK artifact uploads

### 2. Ring Fix Build (`.github/workflows/build-android-ring-fix.yml`)
- Specialized handling for libp2p/ring cross-compilation
- Rust toolchain version pinning
- Pre-build ring compilation
- Matrix builds for different architectures

## 📱 P2P Features

### Peer Discovery
- Automatic peer discovery using mDNS
- Manual peer discovery trigger
- Real-time peer count updates

### Messaging
- Direct peer-to-peer messaging
- Message acknowledgment
- Real-time message status

### Network Status
- Connection status monitoring
- Network type detection
- Peer count tracking

## 🔍 Troubleshooting

### Ring Compilation Issues
The project includes comprehensive documentation for resolving libp2p/ring cross-compilation issues:

1. **Pre-generated Assembly**: `RING_PREGENERATE_ASM=1`
2. **Optimized Linker Configuration**: Custom Cargo config
3. **Rust Version Pinning**: Specific toolchain versions
4. **Alternative Crypto Libraries**: rustls configuration

See [`ANDROID_BUILD_ERRORS.md`](ANDROID_BUILD_ERRORS.md) for detailed troubleshooting steps.

## 📄 License

This project is part of the ZKS Protocol ecosystem.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Support

For issues and questions:
- Check the [troubleshooting documentation](ANDROID_BUILD_ERRORS.md)
- Review GitHub Actions build logs
- Test with the provided GitHub Actions workflows