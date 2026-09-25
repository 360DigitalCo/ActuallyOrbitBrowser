# Orbit Browser

Orbit is a native Rust browser shell for 360Digital built around Servo. The browser does **not** use Chromium, Electron, WebView2, Google services, Supabase, or Neon at runtime.

## Defaults

- Home: `https://360-search.com`
- Default search: `https://360-search.com/search?q-%s`
- Built-in alternate engines: DuckDuckGo, Brave Search, Bing
- Custom search engines: stored locally in Orbit settings
- Internal settings page: `orbit://settings`
- Reload button and `Ctrl+R` use the webview reload path
- Dark browser chrome follows the 360 `buttonTemplate.html` visual language: 6px radius, subtle border, slide/fill-style hover intent, and press feedback.

## Build on Windows

Requirements: Rust 1.90+, Visual Studio Build Tools, CMake, Clang, NASM and the native dependencies required by Servo.

```powershell
cargo build --release
```

The executable will be under `target\\release\\orbit-browser.exe`.

This environment did not have `rustc`/Cargo installed, so a native `.exe` could not be produced here without pretending a build succeeded.

## Windows CI

Use `.github/workflows/windows.yml` to build the release artifact on GitHub Actions.
