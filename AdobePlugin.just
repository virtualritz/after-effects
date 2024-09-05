set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

TargetDir := env_var_or_default("CARGO_TARGET_DIR", "target")
export AESDK_ROOT := if env("AESDK_ROOT", "") == "" { justfile_directory() / "../../sdk/AfterEffectsSDK" } else { env_var("AESDK_ROOT") }
export PRSDK_ROOT := if env("PRSDK_ROOT", "") == "" { justfile_directory() / "../../sdk/Premiere Pro 22.0 C++ SDK" } else { env_var("PRSDK_ROOT") }

[windows]
build:
    cargo build
    if (-not $env:NO_INSTALL) { \
        Start-Process PowerShell -Verb runAs -ArgumentList "-command Copy-Item -Force '{{TargetDir}}\debug\{{BinaryName}}.dll' 'C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\{{PluginName}}.aex'" \
    }

[windows]
release:
    cargo build --release
    Copy-Item -Force '{{TargetDir}}\release\{{BinaryName}}.dll' '{{TargetDir}}\release\{{PluginName}}.aex'
    if (-not $env:NO_INSTALL) { \
        Start-Process PowerShell -Verb runAs -ArgumentList "-command Copy-Item -Force '{{TargetDir}}\release\{{BinaryName}}.dll' 'C:\Program Files\Adobe\Common\Plug-ins\7.0\MediaCore\{{PluginName}}.aex'" \
    }

[macos]
build:
    cargo build
    just -f {{justfile()}} create_bundle debug {{TargetDir}}

[macos]
release:
    cargo build --release
    just -f {{justfile()}} create_bundle release {{TargetDir}}

[macos]
create_bundle profile TargetDir:
    #!/bin/bash
    set -e
    echo "Creating plugin bundle"
    rm -Rf {{TargetDir}}/{{profile}}/{{PluginName}}.plugin
    mkdir -p {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Resources
    mkdir -p {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/MacOS

    echo "eFKTFXTC" >> {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/PkgInfo
    /usr/libexec/PlistBuddy -c 'add CFBundlePackageType string eFKT' {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Info.plist
    /usr/libexec/PlistBuddy -c 'add CFBundleSignature string FXTC' {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Info.plist
    /usr/libexec/PlistBuddy -c 'add CFBundleIdentifier string {{BundleIdentifier}}' {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Info.plist

    if [ "{{profile}}" == "release" ]; then
        # Build universal binary
        rustup target add aarch64-apple-darwin
        rustup target add x86_64-apple-darwin

        cargo build --release --target x86_64-apple-darwin
        cargo build --release --target aarch64-apple-darwin

        cp {{TargetDir}}/x86_64-apple-darwin/release/{{BinaryName}}.rsrc {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Resources/{{PluginName}}.rsrc
        lipo {{TargetDir}}/{x86_64,aarch64}-apple-darwin/release/lib{{BinaryName}}.dylib -create -output {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/MacOS/{{PluginName}}.dylib
        mv {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/MacOS/{{PluginName}}.dylib {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/MacOS/{{PluginName}}
    else
        cp {{TargetDir}}/{{profile}}/{{BinaryName}}.rsrc {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/Resources/{{PluginName}}.rsrc
        cp {{TargetDir}}/{{profile}}/lib{{BinaryName}}.dylib {{TargetDir}}/{{profile}}/{{PluginName}}.plugin/Contents/MacOS/{{PluginName}}
    fi

    # codesign with the first development cert we can find using its hash
    if [ -z "$NO_SIGN" ]; then
        codesign --options runtime --timestamp -strict  --sign $( security find-identity -v -p codesigning | grep -m 1 "Apple Development" | awk -F ' ' '{print $2}' ) {{TargetDir}}/{{profile}}/{{PluginName}}.plugin
    fi

    # Install
    if [ -z "$NO_INSTALL" ]; then
        sudo cp -rf "{{TargetDir}}/{{profile}}/{{PluginName}}.plugin" "/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/"
    fi
