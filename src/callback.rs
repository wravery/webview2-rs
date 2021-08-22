use std::sync::mpsc;

use windows::{implement, Interface, HRESULT};

use crate::{
    pwstr::string_from_pwstr,
    webview2::*,
    Microsoft,
    Windows::{self, Win32::Foundation::PWSTR},
};

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
    type Output = windows::Result<()>;
}

impl<'a> InvokeArg<'a> for HRESULT {
    type Input = Self;

    fn convert(input: HRESULT) -> windows::Result<()> {
        input.ok()
    }
}

impl ClosureArg for PWSTR {
    type Output = String;
}

impl<'a> InvokeArg<'a> for PWSTR {
    type Input = Self;

    fn convert(input: PWSTR) -> String {
        string_from_pwstr(input)
    }
}

impl<I: Interface> ClosureArg for Option<I> {
    type Output = Self;
}

impl<'a, I: 'a + Interface> InvokeArg<'a> for Option<I> {
    type Input = &'a Self;

    fn convert(input: &'a Self) -> <Self as ClosureArg>::Output {
        input.clone()
    }
}

/// Generic closure signature for [`CompletedCallback`].
pub type CompletedClosure<Arg1, Arg2> = Box<
    dyn FnOnce(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> ::windows::Result<()>,
>;

/// Generic closure signature for [`EventCallback`].
pub type EventClosure<Arg1, Arg2> = Box<
    dyn FnMut(<Arg1 as ClosureArg>::Output, <Arg2 as ClosureArg>::Output) -> windows::Result<()>,
>;

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
pub struct WebMessageReceivedEventHandler(
    ICoreWebView2WebMessageReceivedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebMessageReceivedEventArgs>,
);

#[event_callback]
pub struct WebResourceRequestedEventHandler(
    ICoreWebView2WebResourceRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2WebResourceRequestedEventArgs>,
);

#[event_callback]
pub struct PermissionRequestedEventHandler(
    ICoreWebView2PermissionRequestedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2PermissionRequestedEventArgs>,
);

#[event_callback]
pub struct NavigationCompletedEventHandler(
    ICoreWebView2NavigationCompletedEventHandler,
    Option<ICoreWebView2>,
    Option<ICoreWebView2NavigationCompletedEventArgs>,
);

#[completed_callback]
pub struct AddScriptToExecuteOnDocumentCreatedCompletedHandler(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    HRESULT,
    PWSTR,
);

#[completed_callback]
pub struct ExecuteScriptCompletedHandler(
    ICoreWebView2ExecuteScriptCompletedHandler,
    HRESULT,
    PWSTR,
);
