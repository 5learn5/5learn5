$ScriptDirectory = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent
$wasmName = "lamp"


if (Test-Path -PathType Leaf -Path "./target/wasm32-unknown-unknown/${wasmName}.wasm") {
    Remove-Item  -Path "./target/wasm32-unknown-unknown/${wasmName}.wasm" -Force
}

# RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cargo build --target wasm32-unknown-unknown --release

if (!(Test-Path -PathType Container -Path "res")) {
    New-Item -ItemType Container "res" -Force | Out-Null
}
else {
    if (Test-Path -PathType Leaf -Path "./res/${wasmName}.wasm") {
        Remove-Item -Path "./res/${wasmName}.wasm" -Force
    }
}

copy-item "$ScriptDirectory\target\wasm32-unknown-unknown\release\${wasmName}.wasm" -Destination "$ScriptDirectory/res" -Force
set-location -Path $ScriptDirectory