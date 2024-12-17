use regex::Regex;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use windows_bindgen::bindgen;

use crate::webview2_path::*;

pub fn update_bindings() -> crate::Result<bool> {
    let source_path = generate_bindings()?;
    format_bindings(&source_path)?;
    let source = read_bindings(&source_path)?;

    let mut dest_path = get_bindings_dir()?;
    dest_path.push("src");
    dest_path.push("Microsoft.rs");
    let dest = read_bindings(&dest_path)?;

    if source != dest {
        fs::copy(&source_path, &dest_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn generate_bindings() -> crate::Result<PathBuf> {
    const WINMD_FILE: &str = "Microsoft.Web.WebView2.Win32.winmd";

    let mut winmd_path = get_manifest_dir();
    winmd_path.push("winmd");
    winmd_path.push(WINMD_FILE);
    let mut source_path = get_out_dir();
    source_path.push("Microsoft.rs");
    println!(
        "{}",
        bindgen([
            "--in",
            winmd_path.to_str().expect("invalid winmd path"),
            "--out",
            source_path.to_str().expect("invalid Microsoft.rs path"),
            "--filter",
            "Microsoft.Web.WebView2.Win32",
            "--config",
            "implement",
        ])
        .expect("bindgen failed")
    );

    let mut bindings = Default::default();
    fs::File::open(source_path.clone())?.read_to_string(&mut bindings)?;

    let mut source_file = fs::File::create(source_path.clone())?;

    source_file.write_all(patch_bindings(bindings)?.as_bytes())?;
    Ok(source_path)
}

fn patch_bindings(bindings: String) -> crate::Result<String> {
    let pattern = Regex::new(r#"#\s*\[\s*link\s*\(\s*name\s*=\s*"WebView2Loader"\s*\)\s*\]"#)?;
    let replacement = r#"
            #[cfg_attr(target_env = "msvc", link(name = "WebView2LoaderStatic", kind = "static"))]
            #[cfg_attr(not(target_env = "msvc"), link(name = "WebView2Loader.dll"))]
        "#;
    Ok(pattern.replace_all(&bindings, replacement).to_string())
}

fn format_bindings(source_path: &Path) -> crate::Result<()> {
    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(source_path);
    cmd.output()?;
    Ok(())
}

fn read_bindings(source_path: &Path) -> crate::Result<String> {
    let mut source_file = fs::File::open(source_path)?;
    let mut updated = String::default();
    source_file.read_to_string(&mut updated)?;
    Ok(updated)
}
