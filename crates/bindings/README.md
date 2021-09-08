# webview2-com-sys
This crate implements unsafe Rust bindings for the [WebView2](https://aka.ms/webview2) COM APIs using the [Windows](https://github.com/microsoft/windows-rs) crate.

## Getting Started
This crate has a friendlier wrapper in [webview2-com](https://crates.io/crates/webview2-com).

## Windows Metadata
The Windows crate requires a Windows Metadata (`winmd`) file describing the API. The one used in this crate was generated with the [webview2-win32md](https://github.com/wravery/webview2-win32md) project.
