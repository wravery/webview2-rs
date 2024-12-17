use std::{convert::From, path::PathBuf};

pub fn get_out_dir() -> PathBuf {
    PathBuf::from(env!("OUT_DIR"))
}

pub fn get_manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn get_bindings_dir() -> crate::Result<PathBuf> {
    let manifest_dir = get_manifest_dir();
    let mut bindings_dir = get_manifest_dir().parent().map_or_else(
        || Err(crate::Error::MissingParent(manifest_dir)),
        |parent| Ok(PathBuf::from(parent)),
    )?;
    bindings_dir.push("bindings");
    Ok(bindings_dir)
}

pub fn get_webview2_com_dir() -> crate::Result<PathBuf> {
    let manifest_dir = get_manifest_dir();
    let mut webview2_com_dir = get_manifest_dir().parent().map_or_else(
        || Err(crate::Error::MissingParent(manifest_dir)),
        |parent| Ok(PathBuf::from(parent)),
    )?;
    webview2_com_dir.push("webview2-com");
    Ok(webview2_com_dir)
}
