fn main() -> webview2_nuget::Result<()> {
    windows::build! {
        Microsoft::Web::WebView2::Win32::*,
        Windows::Win32::Foundation::{
            E_NOINTERFACE, E_POINTER, HINSTANCE, LRESULT, POINT, PWSTR, RECT, SIZE, S_OK,
        },
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx},
        Windows::Win32::System::{
            Com::{CoTaskMemAlloc, CoTaskMemFree},
            LibraryLoader::GetModuleHandleA,
            Threading::GetCurrentThreadId,
            WinRT::EventRegistrationToken,
        },
        Windows::Win32::UI::{
            HiDpi::{SetProcessDpiAwareness, PROCESS_DPI_AWARENESS},
            KeyboardAndMouseInput::SetFocus,
            WindowsAndMessaging::*,
        },
    };

    let package_root = webview2_nuget::install()?;
    webview2_nuget::update_windows(&package_root)?;
    webview2_nuget::update_browser_version(&package_root)?;

    Ok(())
}

#[macro_use]
extern crate thiserror;

mod webview2_nuget {
    use std::{
        convert::From,
        env, fs,
        io::{self, Read, Write},
        path::{Path, PathBuf},
        process::Command,
    };

    use regex::Regex;

    const WEBVIEW2_NAME: &str = "Microsoft.Web.WebView2";
    const WEBVIEW2_VERSION: &str = "1.0.902.49";

    #[cfg(not(windows))]
    pub fn install() -> Result<PathBuf> {
        get_manifest_dir()
    }

    #[cfg(windows)]
    pub fn install() -> Result<PathBuf> {
        let manifest_dir = get_manifest_dir()?;
        let install_root = match manifest_dir.to_str() {
            Some(path) => path,
            None => return Err(Error::MissingPath(manifest_dir)),
        };

        let mut package_root = manifest_dir.clone();
        package_root.push(format!("{}.{}", WEBVIEW2_NAME, WEBVIEW2_VERSION));

        if !check_nuget_dir(install_root)? {
            let mut nuget_path = manifest_dir.clone();
            nuget_path.push("tools");
            nuget_path.push("nuget.exe");

            let nuget_tool = match nuget_path.to_str() {
                Some(path) => path,
                None => return Err(Error::MissingPath(nuget_path)),
            };

            Command::new(nuget_tool)
                .args(&[
                    "install",
                    WEBVIEW2_NAME,
                    "-OutputDirectory",
                    install_root,
                    "-Version",
                    WEBVIEW2_VERSION,
                ])
                .output()?;

            if !check_nuget_dir(install_root)? {
                return Err(Error::MissingPath(package_root));
            }
        }

        Ok(package_root)
    }

    fn get_manifest_dir() -> Result<PathBuf> {
        Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?))
    }

    #[cfg(windows)]
    fn check_nuget_dir(install_root: &str) -> Result<bool> {
        let nuget_path = format!("{}.{}", WEBVIEW2_NAME, WEBVIEW2_VERSION);
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
    pub fn update_windows(_: &Path) -> Result<()> {
        Ok(())
    }

    #[cfg(windows)]
    pub fn update_windows(package_root: &Path) -> Result<()> {
        const WEBVIEW2_STATIC_LIB: &str = "WebView2LoaderStatic.lib";
        const WEBVIEW2_TARGETS: &[&str] = &["arm64", "x64", "x86"];

        let mut windows_dir = get_workspace_dir()?;
        windows_dir.push(".windows");

        let mut native_dir = package_root.to_path_buf();
        native_dir.push("build");
        native_dir.push("native");
        for target in WEBVIEW2_TARGETS {
            let mut lib_dest = windows_dir.clone();
            lib_dest.push(target);
            lib_dest.push(WEBVIEW2_STATIC_LIB);
            let mut lib_src = native_dir.clone();
            lib_src.push(target);
            lib_src.push(WEBVIEW2_STATIC_LIB);
            eprintln!("Copy from {:?} -> {:?}", lib_src, lib_dest);
            fs::copy(lib_src.as_path(), lib_dest.as_path())?;
        }

        Ok(())
    }

    fn get_workspace_dir() -> Result<PathBuf> {
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct CargoMetadata {
            workspace_root: String,
        }

        let output = Command::new(env::var("CARGO")?)
            .args(&["metadata", "--format-version=1", "--no-deps", "--offline"])
            .output()?;

        let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)?;

        Ok(PathBuf::from(metadata.workspace_root))
    }

    #[cfg(not(windows))]
    pub fn update_browser_version(_: &PathBuf) -> Result<bool> {
        Ok(false)
    }

    #[cfg(windows)]
    pub fn update_browser_version(package_root: &Path) -> Result<bool> {
        let version = get_target_broweser_version(package_root)?;
        let mut source_path = get_manifest_dir()?;
        source_path.push("src");
        source_path.push("browser_version.rs");
        let mut source_file = fs::File::create(source_path)?;
        writeln!(
            source_file,
            r#"pub const CORE_WEBVIEW_TARGET_PRODUCT_VERSION: &str = "{}";"#,
            version
        )?;
        Ok(true)
    }

    #[cfg(windows)]
    fn get_target_broweser_version(package_root: &Path) -> Result<String> {
        let mut include_path = package_root.to_path_buf();
        include_path.push("build");
        include_path.push("native");
        include_path.push("include");
        include_path.push("WebView2EnvironmentOptions.h");
        let mut contents = String::new();
        fs::File::open(include_path)?.read_to_string(&mut contents)?;
        let pattern =
            Regex::new(r#"^\s*#define\s+CORE_WEBVIEW_TARGET_PRODUCT_VERSION\s+L"(.*)"\s*$"#)?;
        match contents
            .lines()
            .filter_map(|line| pattern.captures(line))
            .filter_map(|captures| captures.get(1))
            .next()
        {
            Some(capture) => Ok(capture.as_str().into()),
            None => Err(Error::MissingVersion),
        }
    }

    #[derive(Debug, Error)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] io::Error),
        #[error(transparent)]
        Var(#[from] env::VarError),
        #[error(transparent)]
        Json(#[from] serde_json::Error),
        #[error(transparent)]
        Regex(#[from] regex::Error),
        #[error("Missing Version")]
        MissingVersion,
        #[error("Missing Path")]
        MissingPath(PathBuf),
    }

    pub type Result<T> = std::result::Result<T, Error>;
}
