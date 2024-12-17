use webview2_com_sys::Microsoft::Web::WebView2::Win32::*;

/// [`CompareBrowserVersions`]
pub fn compare_browser_versions<
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
>(
    version1: P0,
    version2: P1,
) -> windows_core::Result<i32> {
    let mut result = Default::default();
    unsafe { CompareBrowserVersions(version1, version2, &mut result) }?;
    Ok(result)
}

/// [`CreateCoreWebView2Environment`]
pub fn create_environment<
    P0: windows_core::Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
>(
    environmentcreatedhandler: P0,
) -> windows_core::Result<()> {
    unsafe { CreateCoreWebView2Environment(environmentcreatedhandler) }
}

/// [`CreateCoreWebView2EnvironmentWithOptions`]
pub fn create_environment_with_options<
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<ICoreWebView2EnvironmentOptions>,
    P3: windows_core::Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
>(
    browserexecutablefolder: P0,
    userdatafolder: P1,
    environmentoptions: P2,
    environmentcreatedhandler: P3,
) -> windows_core::Result<()> {
    unsafe {
        CreateCoreWebView2EnvironmentWithOptions(
            browserexecutablefolder,
            userdatafolder,
            environmentoptions,
            environmentcreatedhandler,
        )
    }
}

/// [`GetAvailableCoreWebView2BrowserVersionString`]
pub fn get_available_browser_version_string<P0: windows_core::Param<windows_core::PCWSTR>>(
    browserexecutablefolder: P0,
) -> windows_core::Result<String> {
    let mut versioninfo = windows_core::PWSTR::null();
    unsafe {
        GetAvailableCoreWebView2BrowserVersionString(browserexecutablefolder, &mut versioninfo)
    }?;
    Ok(crate::pwstr::take_pwstr(versioninfo))
}

/// [`GetAvailableCoreWebView2BrowserVersionStringWithOptions`]
pub fn get_available_browser_version_string_with_options<
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<ICoreWebView2EnvironmentOptions>,
>(
    browserexecutablefolder: P0,
    environmentoptions: P1,
) -> windows_core::Result<String> {
    let mut versioninfo = windows_core::PWSTR::null();
    unsafe {
        GetAvailableCoreWebView2BrowserVersionStringWithOptions(
            browserexecutablefolder,
            environmentoptions,
            &mut versioninfo,
        )
    }?;
    Ok(crate::pwstr::take_pwstr(versioninfo))
}
