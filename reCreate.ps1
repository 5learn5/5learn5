$ScriptDirectory = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent

$masterAccount = "its.testnet"
$childAccount = "lamp.its.testnet"
$global:wasmName = "lamp"

# re-uilding the wasm
# assumption is build.ps1 is in same folder
.\build.ps1
# Re-deploying the contract
near delete $childAccount $masterAccount  | Out-Null
near create-account $childAccount --masterAccount $masterAccount

near deploy $childAccount --wasmFile .\res\${wasmName}.wasm --accountId $masterAccount

set-location -Path $ScriptDirectory