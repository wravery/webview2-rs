extern crate windows_bindgen;

fn main() -> Result<()> {
    match webview2_nuget::install() {
        Ok(package_root) => {
            webview2_nuget::update_libs(&package_root)?;

            if webview2_nuget::update_callback_interfaces(&package_root)? {
                println!("callback_interfaces.rs changed");
            }
        }
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }

    if webview2_bindgen::update_bindings()? {
        println!("Microsoft.rs changed");
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing Parent")]
    MissingParent(std::path::PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("Missing Typedef")]
    MissingTypedef,
    #[error("Missing Path")]
    MissingPath(std::path::PathBuf),
    #[error("Failed to run nuget CLI.\n{0}")]
    NugetCli(String),
    #[cfg(not(windows))]
    #[error("The nuget CLI requires mono to run on this host.")]
    MissingMono,
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_use]
extern crate thiserror;

mod webview2_path {
    use std::{convert::From, path::PathBuf};

    pub fn get_out_dir() -> PathBuf {
        PathBuf::from(env!("OUT_DIR"))
    }

    pub fn get_manifest_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    pub fn get_bindings_dir() -> super::Result<PathBuf> {
        let manifest_dir = get_manifest_dir();
        let mut bindings_dir = get_manifest_dir().parent().map_or_else(
            || Err(super::Error::MissingParent(manifest_dir)),
            |parent| Ok(PathBuf::from(parent)),
        )?;
        bindings_dir.push("bindings");
        Ok(bindings_dir)
    }
}

mod webview2_nuget {
    use std::{
        convert::From,
        fs,
        io::{Read, Write},
        path::{Path, PathBuf},
        process::{Command, Output},
    };

    use regex::Regex;

    use super::webview2_path::*;

    include!("../../bindings/src/callback_interfaces.rs");

    const WEBVIEW2_NAME: &str = "Microsoft.Web.WebView2";
    const WEBVIEW2_VERSION: &str = "1.0.2365.46";

    pub fn install() -> super::Result<PathBuf> {
        let out_dir = get_out_dir();
        let install_root = match out_dir.to_str() {
            Some(path) => path,
            None => return Err(super::Error::MissingPath(out_dir)),
        };

        let mut package_root = out_dir.clone();
        package_root.push(format!("{WEBVIEW2_NAME}.{WEBVIEW2_VERSION}"));

        if !check_nuget_dir(install_root)? {
            let mut nuget_path = get_manifest_dir();
            nuget_path.push("tools");
            nuget_path.push("nuget.exe");

            let nuget_tool = match nuget_path.to_str() {
                Some(path) => path,
                None => return Err(super::Error::MissingPath(nuget_path)),
            };

            let output = install_sdk_package(nuget_tool, install_root)?;

            if !output.status.success() {
                return Err(super::Error::NugetCli(format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                )));
            }

            if !check_nuget_dir(install_root)? {
                return Err(super::Error::MissingPath(package_root));
            }
        }

        Ok(package_root)
    }

    fn check_nuget_dir(install_root: &str) -> super::Result<bool> {
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
    fn install_sdk_package(nuget_tool: &str, install_root: &str) -> super::Result<Output> {
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
            .map_err(|_| super::Error::MissingMono)
    }

    #[cfg(windows)]
    fn install_sdk_package(nuget_tool: &str, install_root: &str) -> super::Result<Output> {
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
            .map_err(|_| super::Error::NugetCli(String::from("nuget.exe not found")))
    }

    pub fn update_libs(package_root: &Path) -> super::Result<()> {
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

    pub fn update_callback_interfaces(package_root: &Path) -> super::Result<bool> {
        let interfaces = get_callback_interfaces(package_root)?;
        let declared = all_declared().into_iter().map(String::from).collect();
        if interfaces == declared {
            return Ok(false);
        }

        let mut source_path = get_bindings_dir()?;
        source_path.push("src");
        source_path.push("callback_interfaces.rs");
        let mut source_file = fs::File::create(source_path)?;
        writeln!(
            source_file,
            r#"use std::collections::BTreeSet;

/// Generate a list of all `ICoreWebView2...Handler` interfaces declared in `WebView2.h`. This is
/// for testing purposes to make sure they are all covered in [callback.rs](../../src/callback.rs).
pub fn all_declared() -> BTreeSet<&'static str> {{
    let mut interfaces = BTreeSet::new();
"#,
        )?;
        for interface in interfaces {
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

    fn get_callback_interfaces(package_root: &Path) -> super::Result<BTreeSet<String>> {
        let mut include_path = package_root.to_path_buf();
        include_path.push("build");
        include_path.push("native");
        include_path.push("include");
        include_path.push("WebView2.h");
        let mut contents = String::new();
        fs::File::open(include_path)?.read_to_string(&mut contents)?;
        let pattern =
            Regex::new(r"^\s*typedef\s+interface\s+(ICoreWebView2[A-Za-z0-9]+Handler)\s+")?;
        let interfaces: BTreeSet<String> = contents
            .lines()
            .filter_map(|line| pattern.captures(line))
            .filter_map(|captures| captures.get(1))
            .map(|match_1| String::from(match_1.as_str()))
            .collect();
        if interfaces.is_empty() {
            Err(super::Error::MissingTypedef)
        } else {
            Ok(interfaces)
        }
    }
}

mod webview2_bindgen {
    use std::{
        fs,
        io::{Read, Write},
        path::{Path, PathBuf},
    };

    use regex::Regex;

    use windows_bindgen::bindgen;

    use super::webview2_path::*;

    pub fn update_bindings() -> super::Result<bool> {
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

    fn generate_bindings() -> super::Result<PathBuf> {
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

    fn patch_bindings(bindings: String) -> super::Result<String> {
        let pattern = Regex::new(r#"#\s*\[\s*link\s*\(\s*name\s*=\s*"webview2loader"\s*\)\s*\]"#)?;
        let replacement = r#"
            #[cfg_attr(target_env = "msvc", link(name = "WebView2LoaderStatic", kind = "static"))]
            #[cfg_attr(not(target_env = "msvc"), link(name = "WebView2Loader.dll"))]
        "#;
        Ok(pattern.replace_all(&bindings, replacement).to_string())
    }

    fn format_bindings(source_path: &Path) -> super::Result<()> {
        let mut cmd = ::std::process::Command::new("rustfmt");
        cmd.arg(source_path);
        cmd.output()?;
        Ok(())
    }

    fn read_bindings(source_path: &Path) -> super::Result<String> {
        let mut source_file = fs::File::open(source_path)?;
        let mut updated = String::default();
        source_file.read_to_string(&mut updated)?;
        Ok(updated)
    }
}
