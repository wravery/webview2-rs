use std::sync::mpsc;

use windows::{implement, HRESULT};

use crate::{
    pwstr::string_from_pwstr,
    webview2::*,
    Microsoft,
    Windows::{self, Win32::Foundation::PWSTR},
};

/// Generic closure signature for [`CompletedCallback`].
pub type CompletedClosure<Arg1, Arg2> = Box<dyn FnOnce(Arg1, Arg2) -> ::windows::Result<()>>;

/// Generic closure signature for [`EventCallback`].
pub type EventClosure<Arg1, Arg2> = Box<dyn FnMut(Arg1, Arg2) -> windows::Result<()>>;

#[completed_callback]
pub struct CreateCoreWebView2EnvironmentCompletedHandler(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    HRESULT,
    ICoreWebView2Environment,
);

#[completed_callback]
pub struct CreateCoreWebView2ControllerCompletedHandler(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    HRESULT,
    ICoreWebView2Controller,
);

#[event_callback]
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    ICoreWebView2,
    ICoreWebView2WebMessageReceivedEventArgs,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    ICoreWebView2,
    ICoreWebView2WebResourceRequestedEventArgs,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    ICoreWebView2,
    ICoreWebView2PermissionRequestedEventArgs,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    ICoreWebView2,
    ICoreWebView2NavigationCompletedEventArgs,
);

#[string_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
);

#[string_callback]
pub struct ExecuteScriptCompletedHandler(ICoreWebView2ExecuteScriptCompletedHandler);
