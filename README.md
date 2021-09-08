# webview2-rs
Rust bindings for the WebView2 COM APIs

## Crates in this repo
The root of this repo defines a virtual workspace in [Cargo.toml](./Cargo.toml) which includes three crates:
- [webview2-com](./crates/webview2-com/README.md): The main crate, which depends on the other 2.
- [webview2-com-macros](./crates/callback-macros/README.md): Macros used to build boilerplate implementations of all the `ICoreWebView2...Handler` interfaces declared in `WebView2.h`.
- [webview2-com-sys](./crates/bindings/README.md): Unsafe bindings built with the [Windows](https://github.com/microsoft/windows-rs) crate.

## Windows Metadata
The Windows crate requires a Windows Metadata (`winmd`) file describing the API. The one used in this repo was generated with the [webview2-win32md](https://github.com/wravery/webview2-win32md) project.

## Getting started
Run all the tests from the root of the repo:
```cmd
> cargo test
```

Or run the sample app from anywhere in the repo:
```cmd
> cargo run --example sample
```

See the [README.md](./crates/webview2-com/README.md) in `webview2-com` for more details about using that crate in your own project.

## Updating the WebView2 SDK
The `webview2-com-sys` [build.rs](./crates/bindings/build.rs) script automatically downloads the NuGet package for the SDK and updates the static libs (`WebView2LoaderStatic.lib`) for each architecture under `**/.windows/`, if you build on a Windows machine. You can tell the build script to use a different version by updating `WEBVIEW2_VERSION` in `build.rs`:
```rust
    const WEBVIEW2_VERSION: &str = "1.0.902.49";
```
 It will also regenerate [browser_version.rs](./crates/bindings/src/browser_version.rs) and [callback_interfaces.rs](./crates/bindings/src/callback_interfaces.rs) if they change in a new version. These two files are used in `webview2-com`, and in particular, the tests in [callback.rs](./crates/webview2-com/src/callback.rs) verify that all of the interfaces listed in `callback_interfaces.rs` are implemented. If a new version of the SDK declared additional callback interfaces, you will need to add those interfaces to `callback.rs` using the `#[completed_callback]` (for `ICoreWebView2...CompletedHandler` interfaces) and `#[event_callback]` (for `ICoreWebView2...EventHandler` interfaces) macros.

 It does not regenerate the `winmd` file automatically because that would depend on having the `dotnet` CLI installed. New versions of the SDK should be backwards compatible, but you may want to regenerate the `Microsoft.Web.WebView2.Win32.winmd` file using `webview2-win32md` if you need functionality which was added in a new version.
 
The Windows crate looks in different locations for the `.windows` directory depending on whether you are building as part of the `Cargo` virtual workspace in the repo (shared workspace root) or publishing one of the sub-crates (manifest/crate directory). If you do regenerate the `winmd` file, you should update it in all 3 locations where it is referenced as part of the build:
 - `/.windows/winmd/Microsoft.Web.WebView2.Win32.winmd`
 - `/crates/bindings/.windows/winmd/Microsoft.Web.WebView2.Win32.winmd`
 - `/crates/webview2-com/.windows/winmd/Microsoft.Web.WebView2.Win32.winmd`