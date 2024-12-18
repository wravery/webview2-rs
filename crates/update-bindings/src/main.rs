extern crate windows_bindgen;

fn main() -> Result<()> {
    match webview2_nuget::install() {
        Ok(package_root) => {
            webview2_nuget::update_libs(&package_root)?;

            if webview2_nuget::update_declared_interfaces(&package_root)? {
                println!("declared_interfaces.rs changed");
            }
        }
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }

    if webview2_bindgen::update_bindings()? {
        println!("Microsoft.rs changed");
    }

    if safe_bindgen::output_functions()? {
        println!("webview2.rs changed");
    }

    Ok(())
}

#[macro_use]
extern crate thiserror;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing Parent")]
    MissingParent(std::path::PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    Syn(#[from] syn::Error),
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

mod webview2_bindgen;
mod webview2_nuget;
mod webview2_path;

mod safe_bindgen;
