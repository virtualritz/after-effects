set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

TargetDir := env_var_or_default("CARGO_TARGET_DIR", "target")
export AESDK_ROOT := if env("AESDK_ROOT", "") == "" { justfile_directory() / "../../sdk/AfterEffectsSDK" } else { env_var("AESDK_ROOT") }
export PRSDK_ROOT := if env("PRSDK_ROOT", "") == "" { justfile_directory() / "../../sdk/Premiere Pro 22.0 C++ SDK" } else { env_var("PRSDK_ROOT") }

[windows]
build:
    cargo build
    Start-Process PowerShell -Verb runAs -ArgumentList "-command Copy-Item -Force '{{TargetDir}}\debug\{{BinaryName}}.dll' 'C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\{{PluginName}}.aex'"

[windows]
release:
    cargo build --release
    Copy-Item -Force '{{TargetDir}}\release\{{BinaryName}}.dll' '{{TargetDir}}\release\{{PluginName}}.aex'
    Start-Process PowerShell -Verb runAs -ArgumentList "-command Copy-Item -Force '{{TargetDir}}\release\{{BinaryName}}.dll' 'C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\{{PluginName}}.aex'"

[macos]
build:
    cargo build
    just -f {{justfile()}} create_bundle {{TargetDir}}/debug

[macos]
release:
    cargo build --release
    just -f {{justfile()}} create_bundle {{TargetDir}}/release

[macos]
create_bundle TargetDir:
    echo "Creating universal plugin bundle"
    rm -Rf {{TargetDir}}/{{PluginName}}.plugin
    mkdir -p {{TargetDir}}/{{PluginName}}.plugin/Contents/Resources
    mkdir -p {{TargetDir}}/{{PluginName}}.plugin/Contents/MacOS

    rustup target add aarch64-apple-darwin
    rustup target add x86_64-apple-darwin

    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin

    echo "eFKTFXTC" >> {{TargetDir}}/{{PluginName}}.plugin/Contents/PkgInfo
    /usr/libexec/PlistBuddy -c 'add CFBundlePackageType string eFKT' {{TargetDir}}/{{PluginName}}.plugin/Contents/Info.plist
    /usr/libexec/PlistBuddy -c 'add CFBundleSignature string FXTC' {{TargetDir}}/{{PluginName}}.plugin/Contents/Info.plist
    /usr/libexec/PlistBuddy -c 'add CFBundleIdentifier string {{BundleIdentifier}}' {{TargetDir}}/{{PluginName}}.plugin/Contents/Info.plist

    cp {{TargetDir}}/x86_64-apple-darwin/release/{{BinaryName}}.rsrc {{TargetDir}}/{{PluginName}}.plugin/Contents/Resources/{{PluginName}}.rsrc

    lipo {{TargetDir}}/{x86_64,aarch64}-apple-darwin/release/{{BinaryName}}.dylib -create -output {{TargetDir}}/{{PluginName}}.plugin/Contents/MacOS/{{BinaryName}}.dylib
    mv {{TargetDir}}/{{PluginName}}.plugin/Contents/MacOS/{{BinaryName}}.dylib {{TargetDir}}/{{PluginName}}.plugin/Contents/MacOS/{{PluginName}}

    # codesign with the first development cert we can find using its hash
    codesign --options runtime --timestamp -strict  --sign $( security find-identity -v -p codesigning | grep -m 1 "Apple Development" | awk -F ' ' '{print $2}' ) {{TargetDir}}/{{PluginName}}.plugin
