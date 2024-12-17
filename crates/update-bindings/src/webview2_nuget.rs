use regex::Regex;
use std::{
    convert::From,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::webview2_path::*;

include!("../../bindings/src/declared_interfaces.rs");

const WEBVIEW2_NAME: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VERSION: &str = "1.0.2903.40";

pub fn install() -> crate::Result<PathBuf> {
    let out_dir = get_out_dir();
    let install_root = match out_dir.to_str() {
        Some(path) => path,
        None => return Err(crate::Error::MissingPath(out_dir)),
    };

    let mut package_root = out_dir.clone();
    package_root.push(format!("{WEBVIEW2_NAME}.{WEBVIEW2_VERSION}"));

    if !check_nuget_dir(install_root)? {
        let mut nuget_path = get_manifest_dir();
        nuget_path.push("tools");
        nuget_path.push("nuget.exe");

        let nuget_tool = match nuget_path.to_str() {
            Some(path) => path,
            None => return Err(crate::Error::MissingPath(nuget_path)),
        };

        let output = install_sdk_package(nuget_tool, install_root)?;

        if !output.status.success() {
            return Err(crate::Error::NugetCli(format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        if !check_nuget_dir(install_root)? {
            return Err(crate::Error::MissingPath(package_root));
        }
    }

    Ok(package_root)
}

fn check_nuget_dir(install_root: &str) -> crate::Result<bool> {
    let nuget_path = format!("{WEBVIEW2_NAME}.{WEBVIEW2_VERSION}");
    let mut dir_iter = fs::read_dir(install_root)?.filter(|dir| match dir {
        Ok(dir) => match dir.file_type() {
            Ok(file_type) => {
                file_type.is_dir()
                    && match dir.file_name().to_str() {
                        Some(name) => name.eq_ignore_ascii_case(&nuget_path),
                        None => false,
                    }
            }
            Err(_) => false,
        },
        Err(_) => false,
    });
    Ok(dir_iter.next().is_some())
}

#[cfg(not(windows))]
fn install_sdk_package(nuget_tool: &str, install_root: &str) -> crate::Result<Output> {
    Command::new("mono")
        .args(&[
            nuget_tool,
            "install",
            WEBVIEW2_NAME,
            "-OutputDirectory",
            install_root,
            "-Version",
            WEBVIEW2_VERSION,
            "-Source",
            "https://api.nuget.org/v3/index.json",
        ])
        .output()
        .map_err(|_| crate::Error::MissingMono)
}

#[cfg(windows)]
fn install_sdk_package(nuget_tool: &str, install_root: &str) -> crate::Result<Output> {
    Command::new(nuget_tool)
        .args([
            "install",
            WEBVIEW2_NAME,
            "-OutputDirectory",
            install_root,
            "-Version",
            WEBVIEW2_VERSION,
            "-Source",
            "https://api.nuget.org/v3/index.json",
        ])
        .output()
        .map_err(|_| crate::Error::NugetCli(String::from("nuget.exe not found")))
}

pub fn update_libs(package_root: &Path) -> crate::Result<()> {
    const WEBVIEW2_LIBS: &[&str] = &[
        "WebView2Loader.dll",
        "WebView2Loader.dll.lib",
        "WebView2LoaderStatic.lib",
    ];
    const WEBVIEW2_TARGETS: &[&str] = &["arm64", "x64", "x86"];

    let mut native_dir = package_root.to_path_buf();
    native_dir.push("build");
    native_dir.push("native");
    let manifest_dir = get_bindings_dir()?;
    for target in WEBVIEW2_TARGETS {
        for lib in WEBVIEW2_LIBS {
            let mut lib_src = native_dir.clone();
            lib_src.push(target);
            lib_src.push(lib);

            let mut lib_dest = manifest_dir.clone();
            lib_dest.push(target);
            if !lib_dest.is_dir() {
                fs::create_dir(lib_dest.as_path())?;
            }

            lib_dest.push(lib);
            println!("Copy from {lib_src:?} -> {lib_dest:?}");
            fs::copy(lib_src.as_path(), lib_dest.as_path())?;
        }
    }

    Ok(())
}

pub fn update_declared_interfaces(package_root: &Path) -> crate::Result<bool> {
    let contents = get_declared_interfaces(package_root)?;
    let callbacks = get_declared_callbacks(contents.as_str())?;
    let declared_callbacks = all_declared_callbacks()
        .into_iter()
        .map(String::from)
        .collect();
    let options = get_declared_options(contents.as_str())?;
    let declared_options = all_declared_options()
        .into_iter()
        .map(String::from)
        .collect();
    if callbacks == declared_callbacks && options == declared_options {
        return Ok(false);
    }

    let mut source_path = get_bindings_dir()?;
    source_path.push("src");
    source_path.push("declared_interfaces.rs");
    let mut source_file = fs::File::create(source_path)?;
    writeln!(
        source_file,
        r#"use std::collections::BTreeSet;

/// Generate a list of all `ICoreWebView2...Handler` interfaces declared in `WebView2.h`. This is
/// for testing purposes to make sure they are all covered in
/// [callback.rs](../../webview2-com/src/callback.rs).
pub fn all_declared_callbacks() -> BTreeSet<&'static str> {{
    let mut interfaces = BTreeSet::new();
"#,
    )?;
    for interface in callbacks {
        writeln!(source_file, r#"    interfaces.insert("{interface}");"#)?;
    }
    writeln!(
        source_file,
        r#"
    interfaces
}}
    
/// Generate a list of all `ICoreWebView2EnvironmentOptions` interfaces declared in `WebView2.h`.
/// This is for testing purposes to make sure they are all covered in
/// [options.rs](../../webview2-com/src/options.rs).
pub fn all_declared_options() -> BTreeSet<&'static str> {{
    let mut interfaces = BTreeSet::new();
"#,
    )?;
    for interface in options {
        writeln!(source_file, r#"    interfaces.insert("{interface}");"#)?;
    }
    writeln!(
        source_file,
        r#"
    interfaces
}}"#
    )?;
    Ok(true)
}

fn get_declared_interfaces(package_root: &Path) -> crate::Result<String> {
    let mut include_path = package_root.to_path_buf();
    include_path.push("build");
    include_path.push("native");
    include_path.push("include");
    include_path.push("WebView2.h");
    let mut contents = String::new();
    fs::File::open(include_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_declared_callbacks(contents: &str) -> crate::Result<BTreeSet<String>> {
    let pattern = Regex::new(r"^\s*typedef\s+interface\s+(ICoreWebView2[A-Za-z0-9]+Handler)\s+")?;
    let interfaces: BTreeSet<String> = contents
        .lines()
        .filter_map(|line| pattern.captures(line))
        .filter_map(|captures| captures.get(1))
        .map(|match_1| String::from(match_1.as_str()))
        .collect();
    if interfaces.is_empty() {
        Err(crate::Error::MissingTypedef)
    } else {
        Ok(interfaces)
    }
}

fn get_declared_options(contents: &str) -> crate::Result<BTreeSet<String>> {
    let pattern =
        Regex::new(r"^\s*typedef\s+interface\s+(ICoreWebView2EnvironmentOptions[0-9]*)\s+")?;
    let interfaces: BTreeSet<String> = contents
        .lines()
        .filter_map(|line| pattern.captures(line))
        .filter_map(|captures| captures.get(1))
        .map(|match_1| String::from(match_1.as_str()))
        .collect();
    if interfaces.is_empty() {
        Err(crate::Error::MissingTypedef)
    } else {
        Ok(interfaces)
    }
}
