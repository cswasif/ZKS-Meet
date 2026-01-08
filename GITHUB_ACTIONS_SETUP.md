# GitHub Actions Setup Guide

This guide explains how to use and customize the GitHub Actions workflows for building Android APKs in the ZKS-Meet repository.

## 🚀 Quick Start

The repository includes two automated workflows that handle the complex Android cross-compilation process:

### 1. Standard Android Build
- **File**: `.github/workflows/build-android.yml`
- **Purpose**: Multi-target builds with caching and security scanning
- **Trigger**: Push to `main` or `android-fix` branches

### 2. Ring Fix Build
- **File**: `.github/workflows/build-android-ring-fix.yml`
- **Purpose**: Specialized handling for libp2p/ring cross-compilation issues
- **Trigger**: Push to `main`, `android-fix`, or `ring-fix` branches

## 📋 Workflow Features

### Standard Build Workflow
```yaml
- Multi-architecture support (ARM64, ARMv7, x86_64, x86)
- Dependency caching for faster builds
- Security scanning with cargo audit
- APK artifact uploads
- Build matrix for parallel execution
```

### Ring Fix Workflow
```yaml
- Rust version pinning (1.77.2)
- Pre-generated ring assembly
- Specialized linker configuration
- Build log uploads on failure
- Matrix builds with different configurations
```

## 🔧 Configuration

### Environment Variables
The workflows use these key environment variables:

```yaml
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  RING_PREGENERATE_ASM: 1  # Optimizes ring compilation
```

### Android SDK/NDK Setup
```yaml
- name: Setup Android SDK
  uses: android-actions/setup-android@v3
  with:
    api-level: 33
    ndk: 25.2.9519653
    cmake: 3.22.1
```

### Rust Toolchain Configuration
```yaml
- name: Setup Rust
  uses: dtolnay/rust-toolchain@stable
  with:
    targets: ${{ matrix.target }}
```

## 🎯 Build Targets

### Supported Architectures
1. **aarch64-linux-android** (ARM64) - Modern Android devices
2. **armv7-linux-androideabi** (ARMv7) - Older Android devices
3. **x86_64-linux-android** (x86_64) - Android emulators
4. **i686-linux-android** (x86) - Legacy emulators

### Build Matrix Strategy
```yaml
strategy:
  matrix:
    target:
      - aarch64-linux-android
      - armv7-linux-androideabi
      - x86_64-linux-android
      - i686-linux-android
```

## 📦 Artifact Management

### APK Artifacts
Successful builds produce:
- **Debug APKs**: `src-tauri/gen/android/app/build/outputs/apk/debug/`
- **Release APKs**: `src-tauri/gen/android/app/build/outputs/apk/release/`

### Artifact Upload Configuration
```yaml
- name: Upload APK artifacts
  uses: actions/upload-artifact@v4
  if: success()
  with:
    name: android-apk-${{ matrix.target }}
    path: |
      src-tauri/gen/android/app/build/outputs/apk/debug/
      src-tauri/gen/android/app/build/outputs/apk/release/
    retention-days: 30
```

## 🔍 Troubleshooting

### Build Failures
1. **Check build logs** in GitHub Actions
2. **Download failure artifacts** for detailed logs
3. **Review ring compilation errors** in the ring-fix workflow

### Common Issues

#### Ring Compilation Errors
```yaml
# Use the ring-fix workflow for these errors:
- Rust version pinning
- Pre-generated assembly
- Specialized linker flags
```

#### Missing Dependencies
```yaml
# Ensure all dependencies are installed:
- name: Install system dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y build-essential pkg-config libssl-dev
```

## 🚀 Usage Examples

### Triggering Builds
```bash
# Push to main branch to trigger standard build
git push origin main

# Push to android-fix branch to trigger both builds
git push origin android-fix

# Push to ring-fix branch for specialized ring handling
git push origin ring-fix
```

### Manual Trigger
1. Go to repository **Actions** tab
2. Select desired workflow
3. Click **Run workflow**
4. Choose branch and options
5. Click **Run workflow** button

### Monitoring Builds
1. Check **Actions** tab for build status
2. Click on specific workflow run
3. Review build logs and artifacts
4. Download APK files from successful builds

## 🔧 Customization

### Adding New Targets
Edit the matrix strategy in the workflow file:
```yaml
strategy:
  matrix:
    target:
      - your-new-target
      - existing-targets...
```

### Modifying Build Commands
Customize the build steps:
```yaml
- name: Custom build step
  run: |
    cd src-tauri
    cargo tauri android build --apk --your-custom-flag
```

### Environment Variable Changes
Update environment variables:
```yaml
env:
  YOUR_CUSTOM_VAR: value
  RING_PREGENERATE_ASM: 1
```

## 📊 Performance Optimization

### Caching Strategy
The workflows implement three-level caching:
1. **Cargo registry cache**: Dependencies
2. **Cargo index cache**: Package index
3. **Cargo build cache**: Compiled artifacts

```yaml
- name: Cache cargo registry
  uses: actions/cache@v4
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
```

### Build Time Reduction
- Use dependency caching
- Optimize build matrix
- Parallel execution
- Artifact retention policies

## 🛡️ Security Features

### Security Scanning
```yaml
- name: Security audit
  run: cargo audit
  continue-on-error: true
```

### Dependency Updates
Regularly update dependencies:
```bash
cargo update
cargo audit
```

## 📈 Monitoring and Analytics

### Build Status Badges
Add to README.md:
```markdown
[![Build Android APK](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android.yml/badge.svg)](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android.yml)
[![Build Android APK Ring Fix](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android-ring-fix.yml/badge.svg)](https://github.com/cswasif/ZKS-Meet/actions/workflows/build-android-ring-fix.yml)
```

### Build Metrics
Monitor:
- Build success rate
- Build duration
- Artifact sizes
- Dependency vulnerabilities

## 🔄 Workflow Maintenance

### Regular Updates
1. **Update action versions** monthly
2. **Review build logs** for warnings
3. **Update dependencies** regularly
4. **Test new Android NDK versions**

### Version Management
```yaml
# Use specific versions for reproducibility:
uses: actions/checkout@v4
uses: dtolnay/rust-toolchain@stable
uses: android-actions/setup-android@v3
```

## 🆘 Support

### Getting Help
1. **Check build logs** for specific errors
2. **Review this documentation** for solutions
3. **Test with provided workflows** first
4. **Create issues** with detailed error logs

### Contributing
When improving workflows:
1. **Test changes** in a separate branch
2. **Document new features** in this guide
3. **Update build status badges** if needed
4. **Share improvements** with the team

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Tauri Mobile Development](https://tauri.app/mobile/)
- [Android NDK Guide](https://developer.android.com/ndk/guides)
- [Rust Cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

---

**Last Updated**: January 2026
**Version**: 1.0
**Maintainer**: ZKS Protocol Team