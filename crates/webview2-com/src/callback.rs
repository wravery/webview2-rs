use windows::{
    core::{IUnknown, Interface, HRESULT, PCWSTR},
    Win32::{Foundation::BOOL, System::Com::IStream},
};

use windows_implement::implement;

use crate::{pwstr::string_from_pcwstr, Microsoft::Web::WebView2::Win32::*};

pub trait ClosureArg {
    type Output: Sized;
}

pub trait InvokeArg<'a> {
    type Input: 'a;

    fn convert(input: Self::Input) -> <Self as ClosureArg>::Output
    where
        Self: ClosureArg;
}

impl ClosureArg for HRESULT {
    type Output = windows::core::Result<()>;
}

impl<'a> InvokeArg<'a> for HRESULT {
    type Input = Self;

    fn convert(input: HRESULT) -> windows::core::Result<()> {
        input.ok()
    }
}

impl ClosureArg for BOOL {
    type Output = bool;
}

impl<'a> InvokeArg<'a> for BOOL {
    type Input = Self;

    fn convert(input: BOOL) -> bool {
        input.as_bool()
    }
}

impl ClosureArg for PCWSTR {
    type Output = String;
}

impl<'a> InvokeArg<'a> for PCWSTR {
    type Input = &'a Self;

    fn convert(input: &PCWSTR) -> String {
        string_from_pcwstr(input)
    }
}

impl<I: Interface> ClosureArg for Option<I> {
    type Output = Self;
}

impl<'a, I: 'a + Interface + Clone> InvokeArg<'a> for Option<I> {
    type Input = &'a Self;

    fn convert(input: &'a Self) -> <Self as ClosureArg>::Output {
        input.clone()
    }
}

/// Generic closure signature for [`completed_callback`].
pub type CompletedClosure<Arg1, Arg2> = Box<
    dyn FnOnce(
        <Arg1 as ClosureArg>::Output,
        <Arg2 as ClosureArg>::Output,
    ) -> ::windows::core::Result<()>,
>;

/// Generic closure signature for [`event_callback`].
pub type EventClosure<Arg1, Arg2> = Box<
    dyn FnMut(
        <Arg1 as ClosureArg>::Output,
        <Arg2 as ClosureArg>::Output,
    ) -> windows::core::Result<()>,
>;

#[event_callback]
pub struct IsDefaultDownloadDialogOpenChangedEventHandler(
    ICoreWebView2IsDefaultDownloadDialogOpenChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct IsDocumentPlayingAudioChangedEventHandler(
    ICoreWebView2IsDocumentPlayingAudioChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct IsMutedChangedEventHandler(
    ICoreWebView2IsMutedChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Environment>,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2Controller>,
);

#[event_callback]
pub struct NewBrowserVersionAvailableEventHandler(
    ICoreWebView2NewBrowserVersionAvailableEventHandler,
    Option<ICoreWebView2Environment>,
    Option<IUnknown>,
);

#[completed_callback]
pub struct CreateCoreWebView2CompositionControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2CompositionControllerCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CompositionController>,
);

#[event_callback]
pub struct CursorChangedEventHandler(
    ICoreWebView2CursorChangedEventHandler,
    Option<ICoreWebView2CompositionController>,
    Option<IUnknown>,
);

#[event_callback]
pub struct ZoomFactorChangedEventHandler(
    ICoreWebView2ZoomFactorChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct MoveFocusRequestedEventHandler(
    ICoreWebView2MoveFocusRequestedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2MoveFocusRequestedEventArgs>,
);

#[event_callback]
pub struct FocusChangedEventHandler(
    ICoreWebView2FocusChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct AcceleratorKeyPressedEventHandler(
    ICoreWebView2AcceleratorKeyPressedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<ICoreWebView2AcceleratorKeyPressedEventArgs>,
);

#[event_callback]
pub struct ProcessInfosChangedEventHandler(
    ICoreWebView2ProcessInfosChangedEventHandler,
    Option<ICoreWebView2Environment>,
    Option<IUnknown>,
);

#[event_callback]
pub struct RasterizationScaleChangedEventHandler(
    ICoreWebView2RasterizationScaleChangedEventHandler,
    Option<ICoreWebView2Controller>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationStartingEventHandler(
    ICoreWebView2NavigationStartingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationStartingEventArgs>,
);

#[event_callback]
pub struct ContentLoadingEventHandler(
    ICoreWebView2ContentLoadingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ContentLoadingEventArgs>,
);

#[event_callback]
pub struct SourceChangedEventHandler(
    ICoreWebView2SourceChangedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2SourceChangedEventArgs>,
);

#[event_callback]
pub struct DOMContentLoadedEventHandler(
    ICoreWebView2DOMContentLoadedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2DOMContentLoadedEventArgs>,
);

#[event_callback]
pub struct HistoryChangedEventHandler(
    ICoreWebView2HistoryChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationCompletedEventArgs>,
);

#[event_callback]
pub struct ScriptDialogOpeningEventHandler(
    ICoreWebView2ScriptDialogOpeningEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ScriptDialogOpeningEventArgs>,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2PermissionRequestedEventArgs>,
);

#[event_callback]
pub struct ProcessFailedEventHandler(
    ICoreWebView2ProcessFailedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ProcessFailedEventArgs>,
);

#[completed_callback]
pub struct PrintToPdfCompletedHandler(ICoreWebView2PrintToPdfCompletedHandler, HRESULT, BOOL);

#[completed_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    HRESULT,
    PCWSTR,
);

#[completed_callback]
pub struct ExecuteScriptCompletedHandler(
    ICoreWebView2ExecuteScriptCompletedHandler,
    HRESULT,
    PCWSTR,
);

#[completed_callback]
pub struct CapturePreviewCompletedHandler(ICoreWebView2CapturePreviewCompletedHandler, HRESULT);

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebMessageReceivedEventArgs>,
);

#[completed_callback]
pub struct CallDevToolsProtocolMethodCompletedHandler(
    ICoreWebView2CallDevToolsProtocolMethodCompletedHandler,
    HRESULT,
    PCWSTR,
);

#[event_callback]
pub struct NewWindowRequestedEventHandler(
    ICoreWebView2NewWindowRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NewWindowRequestedEventArgs>,
);

#[event_callback]
pub struct DocumentTitleChangedEventHandler(
    ICoreWebView2DocumentTitleChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct ContainsFullScreenElementChangedEventHandler(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceRequestedEventArgs>,
);

#[event_callback]
pub struct WebResourceResponseReceivedEventHandler(
    ICoreWebView2WebResourceResponseReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceResponseReceivedEventArgs>,
);

#[completed_callback]
pub struct WebResourceResponseViewGetContentCompletedHandler(
    ICoreWebView2WebResourceResponseViewGetContentCompletedHandler,
    HRESULT,
    Option<IStream>,
);

#[event_callback]
pub struct WindowCloseRequestedEventHandler(
    ICoreWebView2WindowCloseRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<IUnknown>,
);

#[event_callback]
pub struct DownloadStartingEventHandler(
    ICoreWebView2DownloadStartingEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2DownloadStartingEventArgs>,
);

#[event_callback]
pub struct BytesReceivedChangedEventHandler(
    ICoreWebView2BytesReceivedChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct BrowserProcessExitedEventHandler(
    ICoreWebView2BrowserProcessExitedEventHandler,
    Option<ICoreWebView2Environment>,
    Option<ICoreWebView2BrowserProcessExitedEventArgs>,
);

#[event_callback]
pub struct EstimatedEndTimeChangedEventHandler(
    ICoreWebView2EstimatedEndTimeChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct StateChangedEventHandler(
    ICoreWebView2StateChangedEventHandler,
    Option<ICoreWebView2DownloadOperation>,
    Option<IUnknown>,
);

#[event_callback]
pub struct DevToolsProtocolEventReceivedEventHandler(
    ICoreWebView2DevToolsProtocolEventReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2DevToolsProtocolEventReceivedEventArgs>,
);

#[event_callback]
pub struct FrameContentLoadingEventHandler(
    ICoreWebView2FrameContentLoadingEventHandler,
    Option<ICoreWebView2Frame>,
    Option<ICoreWebView2ContentLoadingEventArgs>,
);

#[event_callback]
pub struct FrameCreatedEventHandler(
    ICoreWebView2FrameCreatedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2FrameCreatedEventArgs>,
);

#[event_callback]
pub struct FrameDOMContentLoadedEventHandler(
    ICoreWebView2FrameDOMContentLoadedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<ICoreWebView2DOMContentLoadedEventArgs>,
);

#[event_callback]
pub struct FrameDestroyedEventHandler(
    ICoreWebView2FrameDestroyedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<IUnknown>,
);

#[event_callback]
pub struct FrameNameChangedEventHandler(
    ICoreWebView2FrameNameChangedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<IUnknown>,
);

#[event_callback]
pub struct ClientCertificateRequestedEventHandler(
    ICoreWebView2ClientCertificateRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2ClientCertificateRequestedEventArgs>,
);

#[completed_callback]
pub struct FrameNavigationCompletedEventHandler(
    ICoreWebView2FrameNavigationCompletedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<ICoreWebView2NavigationCompletedEventArgs>,
);

#[event_callback]
pub struct FrameNavigationStartingEventHandler(
    ICoreWebView2FrameNavigationStartingEventHandler,
    Option<ICoreWebView2Frame>,
    Option<ICoreWebView2NavigationStartingEventArgs>,
);

#[event_callback]
pub struct FrameWebMessageReceivedEventHandler(
    ICoreWebView2FrameWebMessageReceivedEventHandler,
    Option<ICoreWebView2Frame>,
    Option<ICoreWebView2WebMessageReceivedEventArgs>,
);

#[completed_callback]
pub struct GetCookiesCompletedHandler(
    ICoreWebView2GetCookiesCompletedHandler,
    HRESULT,
    Option<ICoreWebView2CookieList>,
);

#[completed_callback]
pub struct TrySuspendCompletedHandler(ICoreWebView2TrySuspendCompletedHandler, HRESULT, BOOL);

#[cfg(test)]
mod test {
    use std::{collections::BTreeSet, env, fs::File, io::Read, path::PathBuf};

    use regex::Regex;

    use webview2_com_sys::callback_interfaces;

    #[test]
    fn all_implemented() {
        let mut source_path = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("cargo should set CARGO_MANIFEST_DIR"),
        );
        source_path.push("src");
        source_path.push("callback.rs");
        let mut contents = String::new();
        File::open(source_path)
            .expect("can open the file")
            .read_to_string(&mut contents)
            .expect("can read the file");
        let pattern = Regex::new(r#"(ICoreWebView2[A-Za-z0-9]+Handler)"#).expect("valid regex");
        let implemented: BTreeSet<&str> = contents
            .lines()
            .filter_map(|line| pattern.captures(line))
            .filter_map(|captures| captures.get(1))
            .map(|match_1| match_1.as_str())
            .collect();
        assert_eq!(
            implemented,
            callback_interfaces::all_declared(),
            "all declared interfaces should be implemented"
        );
    }
}
