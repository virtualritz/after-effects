# Cross-Compilation on Linux

## Overview

As noted in the README, Adobe After Effects and Premiere Pro only exist for **macOS** and **Windows**. There is no Linux version of these applications, and therefore no Linux SDK or native Linux support for plugins.

However, this Rust codebase has been designed with Linux compatibility in mind for the build toolchain. The `pipl` crate already includes Linux support (treating it the same as macOS for endianness and resource generation), which enables **cross-compilation** from Linux to target macOS and Windows platforms.

## Current State

The codebase already supports:
- Building on macOS → producing macOS plugins (.plugin bundles)
- Building on Windows → producing Windows plugins (.aex/.dll)
- Building on Linux → **partially supported** (see below)

### Linux Build Support in the Codebase

The `pipl` crate (plugin resource generator) already has Linux support:
- Line 7 in `pipl/src/lib.rs`: `#[cfg(any(target_os = "macos", target_os = "linux"))]`
- Line 59: Linux uses big-endian byte order (same as macOS)
- Line 1642: Linux generates PkgInfo and plist files (same as macOS)

The `AdobePlugin.just` file only has recipes for macOS and Windows, but the underlying Rust code can compile on Linux.

## Cross-Compilation Scenarios

### 1. Linux → Windows (Easier)

Cross-compiling to Windows from Linux is **relatively straightforward** using the MinGW toolchain.

#### Option A: Using MinGW-w64 (Direct)

**Pros:**
- Native Linux packages available
- No Docker required
- Relatively simple setup

**Cons:**
- Requires manual linker configuration
- May have issues with some Windows-specific crates

**Setup:**
```bash
# Install MinGW-w64
sudo apt install mingw-w64  # Debian/Ubuntu
# or
sudo dnf install mingw64-gcc  # Fedora/RHEL

# Add Rust target
rustup target add x86_64-pc-windows-gnu

# Configure cargo linker (create .cargo/config.toml)
cat > .cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"
EOF

# Build
cargo build --target x86_64-pc-windows-gnu --release
```

#### Option B: Using the `cross` tool (Recommended)

**Pros:**
- Zero setup - uses Docker containers
- Handles all dependencies automatically
- Consistent build environment

**Cons:**
- Requires Docker
- Larger disk space usage

**Setup:**
```bash
# Install cross
cargo install cross

# Build (cross handles everything)
cross build --target x86_64-pc-windows-gnu --release
```

The output will be a `.dll` file that needs to be renamed to `.aex` for After Effects/Premiere Pro.

**Dependencies to consider:**
- `win_dbg_logger` (Windows-only, in after-effects/Cargo.toml)
- `windows-sys` (Windows-only, in after-effects/Cargo.toml)

These should compile fine with MinGW toolchain.

### 2. Linux → macOS (More Complex)

Cross-compiling to macOS from Linux is **significantly more challenging** due to:
1. Apple's proprietary SDK licensing restrictions
2. Requirement for Xcode SDK files
3. Code signing requirements for modern After Effects/Premiere Pro (25.2+)

#### Using osxcross

**Pros:**
- Most comprehensive solution for macOS cross-compilation
- Supports multiple macOS SDK versions

**Cons:**
- Complex setup requiring macOS SDK extraction
- SDK cannot be legally redistributed
- Code signing still problematic
- May violate Apple's EULA

**Requirements:**
- Access to a Mac or Xcode download to extract the SDK
- Significant setup time

**High-level process:**
```bash
# 1. Add Rust targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# 2. Install osxcross
git clone https://github.com/tpoechtrager/osxcross
cd osxcross

# 3. Obtain macOS SDK (must be done legally from Xcode)
# You need to extract SDK from Xcode on a Mac and transfer it
# Example: MacOSX14.5.sdk.tar.xz

# 4. Build osxcross toolchain
./build.sh

# 5. Configure Cargo
cat >> ~/.cargo/config.toml << 'EOF'
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin23-clang"
ar = "x86_64-apple-darwin23-ar"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin23-clang"
ar = "aarch64-apple-darwin23-ar"
EOF

# 6. Build with osxcross in PATH
export PATH="$PWD/target/bin:$PATH"
cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release

# 7. Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/libplugin.dylib \
  target/aarch64-apple-darwin/release/libplugin.dylib \
  -output plugin.dylib
```

**Major limitations:**
- Code signing: The `AdobePlugin.just` script requires `codesign` which is macOS-only
- As of After Effects & Premiere Pro 25.2, all macOS plugins **require a valid signature** or they fail to load with error "2685337601"
- Ad-hoc signing from Linux won't work for distribution
- Need Apple Developer certificate for proper signing

**Dependencies to consider:**
- `oslog` (macOS-only logging, in after-effects/Cargo.toml)
- `objc2-core-foundation` (macOS-only, in after-effects/Cargo.toml)

These require the macOS SDK to compile.

## Recommendations

### For Development on Linux

**Best approach:** Use virtual machines or cloud instances
- Run Windows VM for Windows plugin development
- Run macOS VM (on actual Mac hardware via cloud) for macOS development
- Both options allow proper testing and signing

### For CI/CD (GitHub Actions, GitLab CI, etc.)

**Recommended setup:**
```yaml
matrix:
  os: [ubuntu-latest, windows-latest, macos-latest]
```

This allows:
- Native builds on each platform
- Proper code signing on macOS (using secrets for certificates)
- Testing in actual target environments

### For Cross-Compilation Experiments

If you still want to try cross-compilation on Linux:

1. **Windows target**: Use `cross` tool (Docker-based) - this should work reliably
2. **macOS target**: Only attempt if you:
   - Have legal access to macOS SDK
   - Don't need code signing (for testing only)
   - Are willing to invest significant setup time

## Technical Constraints

### Why No Linux Runtime?

After Effects and Premiere Pro are not available on Linux, so even if you could compile a plugin:
- There's no host application to load it
- The AE/Pr SDKs are platform-specific
- No Linux SDK exists

The only use case for Linux builds would be:
- Cross-compilation to other platforms (covered above)
- Unit testing pure Rust code (not the FFI/plugin layer)

### Plugin Format Requirements

- **Windows**: `.dll` renamed to `.aex`
- **macOS**: `.plugin` bundle containing:
  - `Contents/MacOS/PluginName` (universal binary)
  - `Contents/Resources/PluginName.rsrc` (PiPL resource)
  - `Contents/Info.plist`
  - `Contents/PkgInfo`
  - Code signature

The `AdobePlugin.just` file handles this bundling, but only works on the native platform.

## Conclusion

**Cross-compilation from Linux is technically possible but practically limited:**

✅ **Linux → Windows**: Feasible with MinGW or `cross` tool, suitable for CI/CD
❌ **Linux → macOS**: Technically possible but impractical due to:
  - SDK licensing issues
  - Code signing requirements
  - Complex setup
  - Cannot test without actual macOS

**Recommended approach**: Use native builds or CI/CD with multiple platforms rather than cross-compilation.

## Future Improvements

To better support Linux-based development workflows:

1. **Add Linux recipes to `AdobePlugin.just`**:
   ```justfile
   [linux]
   build-windows:
       cross build --target x86_64-pc-windows-gnu
       # Handle .dll → .aex renaming

   [linux]
   build-macos:
       # Only if osxcross is set up
       echo "Warning: macOS cross-compilation from Linux has limitations"
       # osxcross build commands
   ```

2. **Document SDK acquisition** for legal cross-compilation

3. **Add Docker/container configurations** for reproducible cross-compilation environments

4. **Consider separating testable Rust logic** from plugin FFI layer to enable unit testing on Linux

## References

- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [osxcross](https://github.com/tpoechtrager/osxcross)
- [cross tool](https://github.com/cross-rs/cross)
- [MinGW-w64](https://www.mingw-w64.org/)
