$ErrorActionPreference = 'Stop'
cargo build --release
if (!(Test-Path 'target/release/orbit-browser.exe')) { throw 'Orbit executable was not produced.' }
Write-Host 'Orbit build complete: target/release/orbit-browser.exe'
