use webview2_com_sys::Microsoft::Web::WebView2::Win32::*;

#[derive(Clone)]
pub struct SafeWrapper<T: Clone> {
    value: T,
}

impl<T: Clone> SafeWrapper<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> AsRef<T> for SafeWrapper<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, U> From<&SafeWrapper<T>> for SafeWrapper<U>
where
    T: Clone,
    U: Clone + From<T>,
{
    fn from(value: &SafeWrapper<T>) -> Self {
        let value = value.as_ref().clone();
        Self::new(value.into())
    }
}

/// [`CompareBrowserVersions`]
#[inline]
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
#[inline]
pub fn create_environment<
    P0: windows_core::Param<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
>(
    environmentcreatedhandler: P0,
) -> windows_core::Result<()> {
    unsafe { CreateCoreWebView2Environment(environmentcreatedhandler) }
}

/// [`CreateCoreWebView2EnvironmentWithOptions`]
#[inline]
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
#[inline]
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
#[inline]
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

/// [`ICoreWebView2`]
pub type WebView2 = SafeWrapper<ICoreWebView2>;

impl WebView2 {
    /// [`ICoreWebView2::Settings`]
    #[inline]
    pub fn settings(&self) -> windows_core::Result<ICoreWebView2Settings> {
        let result = unsafe { self.as_ref().Settings() }?;
        Ok(result)
    }

    /// [`ICoreWebView2::Source`]
    #[inline]
    pub fn source(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Source(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2::Navigate`]
    #[inline]
    pub fn navigate<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        uri: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().Navigate(uri) }
    }

    /// [`ICoreWebView2::NavigateToString`]
    #[inline]
    pub fn navigate_to_string<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        htmlcontent: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().NavigateToString(htmlcontent) }
    }

    /// [`ICoreWebView2::add_NavigationStarting`]
    #[inline]
    pub fn add_navigation_starting<
        P0: windows_core::Param<ICoreWebView2NavigationStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NavigationStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_NavigationStarting`]
    #[inline]
    pub fn remove_navigation_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NavigationStarting(token) }
    }

    /// [`ICoreWebView2::add_ContentLoading`]
    #[inline]
    pub fn add_content_loading<P0: windows_core::Param<ICoreWebView2ContentLoadingEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_ContentLoading(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_ContentLoading`]
    #[inline]
    pub fn remove_content_loading(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ContentLoading(token) }
    }

    /// [`ICoreWebView2::add_SourceChanged`]
    #[inline]
    pub fn add_source_changed<P0: windows_core::Param<ICoreWebView2SourceChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_SourceChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_SourceChanged`]
    #[inline]
    pub fn remove_source_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_SourceChanged(token) }
    }

    /// [`ICoreWebView2::add_HistoryChanged`]
    #[inline]
    pub fn add_history_changed<P0: windows_core::Param<ICoreWebView2HistoryChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_HistoryChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_HistoryChanged`]
    #[inline]
    pub fn remove_history_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_HistoryChanged(token) }
    }

    /// [`ICoreWebView2::add_NavigationCompleted`]
    #[inline]
    pub fn add_navigation_completed<
        P0: windows_core::Param<ICoreWebView2NavigationCompletedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NavigationCompleted(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_NavigationCompleted`]
    #[inline]
    pub fn remove_navigation_completed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NavigationCompleted(token) }
    }

    /// [`ICoreWebView2::add_FrameNavigationStarting`]
    #[inline]
    pub fn add_frame_navigation_starting<
        P0: windows_core::Param<ICoreWebView2NavigationStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_FrameNavigationStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_FrameNavigationStarting`]
    #[inline]
    pub fn remove_frame_navigation_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_FrameNavigationStarting(token) }
    }

    /// [`ICoreWebView2::add_FrameNavigationCompleted`]
    #[inline]
    pub fn add_frame_navigation_completed<
        P0: windows_core::Param<ICoreWebView2NavigationCompletedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_FrameNavigationCompleted(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_FrameNavigationCompleted`]
    #[inline]
    pub fn remove_frame_navigation_completed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_FrameNavigationCompleted(token) }
    }

    /// [`ICoreWebView2::add_ScriptDialogOpening`]
    #[inline]
    pub fn add_script_dialog_opening<
        P0: windows_core::Param<ICoreWebView2ScriptDialogOpeningEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ScriptDialogOpening(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_ScriptDialogOpening`]
    #[inline]
    pub fn remove_script_dialog_opening(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ScriptDialogOpening(token) }
    }

    /// [`ICoreWebView2::add_PermissionRequested`]
    #[inline]
    pub fn add_permission_requested<
        P0: windows_core::Param<ICoreWebView2PermissionRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_PermissionRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_PermissionRequested`]
    #[inline]
    pub fn remove_permission_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_PermissionRequested(token) }
    }

    /// [`ICoreWebView2::add_ProcessFailed`]
    #[inline]
    pub fn add_process_failed<P0: windows_core::Param<ICoreWebView2ProcessFailedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_ProcessFailed(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_ProcessFailed`]
    #[inline]
    pub fn remove_process_failed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ProcessFailed(token) }
    }

    /// [`ICoreWebView2::AddScriptToExecuteOnDocumentCreated`]
    #[inline]
    pub fn add_script_to_execute_on_document_created<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler>,
    >(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .AddScriptToExecuteOnDocumentCreated(javascript, handler)
        }
    }

    /// [`ICoreWebView2::RemoveScriptToExecuteOnDocumentCreated`]
    #[inline]
    pub fn remove_script_to_execute_on_document_created<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        id: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveScriptToExecuteOnDocumentCreated(id) }
    }

    /// [`ICoreWebView2::ExecuteScript`]
    #[inline]
    pub fn execute_script<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ExecuteScriptCompletedHandler>,
    >(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ExecuteScript(javascript, handler) }
    }

    /// [`ICoreWebView2::CapturePreview`]
    #[inline]
    pub fn capture_preview<
        P0: windows_core::Param<windows::Win32::System::Com::IStream>,
        P1: windows_core::Param<ICoreWebView2CapturePreviewCompletedHandler>,
    >(
        &self,
        imageformat: COREWEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT,
        imagestream: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CapturePreview(imageformat, imagestream, handler)
        }
    }

    /// [`ICoreWebView2::Reload`]
    #[inline]
    pub fn reload(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Reload() }
    }

    /// [`ICoreWebView2::PostWebMessageAsJson`]
    #[inline]
    pub fn post_web_message_as_json<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        webmessageasjson: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().PostWebMessageAsJson(webmessageasjson) }
    }

    /// [`ICoreWebView2::PostWebMessageAsString`]
    #[inline]
    pub fn post_web_message_as_string<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        webmessageasstring: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().PostWebMessageAsString(webmessageasstring) }
    }

    /// [`ICoreWebView2::add_WebMessageReceived`]
    #[inline]
    pub fn add_web_message_received<
        P0: windows_core::Param<ICoreWebView2WebMessageReceivedEventHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_WebMessageReceived(handler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_WebMessageReceived`]
    #[inline]
    pub fn remove_web_message_received(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_WebMessageReceived(token) }
    }

    /// [`ICoreWebView2::CallDevToolsProtocolMethod`]
    #[inline]
    pub fn call_dev_tools_protocol_method<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ICoreWebView2CallDevToolsProtocolMethodCompletedHandler>,
    >(
        &self,
        methodname: P0,
        parametersasjson: P1,
        handler: P2,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CallDevToolsProtocolMethod(methodname, parametersasjson, handler)
        }
    }

    /// [`ICoreWebView2::BrowserProcessId`]
    #[inline]
    pub fn browser_process_id(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().BrowserProcessId(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2::CanGoBack`]
    #[inline]
    pub fn can_go_back(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut cangoback = Default::default();
        unsafe { self.as_ref().CanGoBack(&mut cangoback) }?;
        Ok(cangoback)
    }

    /// [`ICoreWebView2::CanGoForward`]
    #[inline]
    pub fn can_go_forward(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut cangoforward = Default::default();
        unsafe { self.as_ref().CanGoForward(&mut cangoforward) }?;
        Ok(cangoforward)
    }

    /// [`ICoreWebView2::GoBack`]
    #[inline]
    pub fn go_back(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().GoBack() }
    }

    /// [`ICoreWebView2::GoForward`]
    #[inline]
    pub fn go_forward(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().GoForward() }
    }

    /// [`ICoreWebView2::GetDevToolsProtocolEventReceiver`]
    #[inline]
    pub fn get_dev_tools_protocol_event_receiver<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        eventname: P0,
    ) -> windows_core::Result<ICoreWebView2DevToolsProtocolEventReceiver> {
        let result = unsafe { self.as_ref().GetDevToolsProtocolEventReceiver(eventname) }?;
        Ok(result)
    }

    /// [`ICoreWebView2::Stop`]
    #[inline]
    pub fn stop(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Stop() }
    }

    /// [`ICoreWebView2::add_NewWindowRequested`]
    #[inline]
    pub fn add_new_window_requested<
        P0: windows_core::Param<ICoreWebView2NewWindowRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NewWindowRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_NewWindowRequested`]
    #[inline]
    pub fn remove_new_window_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NewWindowRequested(token) }
    }

    /// [`ICoreWebView2::add_DocumentTitleChanged`]
    #[inline]
    pub fn add_document_title_changed<
        P0: windows_core::Param<ICoreWebView2DocumentTitleChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_DocumentTitleChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_DocumentTitleChanged`]
    #[inline]
    pub fn remove_document_title_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_DocumentTitleChanged(token) }
    }

    /// [`ICoreWebView2::DocumentTitle`]
    #[inline]
    pub fn document_title(&self) -> windows_core::Result<String> {
        let mut title = windows_core::PWSTR::null();
        unsafe { self.as_ref().DocumentTitle(&mut title) }?;
        Ok(crate::pwstr::take_pwstr(title))
    }

    /// [`ICoreWebView2::AddHostObjectToScript`]
    #[inline]
    pub fn add_host_object_to_script<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<windows_core::VARIANT> {
        let mut object = Default::default();
        unsafe { self.as_ref().AddHostObjectToScript(name, &mut object) }?;
        Ok(object)
    }

    /// [`ICoreWebView2::RemoveHostObjectFromScript`]
    #[inline]
    pub fn remove_host_object_from_script<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveHostObjectFromScript(name) }
    }

    /// [`ICoreWebView2::OpenDevToolsWindow`]
    #[inline]
    pub fn open_dev_tools_window(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().OpenDevToolsWindow() }
    }

    /// [`ICoreWebView2::add_ContainsFullScreenElementChanged`]
    #[inline]
    pub fn add_contains_full_screen_element_changed<
        P0: windows_core::Param<ICoreWebView2ContainsFullScreenElementChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ContainsFullScreenElementChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_ContainsFullScreenElementChanged`]
    #[inline]
    pub fn remove_contains_full_screen_element_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ContainsFullScreenElementChanged(token) }
    }

    /// [`ICoreWebView2::ContainsFullScreenElement`]
    #[inline]
    pub fn contains_full_screen_element(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut containsfullscreenelement = Default::default();
        unsafe {
            self.as_ref()
                .ContainsFullScreenElement(&mut containsfullscreenelement)
        }?;
        Ok(containsfullscreenelement)
    }

    /// [`ICoreWebView2::add_WebResourceRequested`]
    #[inline]
    pub fn add_web_resource_requested<
        P0: windows_core::Param<ICoreWebView2WebResourceRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_WebResourceRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_WebResourceRequested`]
    #[inline]
    pub fn remove_web_resource_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_WebResourceRequested(token) }
    }

    /// [`ICoreWebView2::AddWebResourceRequestedFilter`]
    #[inline]
    pub fn add_web_resource_requested_filter<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .AddWebResourceRequestedFilter(uri, resourcecontext)
        }
    }

    /// [`ICoreWebView2::RemoveWebResourceRequestedFilter`]
    #[inline]
    pub fn remove_web_resource_requested_filter<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .RemoveWebResourceRequestedFilter(uri, resourcecontext)
        }
    }

    /// [`ICoreWebView2::add_WindowCloseRequested`]
    #[inline]
    pub fn add_window_close_requested<
        P0: windows_core::Param<ICoreWebView2WindowCloseRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_WindowCloseRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2::remove_WindowCloseRequested`]
    #[inline]
    pub fn remove_window_close_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_WindowCloseRequested(token) }
    }
}

/// [`ICoreWebView2AcceleratorKeyPressedEventArgs`]
pub type WebView2AcceleratorKeyPressedEventArgs =
    SafeWrapper<ICoreWebView2AcceleratorKeyPressedEventArgs>;

impl WebView2AcceleratorKeyPressedEventArgs {
    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::KeyEventKind`]
    #[inline]
    pub fn key_event_kind(&self) -> windows_core::Result<COREWEBVIEW2_KEY_EVENT_KIND> {
        let mut keyeventkind = Default::default();
        unsafe { self.as_ref().KeyEventKind(&mut keyeventkind) }?;
        Ok(keyeventkind)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::VirtualKey`]
    #[inline]
    pub fn virtual_key(&self) -> windows_core::Result<u32> {
        let mut virtualkey = Default::default();
        unsafe { self.as_ref().VirtualKey(&mut virtualkey) }?;
        Ok(virtualkey)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::KeyEventLParam`]
    #[inline]
    pub fn key_event_l_param(&self) -> windows_core::Result<i32> {
        let mut lparam = Default::default();
        unsafe { self.as_ref().KeyEventLParam(&mut lparam) }?;
        Ok(lparam)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::PhysicalKeyStatus`]
    #[inline]
    pub fn physical_key_status(&self) -> windows_core::Result<COREWEBVIEW2_PHYSICAL_KEY_STATUS> {
        let mut physicalkeystatus = Default::default();
        unsafe { self.as_ref().PhysicalKeyStatus(&mut physicalkeystatus) }?;
        Ok(physicalkeystatus)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut handled = Default::default();
        unsafe { self.as_ref().Handled(&mut handled) }?;
        Ok(handled)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        handled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(handled) }
    }
}

/// [`ICoreWebView2AcceleratorKeyPressedEventArgs2`]
pub type WebView2AcceleratorKeyPressedEventArgs2 =
    SafeWrapper<ICoreWebView2AcceleratorKeyPressedEventArgs2>;

impl WebView2AcceleratorKeyPressedEventArgs2 {
    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs2::IsBrowserAcceleratorKeyEnabled`]
    #[inline]
    pub fn is_browser_accelerator_key_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsBrowserAcceleratorKeyEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2AcceleratorKeyPressedEventArgs2::SetIsBrowserAcceleratorKeyEnabled`]
    #[inline]
    pub fn set_is_browser_accelerator_key_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsBrowserAcceleratorKeyEnabled(value) }
    }
}

/// [`ICoreWebView2BasicAuthenticationRequestedEventArgs`]
pub type WebView2BasicAuthenticationRequestedEventArgs =
    SafeWrapper<ICoreWebView2BasicAuthenticationRequestedEventArgs>;

impl WebView2BasicAuthenticationRequestedEventArgs {
    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::Challenge`]
    #[inline]
    pub fn challenge(&self) -> windows_core::Result<String> {
        let mut challenge = windows_core::PWSTR::null();
        unsafe { self.as_ref().Challenge(&mut challenge) }?;
        Ok(crate::pwstr::take_pwstr(challenge))
    }

    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::Response`]
    #[inline]
    pub fn response(&self) -> windows_core::Result<ICoreWebView2BasicAuthenticationResponse> {
        let result = unsafe { self.as_ref().Response() }?;
        Ok(result)
    }

    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut cancel = Default::default();
        unsafe { self.as_ref().Cancel(&mut cancel) }?;
        Ok(cancel)
    }

    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        cancel: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(cancel) }
    }

    /// [`ICoreWebView2BasicAuthenticationRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2BasicAuthenticationResponse`]
pub type WebView2BasicAuthenticationResponse =
    SafeWrapper<ICoreWebView2BasicAuthenticationResponse>;

impl WebView2BasicAuthenticationResponse {
    /// [`ICoreWebView2BasicAuthenticationResponse::UserName`]
    #[inline]
    pub fn user_name(&self) -> windows_core::Result<String> {
        let mut username = windows_core::PWSTR::null();
        unsafe { self.as_ref().UserName(&mut username) }?;
        Ok(crate::pwstr::take_pwstr(username))
    }

    /// [`ICoreWebView2BasicAuthenticationResponse::SetUserName`]
    #[inline]
    pub fn set_user_name<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        username: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetUserName(username) }
    }

    /// [`ICoreWebView2BasicAuthenticationResponse::Password`]
    #[inline]
    pub fn password(&self) -> windows_core::Result<String> {
        let mut password = windows_core::PWSTR::null();
        unsafe { self.as_ref().Password(&mut password) }?;
        Ok(crate::pwstr::take_pwstr(password))
    }

    /// [`ICoreWebView2BasicAuthenticationResponse::SetPassword`]
    #[inline]
    pub fn set_password<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        password: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPassword(password) }
    }
}

/// [`ICoreWebView2BrowserExtension`]
pub type WebView2BrowserExtension = SafeWrapper<ICoreWebView2BrowserExtension>;

impl WebView2BrowserExtension {
    /// [`ICoreWebView2BrowserExtension::Id`]
    #[inline]
    pub fn id(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Id(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2BrowserExtension::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2BrowserExtension::Remove`]
    #[inline]
    pub fn remove<P0: windows_core::Param<ICoreWebView2BrowserExtensionRemoveCompletedHandler>>(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().Remove(handler) }
    }

    /// [`ICoreWebView2BrowserExtension::IsEnabled`]
    #[inline]
    pub fn is_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2BrowserExtension::Enable`]
    #[inline]
    pub fn enable<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
        P1: windows_core::Param<ICoreWebView2BrowserExtensionEnableCompletedHandler>,
    >(
        &self,
        isenabled: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().Enable(isenabled, handler) }
    }
}

/// [`ICoreWebView2BrowserExtensionList`]
pub type WebView2BrowserExtensionList = SafeWrapper<ICoreWebView2BrowserExtensionList>;

impl WebView2BrowserExtensionList {
    /// [`ICoreWebView2BrowserExtensionList::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2BrowserExtensionList::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2BrowserExtension> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2BrowserProcessExitedEventArgs`]
pub type WebView2BrowserProcessExitedEventArgs =
    SafeWrapper<ICoreWebView2BrowserProcessExitedEventArgs>;

impl WebView2BrowserProcessExitedEventArgs {
    /// [`ICoreWebView2BrowserProcessExitedEventArgs::BrowserProcessExitKind`]
    #[inline]
    pub fn browser_process_exit_kind(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_BROWSER_PROCESS_EXIT_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().BrowserProcessExitKind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2BrowserProcessExitedEventArgs::BrowserProcessId`]
    #[inline]
    pub fn browser_process_id(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().BrowserProcessId(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2Certificate`]
pub type WebView2Certificate = SafeWrapper<ICoreWebView2Certificate>;

impl WebView2Certificate {
    /// [`ICoreWebView2Certificate::Subject`]
    #[inline]
    pub fn subject(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Subject(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Certificate::Issuer`]
    #[inline]
    pub fn issuer(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Issuer(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Certificate::ValidFrom`]
    #[inline]
    pub fn valid_from(&self) -> windows_core::Result<f64> {
        let mut value = Default::default();
        unsafe { self.as_ref().ValidFrom(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Certificate::ValidTo`]
    #[inline]
    pub fn valid_to(&self) -> windows_core::Result<f64> {
        let mut value = Default::default();
        unsafe { self.as_ref().ValidTo(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Certificate::DerEncodedSerialNumber`]
    #[inline]
    pub fn der_encoded_serial_number(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DerEncodedSerialNumber(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Certificate::DisplayName`]
    #[inline]
    pub fn display_name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DisplayName(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Certificate::ToPemEncoding`]
    #[inline]
    pub fn to_pem_encoding(&self) -> windows_core::Result<String> {
        let mut pemencodeddata = windows_core::PWSTR::null();
        unsafe { self.as_ref().ToPemEncoding(&mut pemencodeddata) }?;
        Ok(crate::pwstr::take_pwstr(pemencodeddata))
    }

    /// [`ICoreWebView2Certificate::PemEncodedIssuerCertificateChain`]
    #[inline]
    pub fn pem_encoded_issuer_certificate_chain(
        &self,
    ) -> windows_core::Result<ICoreWebView2StringCollection> {
        let result = unsafe { self.as_ref().PemEncodedIssuerCertificateChain() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ClientCertificate`]
pub type WebView2ClientCertificate = SafeWrapper<ICoreWebView2ClientCertificate>;

impl WebView2ClientCertificate {
    /// [`ICoreWebView2ClientCertificate::Subject`]
    #[inline]
    pub fn subject(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Subject(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ClientCertificate::Issuer`]
    #[inline]
    pub fn issuer(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Issuer(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ClientCertificate::ValidFrom`]
    #[inline]
    pub fn valid_from(&self) -> windows_core::Result<f64> {
        let mut value = Default::default();
        unsafe { self.as_ref().ValidFrom(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificate::ValidTo`]
    #[inline]
    pub fn valid_to(&self) -> windows_core::Result<f64> {
        let mut value = Default::default();
        unsafe { self.as_ref().ValidTo(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificate::DerEncodedSerialNumber`]
    #[inline]
    pub fn der_encoded_serial_number(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DerEncodedSerialNumber(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ClientCertificate::DisplayName`]
    #[inline]
    pub fn display_name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DisplayName(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ClientCertificate::ToPemEncoding`]
    #[inline]
    pub fn to_pem_encoding(&self) -> windows_core::Result<String> {
        let mut pemencodeddata = windows_core::PWSTR::null();
        unsafe { self.as_ref().ToPemEncoding(&mut pemencodeddata) }?;
        Ok(crate::pwstr::take_pwstr(pemencodeddata))
    }

    /// [`ICoreWebView2ClientCertificate::PemEncodedIssuerCertificateChain`]
    #[inline]
    pub fn pem_encoded_issuer_certificate_chain(
        &self,
    ) -> windows_core::Result<ICoreWebView2StringCollection> {
        let result = unsafe { self.as_ref().PemEncodedIssuerCertificateChain() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ClientCertificate::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_CLIENT_CERTIFICATE_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Kind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2ClientCertificateCollection`]
pub type WebView2ClientCertificateCollection =
    SafeWrapper<ICoreWebView2ClientCertificateCollection>;

impl WebView2ClientCertificateCollection {
    /// [`ICoreWebView2ClientCertificateCollection::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificateCollection::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2ClientCertificate> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ClientCertificateRequestedEventArgs`]
pub type WebView2ClientCertificateRequestedEventArgs =
    SafeWrapper<ICoreWebView2ClientCertificateRequestedEventArgs>;

impl WebView2ClientCertificateRequestedEventArgs {
    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::Host`]
    #[inline]
    pub fn host(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Host(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::Port`]
    #[inline]
    pub fn port(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Port(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::IsProxy`]
    #[inline]
    pub fn is_proxy(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsProxy(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::AllowedCertificateAuthorities`]
    #[inline]
    pub fn allowed_certificate_authorities(
        &self,
    ) -> windows_core::Result<ICoreWebView2StringCollection> {
        let result = unsafe { self.as_ref().AllowedCertificateAuthorities() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::MutuallyTrustedCertificates`]
    #[inline]
    pub fn mutually_trusted_certificates(
        &self,
    ) -> windows_core::Result<ICoreWebView2ClientCertificateCollection> {
        let result = unsafe { self.as_ref().MutuallyTrustedCertificates() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::SelectedCertificate`]
    #[inline]
    pub fn selected_certificate(&self) -> windows_core::Result<ICoreWebView2ClientCertificate> {
        let result = unsafe { self.as_ref().SelectedCertificate() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::SetSelectedCertificate`]
    #[inline]
    pub fn set_selected_certificate<P0: windows_core::Param<ICoreWebView2ClientCertificate>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSelectedCertificate(value) }
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Cancel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(value) }
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }

    /// [`ICoreWebView2ClientCertificateRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2CompositionController`]
pub type WebView2CompositionController = SafeWrapper<ICoreWebView2CompositionController>;

impl WebView2CompositionController {
    /// [`ICoreWebView2CompositionController::SetRootVisualTarget`]
    #[inline]
    pub fn set_root_visual_target<P0: windows_core::Param<windows_core::IUnknown>>(
        &self,
        target: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetRootVisualTarget(target) }
    }

    /// [`ICoreWebView2CompositionController::SendMouseInput`]
    #[inline]
    pub fn send_mouse_input(
        &self,
        eventkind: COREWEBVIEW2_MOUSE_EVENT_KIND,
        virtualkeys: COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS,
        mousedata: u32,
        point: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SendMouseInput(eventkind, virtualkeys, mousedata, point)
        }
    }

    /// [`ICoreWebView2CompositionController::SendPointerInput`]
    #[inline]
    pub fn send_pointer_input<P0: windows_core::Param<ICoreWebView2PointerInfo>>(
        &self,
        eventkind: COREWEBVIEW2_POINTER_EVENT_KIND,
        pointerinfo: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SendPointerInput(eventkind, pointerinfo) }
    }

    /// [`ICoreWebView2CompositionController::Cursor`]
    #[inline]
    pub fn cursor(&self) -> windows_core::Result<windows::Win32::UI::WindowsAndMessaging::HCURSOR> {
        let mut cursor = Default::default();
        unsafe { self.as_ref().Cursor(&mut cursor) }?;
        Ok(cursor)
    }

    /// [`ICoreWebView2CompositionController::SystemCursorId`]
    #[inline]
    pub fn system_cursor_id(&self) -> windows_core::Result<u32> {
        let mut systemcursorid = Default::default();
        unsafe { self.as_ref().SystemCursorId(&mut systemcursorid) }?;
        Ok(systemcursorid)
    }

    /// [`ICoreWebView2CompositionController::add_CursorChanged`]
    #[inline]
    pub fn add_cursor_changed<P0: windows_core::Param<ICoreWebView2CursorChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_CursorChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2CompositionController::remove_CursorChanged`]
    #[inline]
    pub fn remove_cursor_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_CursorChanged(token) }
    }
}

/// [`ICoreWebView2CompositionController2`]
pub type WebView2CompositionController2 = SafeWrapper<ICoreWebView2CompositionController2>;

impl WebView2CompositionController2 {}

/// [`ICoreWebView2CompositionController3`]
pub type WebView2CompositionController3 = SafeWrapper<ICoreWebView2CompositionController3>;

impl WebView2CompositionController3 {
    /// [`ICoreWebView2CompositionController3::DragEnter`]
    #[inline]
    pub fn drag_enter<P0: windows_core::Param<windows::Win32::System::Com::IDataObject>>(
        &self,
        dataobject: P0,
        keystate: u32,
        point: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<u32> {
        let mut effect = Default::default();
        unsafe {
            self.as_ref()
                .DragEnter(dataobject, keystate, point, &mut effect)
        }?;
        Ok(effect)
    }

    /// [`ICoreWebView2CompositionController3::DragLeave`]
    #[inline]
    pub fn drag_leave(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().DragLeave() }
    }

    /// [`ICoreWebView2CompositionController3::DragOver`]
    #[inline]
    pub fn drag_over(
        &self,
        keystate: u32,
        point: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<u32> {
        let mut effect = Default::default();
        unsafe { self.as_ref().DragOver(keystate, point, &mut effect) }?;
        Ok(effect)
    }

    /// [`ICoreWebView2CompositionController3::Drop`]
    #[inline]
    pub fn drop<P0: windows_core::Param<windows::Win32::System::Com::IDataObject>>(
        &self,
        dataobject: P0,
        keystate: u32,
        point: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<u32> {
        let mut effect = Default::default();
        unsafe { self.as_ref().Drop(dataobject, keystate, point, &mut effect) }?;
        Ok(effect)
    }
}

/// [`ICoreWebView2CompositionController4`]
pub type WebView2CompositionController4 = SafeWrapper<ICoreWebView2CompositionController4>;

impl WebView2CompositionController4 {
    /// [`ICoreWebView2CompositionController4::GetNonClientRegionAtPoint`]
    #[inline]
    pub fn get_non_client_region_at_point(
        &self,
        point: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<COREWEBVIEW2_NON_CLIENT_REGION_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().GetNonClientRegionAtPoint(point, &mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2CompositionController4::QueryNonClientRegion`]
    #[inline]
    pub fn query_non_client_region(
        &self,
        kind: COREWEBVIEW2_NON_CLIENT_REGION_KIND,
    ) -> windows_core::Result<ICoreWebView2RegionRectCollectionView> {
        let result = unsafe { self.as_ref().QueryNonClientRegion(kind) }?;
        Ok(result)
    }

    /// [`ICoreWebView2CompositionController4::add_NonClientRegionChanged`]
    #[inline]
    pub fn add_non_client_region_changed<
        P0: windows_core::Param<ICoreWebView2NonClientRegionChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NonClientRegionChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2CompositionController4::remove_NonClientRegionChanged`]
    #[inline]
    pub fn remove_non_client_region_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NonClientRegionChanged(token) }
    }
}

/// [`ICoreWebView2ContentLoadingEventArgs`]
pub type WebView2ContentLoadingEventArgs = SafeWrapper<ICoreWebView2ContentLoadingEventArgs>;

impl WebView2ContentLoadingEventArgs {
    /// [`ICoreWebView2ContentLoadingEventArgs::IsErrorPage`]
    #[inline]
    pub fn is_error_page(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsErrorPage(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContentLoadingEventArgs::NavigationId`]
    #[inline]
    pub fn navigation_id(&self) -> windows_core::Result<u64> {
        let mut value = Default::default();
        unsafe { self.as_ref().NavigationId(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2ContextMenuItem`]
pub type WebView2ContextMenuItem = SafeWrapper<ICoreWebView2ContextMenuItem>;

impl WebView2ContextMenuItem {
    /// [`ICoreWebView2ContextMenuItem::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuItem::Label`]
    #[inline]
    pub fn label(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Label(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuItem::CommandId`]
    #[inline]
    pub fn command_id(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().CommandId(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuItem::ShortcutKeyDescription`]
    #[inline]
    pub fn shortcut_key_description(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ShortcutKeyDescription(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuItem::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Kind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuItem::SetIsEnabled`]
    #[inline]
    pub fn set_is_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsEnabled(value) }
    }

    /// [`ICoreWebView2ContextMenuItem::IsEnabled`]
    #[inline]
    pub fn is_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuItem::SetIsChecked`]
    #[inline]
    pub fn set_is_checked<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsChecked(value) }
    }

    /// [`ICoreWebView2ContextMenuItem::IsChecked`]
    #[inline]
    pub fn is_checked(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsChecked(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuItem::Children`]
    #[inline]
    pub fn children(&self) -> windows_core::Result<ICoreWebView2ContextMenuItemCollection> {
        let result = unsafe { self.as_ref().Children() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ContextMenuItem::add_CustomItemSelected`]
    #[inline]
    pub fn add_custom_item_selected<
        P0: windows_core::Param<ICoreWebView2CustomItemSelectedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_CustomItemSelected(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2ContextMenuItem::remove_CustomItemSelected`]
    #[inline]
    pub fn remove_custom_item_selected(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_CustomItemSelected(token) }
    }
}

/// [`ICoreWebView2ContextMenuItemCollection`]
pub type WebView2ContextMenuItemCollection = SafeWrapper<ICoreWebView2ContextMenuItemCollection>;

impl WebView2ContextMenuItemCollection {
    /// [`ICoreWebView2ContextMenuItemCollection::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuItemCollection::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2ContextMenuItem> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }

    /// [`ICoreWebView2ContextMenuItemCollection::RemoveValueAtIndex`]
    #[inline]
    pub fn remove_value_at_index(&self, index: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveValueAtIndex(index) }
    }

    /// [`ICoreWebView2ContextMenuItemCollection::InsertValueAtIndex`]
    #[inline]
    pub fn insert_value_at_index<P0: windows_core::Param<ICoreWebView2ContextMenuItem>>(
        &self,
        index: u32,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().InsertValueAtIndex(index, value) }
    }
}

/// [`ICoreWebView2ContextMenuRequestedEventArgs`]
pub type WebView2ContextMenuRequestedEventArgs =
    SafeWrapper<ICoreWebView2ContextMenuRequestedEventArgs>;

impl WebView2ContextMenuRequestedEventArgs {
    /// [`ICoreWebView2ContextMenuRequestedEventArgs::MenuItems`]
    #[inline]
    pub fn menu_items(&self) -> windows_core::Result<ICoreWebView2ContextMenuItemCollection> {
        let result = unsafe { self.as_ref().MenuItems() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::ContextMenuTarget`]
    #[inline]
    pub fn context_menu_target(&self) -> windows_core::Result<ICoreWebView2ContextMenuTarget> {
        let result = unsafe { self.as_ref().ContextMenuTarget() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::Location`]
    #[inline]
    pub fn location(&self) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut value = Default::default();
        unsafe { self.as_ref().Location(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::SetSelectedCommandId`]
    #[inline]
    pub fn set_selected_command_id(&self, value: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSelectedCommandId(value) }
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::SelectedCommandId`]
    #[inline]
    pub fn selected_command_id(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().SelectedCommandId(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ContextMenuTarget`]
pub type WebView2ContextMenuTarget = SafeWrapper<ICoreWebView2ContextMenuTarget>;

impl WebView2ContextMenuTarget {
    /// [`ICoreWebView2ContextMenuTarget::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_CONTEXT_MENU_TARGET_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Kind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::IsEditable`]
    #[inline]
    pub fn is_editable(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsEditable(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::IsRequestedForMainFrame`]
    #[inline]
    pub fn is_requested_for_main_frame(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsRequestedForMainFrame(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::PageUri`]
    #[inline]
    pub fn page_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().PageUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuTarget::FrameUri`]
    #[inline]
    pub fn frame_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FrameUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuTarget::HasLinkUri`]
    #[inline]
    pub fn has_link_uri(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasLinkUri(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::LinkUri`]
    #[inline]
    pub fn link_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().LinkUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuTarget::HasLinkText`]
    #[inline]
    pub fn has_link_text(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasLinkText(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::LinkText`]
    #[inline]
    pub fn link_text(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().LinkText(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuTarget::HasSourceUri`]
    #[inline]
    pub fn has_source_uri(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasSourceUri(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::SourceUri`]
    #[inline]
    pub fn source_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().SourceUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ContextMenuTarget::HasSelection`]
    #[inline]
    pub fn has_selection(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasSelection(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ContextMenuTarget::SelectionText`]
    #[inline]
    pub fn selection_text(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().SelectionText(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2Controller`]
pub type WebView2Controller = SafeWrapper<ICoreWebView2Controller>;

impl WebView2Controller {
    /// [`ICoreWebView2Controller::IsVisible`]
    #[inline]
    pub fn is_visible(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isvisible = Default::default();
        unsafe { self.as_ref().IsVisible(&mut isvisible) }?;
        Ok(isvisible)
    }

    /// [`ICoreWebView2Controller::SetIsVisible`]
    #[inline]
    pub fn set_is_visible<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        isvisible: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsVisible(isvisible) }
    }

    /// [`ICoreWebView2Controller::Bounds`]
    #[inline]
    pub fn bounds(&self) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut bounds = Default::default();
        unsafe { self.as_ref().Bounds(&mut bounds) }?;
        Ok(bounds)
    }

    /// [`ICoreWebView2Controller::SetBounds`]
    #[inline]
    pub fn set_bounds(&self, bounds: windows::Win32::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetBounds(bounds) }
    }

    /// [`ICoreWebView2Controller::ZoomFactor`]
    #[inline]
    pub fn zoom_factor(&self) -> windows_core::Result<f64> {
        let mut zoomfactor = Default::default();
        unsafe { self.as_ref().ZoomFactor(&mut zoomfactor) }?;
        Ok(zoomfactor)
    }

    /// [`ICoreWebView2Controller::SetZoomFactor`]
    #[inline]
    pub fn set_zoom_factor(&self, zoomfactor: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetZoomFactor(zoomfactor) }
    }

    /// [`ICoreWebView2Controller::add_ZoomFactorChanged`]
    #[inline]
    pub fn add_zoom_factor_changed<
        P0: windows_core::Param<ICoreWebView2ZoomFactorChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ZoomFactorChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller::remove_ZoomFactorChanged`]
    #[inline]
    pub fn remove_zoom_factor_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ZoomFactorChanged(token) }
    }

    /// [`ICoreWebView2Controller::SetBoundsAndZoomFactor`]
    #[inline]
    pub fn set_bounds_and_zoom_factor(
        &self,
        bounds: windows::Win32::Foundation::RECT,
        zoomfactor: f64,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetBoundsAndZoomFactor(bounds, zoomfactor) }
    }

    /// [`ICoreWebView2Controller::MoveFocus`]
    #[inline]
    pub fn move_focus(&self, reason: COREWEBVIEW2_MOVE_FOCUS_REASON) -> windows_core::Result<()> {
        unsafe { self.as_ref().MoveFocus(reason) }
    }

    /// [`ICoreWebView2Controller::add_MoveFocusRequested`]
    #[inline]
    pub fn add_move_focus_requested<
        P0: windows_core::Param<ICoreWebView2MoveFocusRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_MoveFocusRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller::remove_MoveFocusRequested`]
    #[inline]
    pub fn remove_move_focus_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_MoveFocusRequested(token) }
    }

    /// [`ICoreWebView2Controller::add_GotFocus`]
    #[inline]
    pub fn add_got_focus<P0: windows_core::Param<ICoreWebView2FocusChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_GotFocus(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller::remove_GotFocus`]
    #[inline]
    pub fn remove_got_focus(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_GotFocus(token) }
    }

    /// [`ICoreWebView2Controller::add_LostFocus`]
    #[inline]
    pub fn add_lost_focus<P0: windows_core::Param<ICoreWebView2FocusChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_LostFocus(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller::remove_LostFocus`]
    #[inline]
    pub fn remove_lost_focus(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_LostFocus(token) }
    }

    /// [`ICoreWebView2Controller::add_AcceleratorKeyPressed`]
    #[inline]
    pub fn add_accelerator_key_pressed<
        P0: windows_core::Param<ICoreWebView2AcceleratorKeyPressedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_AcceleratorKeyPressed(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller::remove_AcceleratorKeyPressed`]
    #[inline]
    pub fn remove_accelerator_key_pressed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_AcceleratorKeyPressed(token) }
    }

    /// [`ICoreWebView2Controller::ParentWindow`]
    #[inline]
    pub fn parent_window(&self) -> windows_core::Result<windows::Win32::Foundation::HWND> {
        let mut parentwindow = Default::default();
        unsafe { self.as_ref().ParentWindow(&mut parentwindow) }?;
        Ok(parentwindow)
    }

    /// [`ICoreWebView2Controller::SetParentWindow`]
    #[inline]
    pub fn set_parent_window<P0: windows_core::Param<windows::Win32::Foundation::HWND>>(
        &self,
        parentwindow: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetParentWindow(parentwindow) }
    }

    /// [`ICoreWebView2Controller::NotifyParentWindowPositionChanged`]
    #[inline]
    pub fn notify_parent_window_position_changed(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().NotifyParentWindowPositionChanged() }
    }

    /// [`ICoreWebView2Controller::Close`]
    #[inline]
    pub fn close(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Close() }
    }

    /// [`ICoreWebView2Controller::CoreWebView2`]
    #[inline]
    pub fn core_web_view2(&self) -> windows_core::Result<ICoreWebView2> {
        let result = unsafe { self.as_ref().CoreWebView2() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Controller2`]
pub type WebView2Controller2 = SafeWrapper<ICoreWebView2Controller2>;

impl WebView2Controller2 {
    /// [`ICoreWebView2Controller2::DefaultBackgroundColor`]
    #[inline]
    pub fn default_background_color(&self) -> windows_core::Result<COREWEBVIEW2_COLOR> {
        let mut value = Default::default();
        unsafe { self.as_ref().DefaultBackgroundColor(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Controller2::SetDefaultBackgroundColor`]
    #[inline]
    pub fn set_default_background_color(
        &self,
        value: COREWEBVIEW2_COLOR,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDefaultBackgroundColor(value) }
    }
}

/// [`ICoreWebView2Controller3`]
pub type WebView2Controller3 = SafeWrapper<ICoreWebView2Controller3>;

impl WebView2Controller3 {
    /// [`ICoreWebView2Controller3::RasterizationScale`]
    #[inline]
    pub fn rasterization_scale(&self) -> windows_core::Result<f64> {
        let mut scale = Default::default();
        unsafe { self.as_ref().RasterizationScale(&mut scale) }?;
        Ok(scale)
    }

    /// [`ICoreWebView2Controller3::SetRasterizationScale`]
    #[inline]
    pub fn set_rasterization_scale(&self, scale: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetRasterizationScale(scale) }
    }

    /// [`ICoreWebView2Controller3::ShouldDetectMonitorScaleChanges`]
    #[inline]
    pub fn should_detect_monitor_scale_changes(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldDetectMonitorScaleChanges(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Controller3::SetShouldDetectMonitorScaleChanges`]
    #[inline]
    pub fn set_should_detect_monitor_scale_changes<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetShouldDetectMonitorScaleChanges(value) }
    }

    /// [`ICoreWebView2Controller3::add_RasterizationScaleChanged`]
    #[inline]
    pub fn add_rasterization_scale_changed<
        P0: windows_core::Param<ICoreWebView2RasterizationScaleChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_RasterizationScaleChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Controller3::remove_RasterizationScaleChanged`]
    #[inline]
    pub fn remove_rasterization_scale_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_RasterizationScaleChanged(token) }
    }

    /// [`ICoreWebView2Controller3::BoundsMode`]
    #[inline]
    pub fn bounds_mode(&self) -> windows_core::Result<COREWEBVIEW2_BOUNDS_MODE> {
        let mut boundsmode = Default::default();
        unsafe { self.as_ref().BoundsMode(&mut boundsmode) }?;
        Ok(boundsmode)
    }

    /// [`ICoreWebView2Controller3::SetBoundsMode`]
    #[inline]
    pub fn set_bounds_mode(
        &self,
        boundsmode: COREWEBVIEW2_BOUNDS_MODE,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetBoundsMode(boundsmode) }
    }
}

/// [`ICoreWebView2Controller4`]
pub type WebView2Controller4 = SafeWrapper<ICoreWebView2Controller4>;

impl WebView2Controller4 {
    /// [`ICoreWebView2Controller4::AllowExternalDrop`]
    #[inline]
    pub fn allow_external_drop(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().AllowExternalDrop(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Controller4::SetAllowExternalDrop`]
    #[inline]
    pub fn set_allow_external_drop<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAllowExternalDrop(value) }
    }
}

/// [`ICoreWebView2ControllerOptions`]
pub type WebView2ControllerOptions = SafeWrapper<ICoreWebView2ControllerOptions>;

impl WebView2ControllerOptions {
    /// [`ICoreWebView2ControllerOptions::ProfileName`]
    #[inline]
    pub fn profile_name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ProfileName(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ControllerOptions::SetProfileName`]
    #[inline]
    pub fn set_profile_name<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetProfileName(value) }
    }

    /// [`ICoreWebView2ControllerOptions::IsInPrivateModeEnabled`]
    #[inline]
    pub fn is_in_private_mode_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsInPrivateModeEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ControllerOptions::SetIsInPrivateModeEnabled`]
    #[inline]
    pub fn set_is_in_private_mode_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsInPrivateModeEnabled(value) }
    }
}

/// [`ICoreWebView2ControllerOptions2`]
pub type WebView2ControllerOptions2 = SafeWrapper<ICoreWebView2ControllerOptions2>;

impl WebView2ControllerOptions2 {
    /// [`ICoreWebView2ControllerOptions2::ScriptLocale`]
    #[inline]
    pub fn script_locale(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ScriptLocale(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ControllerOptions2::SetScriptLocale`]
    #[inline]
    pub fn set_script_locale<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetScriptLocale(value) }
    }
}

/// [`ICoreWebView2Cookie`]
pub type WebView2Cookie = SafeWrapper<ICoreWebView2Cookie>;

impl WebView2Cookie {
    /// [`ICoreWebView2Cookie::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut name = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut name) }?;
        Ok(crate::pwstr::take_pwstr(name))
    }

    /// [`ICoreWebView2Cookie::Value`]
    #[inline]
    pub fn value(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Value(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Cookie::SetValue`]
    #[inline]
    pub fn set_value<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetValue(value) }
    }

    /// [`ICoreWebView2Cookie::Domain`]
    #[inline]
    pub fn domain(&self) -> windows_core::Result<String> {
        let mut domain = windows_core::PWSTR::null();
        unsafe { self.as_ref().Domain(&mut domain) }?;
        Ok(crate::pwstr::take_pwstr(domain))
    }

    /// [`ICoreWebView2Cookie::Path`]
    #[inline]
    pub fn path(&self) -> windows_core::Result<String> {
        let mut path = windows_core::PWSTR::null();
        unsafe { self.as_ref().Path(&mut path) }?;
        Ok(crate::pwstr::take_pwstr(path))
    }

    /// [`ICoreWebView2Cookie::Expires`]
    #[inline]
    pub fn expires(&self) -> windows_core::Result<f64> {
        let mut expires = Default::default();
        unsafe { self.as_ref().Expires(&mut expires) }?;
        Ok(expires)
    }

    /// [`ICoreWebView2Cookie::SetExpires`]
    #[inline]
    pub fn set_expires(&self, expires: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetExpires(expires) }
    }

    /// [`ICoreWebView2Cookie::IsHttpOnly`]
    #[inline]
    pub fn is_http_only(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut ishttponly = Default::default();
        unsafe { self.as_ref().IsHttpOnly(&mut ishttponly) }?;
        Ok(ishttponly)
    }

    /// [`ICoreWebView2Cookie::SetIsHttpOnly`]
    #[inline]
    pub fn set_is_http_only<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        ishttponly: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsHttpOnly(ishttponly) }
    }

    /// [`ICoreWebView2Cookie::SameSite`]
    #[inline]
    pub fn same_site(&self) -> windows_core::Result<COREWEBVIEW2_COOKIE_SAME_SITE_KIND> {
        let mut samesite = Default::default();
        unsafe { self.as_ref().SameSite(&mut samesite) }?;
        Ok(samesite)
    }

    /// [`ICoreWebView2Cookie::SetSameSite`]
    #[inline]
    pub fn set_same_site(
        &self,
        samesite: COREWEBVIEW2_COOKIE_SAME_SITE_KIND,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSameSite(samesite) }
    }

    /// [`ICoreWebView2Cookie::IsSecure`]
    #[inline]
    pub fn is_secure(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut issecure = Default::default();
        unsafe { self.as_ref().IsSecure(&mut issecure) }?;
        Ok(issecure)
    }

    /// [`ICoreWebView2Cookie::SetIsSecure`]
    #[inline]
    pub fn set_is_secure<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        issecure: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsSecure(issecure) }
    }

    /// [`ICoreWebView2Cookie::IsSession`]
    #[inline]
    pub fn is_session(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut issession = Default::default();
        unsafe { self.as_ref().IsSession(&mut issession) }?;
        Ok(issession)
    }
}

/// [`ICoreWebView2CookieList`]
pub type WebView2CookieList = SafeWrapper<ICoreWebView2CookieList>;

impl WebView2CookieList {
    /// [`ICoreWebView2CookieList::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2CookieList::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(&self, index: u32) -> windows_core::Result<ICoreWebView2Cookie> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2CookieManager`]
pub type WebView2CookieManager = SafeWrapper<ICoreWebView2CookieManager>;

impl WebView2CookieManager {
    /// [`ICoreWebView2CookieManager::CreateCookie`]
    #[inline]
    pub fn create_cookie<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        value: P1,
        domain: P2,
        path: P3,
    ) -> windows_core::Result<ICoreWebView2Cookie> {
        let result = unsafe { self.as_ref().CreateCookie(name, value, domain, path) }?;
        Ok(result)
    }

    /// [`ICoreWebView2CookieManager::CopyCookie`]
    #[inline]
    pub fn copy_cookie<P0: windows_core::Param<ICoreWebView2Cookie>>(
        &self,
        cookieparam: P0,
    ) -> windows_core::Result<ICoreWebView2Cookie> {
        let result = unsafe { self.as_ref().CopyCookie(cookieparam) }?;
        Ok(result)
    }

    /// [`ICoreWebView2CookieManager::GetCookies`]
    #[inline]
    pub fn get_cookies<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2GetCookiesCompletedHandler>,
    >(
        &self,
        uri: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetCookies(uri, handler) }
    }

    /// [`ICoreWebView2CookieManager::AddOrUpdateCookie`]
    #[inline]
    pub fn add_or_update_cookie<P0: windows_core::Param<ICoreWebView2Cookie>>(
        &self,
        cookie: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().AddOrUpdateCookie(cookie) }
    }

    /// [`ICoreWebView2CookieManager::DeleteCookie`]
    #[inline]
    pub fn delete_cookie<P0: windows_core::Param<ICoreWebView2Cookie>>(
        &self,
        cookie: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().DeleteCookie(cookie) }
    }

    /// [`ICoreWebView2CookieManager::DeleteCookies`]
    #[inline]
    pub fn delete_cookies<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        uri: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().DeleteCookies(name, uri) }
    }

    /// [`ICoreWebView2CookieManager::DeleteCookiesWithDomainAndPath`]
    #[inline]
    pub fn delete_cookies_with_domain_and_path<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        domain: P1,
        path: P2,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .DeleteCookiesWithDomainAndPath(name, domain, path)
        }
    }

    /// [`ICoreWebView2CookieManager::DeleteAllCookies`]
    #[inline]
    pub fn delete_all_cookies(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().DeleteAllCookies() }
    }
}

/// [`ICoreWebView2CustomSchemeRegistration`]
pub type WebView2CustomSchemeRegistration = SafeWrapper<ICoreWebView2CustomSchemeRegistration>;

impl WebView2CustomSchemeRegistration {
    /// [`ICoreWebView2CustomSchemeRegistration::SchemeName`]
    #[inline]
    pub fn scheme_name(&self) -> windows_core::Result<String> {
        let mut schemename = windows_core::PWSTR::null();
        unsafe { self.as_ref().SchemeName(&mut schemename) }?;
        Ok(crate::pwstr::take_pwstr(schemename))
    }

    /// [`ICoreWebView2CustomSchemeRegistration::TreatAsSecure`]
    #[inline]
    pub fn treat_as_secure(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut treatassecure = Default::default();
        unsafe { self.as_ref().TreatAsSecure(&mut treatassecure) }?;
        Ok(treatassecure)
    }

    /// [`ICoreWebView2CustomSchemeRegistration::SetTreatAsSecure`]
    #[inline]
    pub fn set_treat_as_secure<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTreatAsSecure(value) }
    }

    /// [`ICoreWebView2CustomSchemeRegistration::GetAllowedOrigins`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn get_allowed_origins(
        &self,
        allowedoriginscount: *mut u32,
    ) -> windows_core::Result<*mut windows_core::PWSTR> {
        let mut allowedorigins = std::ptr::null_mut();
        unsafe {
            self.as_ref()
                .GetAllowedOrigins(allowedoriginscount, &mut allowedorigins)
        }?;
        Ok(allowedorigins)
    }

    /// [`ICoreWebView2CustomSchemeRegistration::SetAllowedOrigins`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn set_allowed_origins(
        &self,
        allowedoriginscount: u32,
        allowedorigins: *const windows_core::PCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetAllowedOrigins(allowedoriginscount, allowedorigins)
        }
    }

    /// [`ICoreWebView2CustomSchemeRegistration::HasAuthorityComponent`]
    #[inline]
    pub fn has_authority_component(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut hasauthoritycomponent = Default::default();
        unsafe {
            self.as_ref()
                .HasAuthorityComponent(&mut hasauthoritycomponent)
        }?;
        Ok(hasauthoritycomponent)
    }

    /// [`ICoreWebView2CustomSchemeRegistration::SetHasAuthorityComponent`]
    #[inline]
    pub fn set_has_authority_component<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        hasauthoritycomponent: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetHasAuthorityComponent(hasauthoritycomponent)
        }
    }
}

/// [`ICoreWebView2DOMContentLoadedEventArgs`]
pub type WebView2DOMContentLoadedEventArgs = SafeWrapper<ICoreWebView2DOMContentLoadedEventArgs>;

impl WebView2DOMContentLoadedEventArgs {
    /// [`ICoreWebView2DOMContentLoadedEventArgs::NavigationId`]
    #[inline]
    pub fn navigation_id(&self) -> windows_core::Result<u64> {
        let mut value = Default::default();
        unsafe { self.as_ref().NavigationId(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2Deferral`]
pub type WebView2Deferral = SafeWrapper<ICoreWebView2Deferral>;

impl WebView2Deferral {
    /// [`ICoreWebView2Deferral::Complete`]
    #[inline]
    pub fn complete(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Complete() }
    }
}

/// [`ICoreWebView2DevToolsProtocolEventReceivedEventArgs`]
pub type WebView2DevToolsProtocolEventReceivedEventArgs =
    SafeWrapper<ICoreWebView2DevToolsProtocolEventReceivedEventArgs>;

impl WebView2DevToolsProtocolEventReceivedEventArgs {
    /// [`ICoreWebView2DevToolsProtocolEventReceivedEventArgs::ParameterObjectAsJson`]
    #[inline]
    pub fn parameter_object_as_json(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ParameterObjectAsJson(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2DevToolsProtocolEventReceivedEventArgs2`]
pub type WebView2DevToolsProtocolEventReceivedEventArgs2 =
    SafeWrapper<ICoreWebView2DevToolsProtocolEventReceivedEventArgs2>;

impl WebView2DevToolsProtocolEventReceivedEventArgs2 {
    /// [`ICoreWebView2DevToolsProtocolEventReceivedEventArgs2::SessionId`]
    #[inline]
    pub fn session_id(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().SessionId(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2DevToolsProtocolEventReceiver`]
pub type WebView2DevToolsProtocolEventReceiver =
    SafeWrapper<ICoreWebView2DevToolsProtocolEventReceiver>;

impl WebView2DevToolsProtocolEventReceiver {
    /// [`ICoreWebView2DevToolsProtocolEventReceiver::add_DevToolsProtocolEventReceived`]
    #[inline]
    pub fn add_dev_tools_protocol_event_received<
        P0: windows_core::Param<ICoreWebView2DevToolsProtocolEventReceivedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_DevToolsProtocolEventReceived(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2DevToolsProtocolEventReceiver::remove_DevToolsProtocolEventReceived`]
    #[inline]
    pub fn remove_dev_tools_protocol_event_received(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_DevToolsProtocolEventReceived(token) }
    }
}

/// [`ICoreWebView2DownloadOperation`]
pub type WebView2DownloadOperation = SafeWrapper<ICoreWebView2DownloadOperation>;

impl WebView2DownloadOperation {
    /// [`ICoreWebView2DownloadOperation::add_BytesReceivedChanged`]
    #[inline]
    pub fn add_bytes_received_changed<
        P0: windows_core::Param<ICoreWebView2BytesReceivedChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_BytesReceivedChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2DownloadOperation::remove_BytesReceivedChanged`]
    #[inline]
    pub fn remove_bytes_received_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_BytesReceivedChanged(token) }
    }

    /// [`ICoreWebView2DownloadOperation::add_EstimatedEndTimeChanged`]
    #[inline]
    pub fn add_estimated_end_time_changed<
        P0: windows_core::Param<ICoreWebView2EstimatedEndTimeChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_EstimatedEndTimeChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2DownloadOperation::remove_EstimatedEndTimeChanged`]
    #[inline]
    pub fn remove_estimated_end_time_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_EstimatedEndTimeChanged(token) }
    }

    /// [`ICoreWebView2DownloadOperation::add_StateChanged`]
    #[inline]
    pub fn add_state_changed<P0: windows_core::Param<ICoreWebView2StateChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_StateChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2DownloadOperation::remove_StateChanged`]
    #[inline]
    pub fn remove_state_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_StateChanged(token) }
    }

    /// [`ICoreWebView2DownloadOperation::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2DownloadOperation::ContentDisposition`]
    #[inline]
    pub fn content_disposition(&self) -> windows_core::Result<String> {
        let mut contentdisposition = windows_core::PWSTR::null();
        unsafe { self.as_ref().ContentDisposition(&mut contentdisposition) }?;
        Ok(crate::pwstr::take_pwstr(contentdisposition))
    }

    /// [`ICoreWebView2DownloadOperation::MimeType`]
    #[inline]
    pub fn mime_type(&self) -> windows_core::Result<String> {
        let mut mimetype = windows_core::PWSTR::null();
        unsafe { self.as_ref().MimeType(&mut mimetype) }?;
        Ok(crate::pwstr::take_pwstr(mimetype))
    }

    /// [`ICoreWebView2DownloadOperation::TotalBytesToReceive`]
    #[inline]
    pub fn total_bytes_to_receive(&self) -> windows_core::Result<i64> {
        let mut totalbytestoreceive = Default::default();
        unsafe { self.as_ref().TotalBytesToReceive(&mut totalbytestoreceive) }?;
        Ok(totalbytestoreceive)
    }

    /// [`ICoreWebView2DownloadOperation::BytesReceived`]
    #[inline]
    pub fn bytes_received(&self) -> windows_core::Result<i64> {
        let mut bytesreceived = Default::default();
        unsafe { self.as_ref().BytesReceived(&mut bytesreceived) }?;
        Ok(bytesreceived)
    }

    /// [`ICoreWebView2DownloadOperation::EstimatedEndTime`]
    #[inline]
    pub fn estimated_end_time(&self) -> windows_core::Result<String> {
        let mut estimatedendtime = windows_core::PWSTR::null();
        unsafe { self.as_ref().EstimatedEndTime(&mut estimatedendtime) }?;
        Ok(crate::pwstr::take_pwstr(estimatedendtime))
    }

    /// [`ICoreWebView2DownloadOperation::ResultFilePath`]
    #[inline]
    pub fn result_file_path(&self) -> windows_core::Result<String> {
        let mut resultfilepath = windows_core::PWSTR::null();
        unsafe { self.as_ref().ResultFilePath(&mut resultfilepath) }?;
        Ok(crate::pwstr::take_pwstr(resultfilepath))
    }

    /// [`ICoreWebView2DownloadOperation::State`]
    #[inline]
    pub fn state(&self) -> windows_core::Result<COREWEBVIEW2_DOWNLOAD_STATE> {
        let mut downloadstate = Default::default();
        unsafe { self.as_ref().State(&mut downloadstate) }?;
        Ok(downloadstate)
    }

    /// [`ICoreWebView2DownloadOperation::InterruptReason`]
    #[inline]
    pub fn interrupt_reason(&self) -> windows_core::Result<COREWEBVIEW2_DOWNLOAD_INTERRUPT_REASON> {
        let mut interruptreason = Default::default();
        unsafe { self.as_ref().InterruptReason(&mut interruptreason) }?;
        Ok(interruptreason)
    }

    /// [`ICoreWebView2DownloadOperation::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Cancel() }
    }

    /// [`ICoreWebView2DownloadOperation::Pause`]
    #[inline]
    pub fn pause(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Pause() }
    }

    /// [`ICoreWebView2DownloadOperation::Resume`]
    #[inline]
    pub fn resume(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Resume() }
    }

    /// [`ICoreWebView2DownloadOperation::CanResume`]
    #[inline]
    pub fn can_resume(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut canresume = Default::default();
        unsafe { self.as_ref().CanResume(&mut canresume) }?;
        Ok(canresume)
    }
}

/// [`ICoreWebView2DownloadStartingEventArgs`]
pub type WebView2DownloadStartingEventArgs = SafeWrapper<ICoreWebView2DownloadStartingEventArgs>;

impl WebView2DownloadStartingEventArgs {
    /// [`ICoreWebView2DownloadStartingEventArgs::DownloadOperation`]
    #[inline]
    pub fn download_operation(&self) -> windows_core::Result<ICoreWebView2DownloadOperation> {
        let result = unsafe { self.as_ref().DownloadOperation() }?;
        Ok(result)
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut cancel = Default::default();
        unsafe { self.as_ref().Cancel(&mut cancel) }?;
        Ok(cancel)
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        cancel: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(cancel) }
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::ResultFilePath`]
    #[inline]
    pub fn result_file_path(&self) -> windows_core::Result<String> {
        let mut resultfilepath = windows_core::PWSTR::null();
        unsafe { self.as_ref().ResultFilePath(&mut resultfilepath) }?;
        Ok(crate::pwstr::take_pwstr(resultfilepath))
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::SetResultFilePath`]
    #[inline]
    pub fn set_result_file_path<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        resultfilepath: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetResultFilePath(resultfilepath) }
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut handled = Default::default();
        unsafe { self.as_ref().Handled(&mut handled) }?;
        Ok(handled)
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        handled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(handled) }
    }

    /// [`ICoreWebView2DownloadStartingEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment`]
pub type WebView2Environment = SafeWrapper<ICoreWebView2Environment>;

impl WebView2Environment {
    /// [`ICoreWebView2Environment::CreateCoreWebView2Controller`]
    #[inline]
    pub fn create_controller<
        P0: windows_core::Param<windows::Win32::Foundation::HWND>,
        P1: windows_core::Param<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>,
    >(
        &self,
        parentwindow: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CreateCoreWebView2Controller(parentwindow, handler)
        }
    }

    /// [`ICoreWebView2Environment::CreateWebResourceResponse`]
    #[inline]
    pub fn create_web_resource_response<
        P0: windows_core::Param<windows::Win32::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        content: P0,
        statuscode: i32,
        reasonphrase: P1,
        headers: P2,
    ) -> windows_core::Result<ICoreWebView2WebResourceResponse> {
        let result = unsafe {
            self.as_ref()
                .CreateWebResourceResponse(content, statuscode, reasonphrase, headers)
        }?;
        Ok(result)
    }

    /// [`ICoreWebView2Environment::BrowserVersionString`]
    #[inline]
    pub fn browser_version_string(&self) -> windows_core::Result<String> {
        let mut versioninfo = windows_core::PWSTR::null();
        unsafe { self.as_ref().BrowserVersionString(&mut versioninfo) }?;
        Ok(crate::pwstr::take_pwstr(versioninfo))
    }

    /// [`ICoreWebView2Environment::add_NewBrowserVersionAvailable`]
    #[inline]
    pub fn add_new_browser_version_available<
        P0: windows_core::Param<ICoreWebView2NewBrowserVersionAvailableEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NewBrowserVersionAvailable(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Environment::remove_NewBrowserVersionAvailable`]
    #[inline]
    pub fn remove_new_browser_version_available(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NewBrowserVersionAvailable(token) }
    }
}

/// [`ICoreWebView2Environment10`]
pub type WebView2Environment10 = SafeWrapper<ICoreWebView2Environment10>;

impl WebView2Environment10 {
    /// [`ICoreWebView2Environment10::CreateCoreWebView2ControllerOptions`]
    #[inline]
    pub fn create_controller_options(
        &self,
    ) -> windows_core::Result<ICoreWebView2ControllerOptions> {
        let result = unsafe { self.as_ref().CreateCoreWebView2ControllerOptions() }?;
        Ok(result)
    }

    /// [`ICoreWebView2Environment10::CreateCoreWebView2ControllerWithOptions`]
    #[inline]
    pub fn create_controller_with_options<
        P0: windows_core::Param<windows::Win32::Foundation::HWND>,
        P1: windows_core::Param<ICoreWebView2ControllerOptions>,
        P2: windows_core::Param<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>,
    >(
        &self,
        parentwindow: P0,
        options: P1,
        handler: P2,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CreateCoreWebView2ControllerWithOptions(parentwindow, options, handler)
        }
    }

    /// [`ICoreWebView2Environment10::CreateCoreWebView2CompositionControllerWithOptions`]
    #[inline]
    pub fn create_composition_controller_with_options<
        P0: windows_core::Param<windows::Win32::Foundation::HWND>,
        P1: windows_core::Param<ICoreWebView2ControllerOptions>,
        P2: windows_core::Param<ICoreWebView2CreateCoreWebView2CompositionControllerCompletedHandler>,
    >(
        &self,
        parentwindow: P0,
        options: P1,
        handler: P2,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CreateCoreWebView2CompositionControllerWithOptions(parentwindow, options, handler)
        }
    }
}

/// [`ICoreWebView2Environment11`]
pub type WebView2Environment11 = SafeWrapper<ICoreWebView2Environment11>;

impl WebView2Environment11 {
    /// [`ICoreWebView2Environment11::FailureReportFolderPath`]
    #[inline]
    pub fn failure_report_folder_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FailureReportFolderPath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2Environment12`]
pub type WebView2Environment12 = SafeWrapper<ICoreWebView2Environment12>;

impl WebView2Environment12 {
    /// [`ICoreWebView2Environment12::CreateSharedBuffer`]
    #[inline]
    pub fn create_shared_buffer(
        &self,
        size: u64,
    ) -> windows_core::Result<ICoreWebView2SharedBuffer> {
        let result = unsafe { self.as_ref().CreateSharedBuffer(size) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment13`]
pub type WebView2Environment13 = SafeWrapper<ICoreWebView2Environment13>;

impl WebView2Environment13 {
    /// [`ICoreWebView2Environment13::GetProcessExtendedInfos`]
    #[inline]
    pub fn get_process_extended_infos<
        P0: windows_core::Param<ICoreWebView2GetProcessExtendedInfosCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetProcessExtendedInfos(handler) }
    }
}

/// [`ICoreWebView2Environment14`]
pub type WebView2Environment14 = SafeWrapper<ICoreWebView2Environment14>;

impl WebView2Environment14 {
    /// [`ICoreWebView2Environment14::CreateWebFileSystemFileHandle`]
    #[inline]
    pub fn create_web_file_system_file_handle<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        path: P0,
        permission: COREWEBVIEW2_FILE_SYSTEM_HANDLE_PERMISSION,
    ) -> windows_core::Result<ICoreWebView2FileSystemHandle> {
        let result = unsafe {
            self.as_ref()
                .CreateWebFileSystemFileHandle(path, permission)
        }?;
        Ok(result)
    }

    /// [`ICoreWebView2Environment14::CreateWebFileSystemDirectoryHandle`]
    #[inline]
    pub fn create_web_file_system_directory_handle<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        path: P0,
        permission: COREWEBVIEW2_FILE_SYSTEM_HANDLE_PERMISSION,
    ) -> windows_core::Result<ICoreWebView2FileSystemHandle> {
        let result = unsafe {
            self.as_ref()
                .CreateWebFileSystemDirectoryHandle(path, permission)
        }?;
        Ok(result)
    }

    /// [`ICoreWebView2Environment14::CreateObjectCollection`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn create_object_collection(
        &self,
        length: u32,
        items: *mut Option<windows_core::IUnknown>,
    ) -> windows_core::Result<Option<ICoreWebView2ObjectCollection>> {
        let mut objectcollection = Default::default();
        unsafe {
            self.as_ref()
                .CreateObjectCollection(length, items, &mut objectcollection)
        }?;
        Ok(objectcollection)
    }
}

/// [`ICoreWebView2Environment2`]
pub type WebView2Environment2 = SafeWrapper<ICoreWebView2Environment2>;

impl WebView2Environment2 {
    /// [`ICoreWebView2Environment2::CreateWebResourceRequest`]
    #[inline]
    pub fn create_web_resource_request<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows::Win32::System::Com::IStream>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        uri: P0,
        method: P1,
        postdata: P2,
        headers: P3,
    ) -> windows_core::Result<ICoreWebView2WebResourceRequest> {
        let result = unsafe {
            self.as_ref()
                .CreateWebResourceRequest(uri, method, postdata, headers)
        }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment3`]
pub type WebView2Environment3 = SafeWrapper<ICoreWebView2Environment3>;

impl WebView2Environment3 {
    /// [`ICoreWebView2Environment3::CreateCoreWebView2CompositionController`]
    #[inline]
    pub fn create_composition_controller<
        P0: windows_core::Param<windows::Win32::Foundation::HWND>,
        P1: windows_core::Param<ICoreWebView2CreateCoreWebView2CompositionControllerCompletedHandler>,
    >(
        &self,
        parentwindow: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .CreateCoreWebView2CompositionController(parentwindow, handler)
        }
    }

    /// [`ICoreWebView2Environment3::CreateCoreWebView2PointerInfo`]
    #[inline]
    pub fn create_pointer_info(&self) -> windows_core::Result<ICoreWebView2PointerInfo> {
        let result = unsafe { self.as_ref().CreateCoreWebView2PointerInfo() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment4`]
pub type WebView2Environment4 = SafeWrapper<ICoreWebView2Environment4>;

impl WebView2Environment4 {}

/// [`ICoreWebView2Environment5`]
pub type WebView2Environment5 = SafeWrapper<ICoreWebView2Environment5>;

impl WebView2Environment5 {
    /// [`ICoreWebView2Environment5::add_BrowserProcessExited`]
    #[inline]
    pub fn add_browser_process_exited<
        P0: windows_core::Param<ICoreWebView2BrowserProcessExitedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_BrowserProcessExited(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Environment5::remove_BrowserProcessExited`]
    #[inline]
    pub fn remove_browser_process_exited(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_BrowserProcessExited(token) }
    }
}

/// [`ICoreWebView2Environment6`]
pub type WebView2Environment6 = SafeWrapper<ICoreWebView2Environment6>;

impl WebView2Environment6 {
    /// [`ICoreWebView2Environment6::CreatePrintSettings`]
    #[inline]
    pub fn create_print_settings(&self) -> windows_core::Result<ICoreWebView2PrintSettings> {
        let result = unsafe { self.as_ref().CreatePrintSettings() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment7`]
pub type WebView2Environment7 = SafeWrapper<ICoreWebView2Environment7>;

impl WebView2Environment7 {
    /// [`ICoreWebView2Environment7::UserDataFolder`]
    #[inline]
    pub fn user_data_folder(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().UserDataFolder(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2Environment8`]
pub type WebView2Environment8 = SafeWrapper<ICoreWebView2Environment8>;

impl WebView2Environment8 {
    /// [`ICoreWebView2Environment8::add_ProcessInfosChanged`]
    #[inline]
    pub fn add_process_infos_changed<
        P0: windows_core::Param<ICoreWebView2ProcessInfosChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ProcessInfosChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Environment8::remove_ProcessInfosChanged`]
    #[inline]
    pub fn remove_process_infos_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ProcessInfosChanged(token) }
    }

    /// [`ICoreWebView2Environment8::GetProcessInfos`]
    #[inline]
    pub fn get_process_infos(&self) -> windows_core::Result<ICoreWebView2ProcessInfoCollection> {
        let result = unsafe { self.as_ref().GetProcessInfos() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Environment9`]
pub type WebView2Environment9 = SafeWrapper<ICoreWebView2Environment9>;

impl WebView2Environment9 {
    /// [`ICoreWebView2Environment9::CreateContextMenuItem`]
    #[inline]
    pub fn create_context_menu_item<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows::Win32::System::Com::IStream>,
    >(
        &self,
        label: P0,
        iconstream: P1,
        kind: COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND,
    ) -> windows_core::Result<ICoreWebView2ContextMenuItem> {
        let result = unsafe { self.as_ref().CreateContextMenuItem(label, iconstream, kind) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2EnvironmentOptions`]
pub type WebView2EnvironmentOptions = SafeWrapper<ICoreWebView2EnvironmentOptions>;

impl WebView2EnvironmentOptions {
    /// [`ICoreWebView2EnvironmentOptions::AdditionalBrowserArguments`]
    #[inline]
    pub fn additional_browser_arguments(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().AdditionalBrowserArguments(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2EnvironmentOptions::SetAdditionalBrowserArguments`]
    #[inline]
    pub fn set_additional_browser_arguments<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAdditionalBrowserArguments(value) }
    }

    /// [`ICoreWebView2EnvironmentOptions::Language`]
    #[inline]
    pub fn language(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Language(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2EnvironmentOptions::SetLanguage`]
    #[inline]
    pub fn set_language<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetLanguage(value) }
    }

    /// [`ICoreWebView2EnvironmentOptions::TargetCompatibleBrowserVersion`]
    #[inline]
    pub fn target_compatible_browser_version(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().TargetCompatibleBrowserVersion(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2EnvironmentOptions::SetTargetCompatibleBrowserVersion`]
    #[inline]
    pub fn set_target_compatible_browser_version<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTargetCompatibleBrowserVersion(value) }
    }

    /// [`ICoreWebView2EnvironmentOptions::AllowSingleSignOnUsingOSPrimaryAccount`]
    #[inline]
    pub fn allow_single_sign_on_using_os_primary_account(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut allow = Default::default();
        unsafe {
            self.as_ref()
                .AllowSingleSignOnUsingOSPrimaryAccount(&mut allow)
        }?;
        Ok(allow)
    }

    /// [`ICoreWebView2EnvironmentOptions::SetAllowSingleSignOnUsingOSPrimaryAccount`]
    #[inline]
    pub fn set_allow_single_sign_on_using_os_primary_account<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        allow: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetAllowSingleSignOnUsingOSPrimaryAccount(allow)
        }
    }
}

/// [`ICoreWebView2EnvironmentOptions2`]
pub type WebView2EnvironmentOptions2 = SafeWrapper<ICoreWebView2EnvironmentOptions2>;

impl WebView2EnvironmentOptions2 {
    /// [`ICoreWebView2EnvironmentOptions2::ExclusiveUserDataFolderAccess`]
    #[inline]
    pub fn exclusive_user_data_folder_access(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ExclusiveUserDataFolderAccess(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions2::SetExclusiveUserDataFolderAccess`]
    #[inline]
    pub fn set_exclusive_user_data_folder_access<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetExclusiveUserDataFolderAccess(value) }
    }
}

/// [`ICoreWebView2EnvironmentOptions3`]
pub type WebView2EnvironmentOptions3 = SafeWrapper<ICoreWebView2EnvironmentOptions3>;

impl WebView2EnvironmentOptions3 {
    /// [`ICoreWebView2EnvironmentOptions3::IsCustomCrashReportingEnabled`]
    #[inline]
    pub fn is_custom_crash_reporting_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsCustomCrashReportingEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions3::SetIsCustomCrashReportingEnabled`]
    #[inline]
    pub fn set_is_custom_crash_reporting_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsCustomCrashReportingEnabled(value) }
    }
}

/// [`ICoreWebView2EnvironmentOptions5`]
pub type WebView2EnvironmentOptions5 = SafeWrapper<ICoreWebView2EnvironmentOptions5>;

impl WebView2EnvironmentOptions5 {
    /// [`ICoreWebView2EnvironmentOptions5::EnableTrackingPrevention`]
    #[inline]
    pub fn enable_tracking_prevention(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().EnableTrackingPrevention(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions5::SetEnableTrackingPrevention`]
    #[inline]
    pub fn set_enable_tracking_prevention<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetEnableTrackingPrevention(value) }
    }
}

/// [`ICoreWebView2EnvironmentOptions6`]
pub type WebView2EnvironmentOptions6 = SafeWrapper<ICoreWebView2EnvironmentOptions6>;

impl WebView2EnvironmentOptions6 {
    /// [`ICoreWebView2EnvironmentOptions6::AreBrowserExtensionsEnabled`]
    #[inline]
    pub fn are_browser_extensions_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().AreBrowserExtensionsEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions6::SetAreBrowserExtensionsEnabled`]
    #[inline]
    pub fn set_are_browser_extensions_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAreBrowserExtensionsEnabled(value) }
    }
}

/// [`ICoreWebView2EnvironmentOptions7`]
pub type WebView2EnvironmentOptions7 = SafeWrapper<ICoreWebView2EnvironmentOptions7>;

impl WebView2EnvironmentOptions7 {
    /// [`ICoreWebView2EnvironmentOptions7::ChannelSearchKind`]
    #[inline]
    pub fn channel_search_kind(&self) -> windows_core::Result<COREWEBVIEW2_CHANNEL_SEARCH_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().ChannelSearchKind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions7::SetChannelSearchKind`]
    #[inline]
    pub fn set_channel_search_kind(
        &self,
        value: COREWEBVIEW2_CHANNEL_SEARCH_KIND,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetChannelSearchKind(value) }
    }

    /// [`ICoreWebView2EnvironmentOptions7::ReleaseChannels`]
    #[inline]
    pub fn release_channels(&self) -> windows_core::Result<COREWEBVIEW2_RELEASE_CHANNELS> {
        let mut value = Default::default();
        unsafe { self.as_ref().ReleaseChannels(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions7::SetReleaseChannels`]
    #[inline]
    pub fn set_release_channels(
        &self,
        value: COREWEBVIEW2_RELEASE_CHANNELS,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetReleaseChannels(value) }
    }
}

/// [`ICoreWebView2EnvironmentOptions8`]
pub type WebView2EnvironmentOptions8 = SafeWrapper<ICoreWebView2EnvironmentOptions8>;

impl WebView2EnvironmentOptions8 {
    /// [`ICoreWebView2EnvironmentOptions8::ScrollBarStyle`]
    #[inline]
    pub fn scroll_bar_style(&self) -> windows_core::Result<COREWEBVIEW2_SCROLLBAR_STYLE> {
        let mut value = Default::default();
        unsafe { self.as_ref().ScrollBarStyle(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2EnvironmentOptions8::SetScrollBarStyle`]
    #[inline]
    pub fn set_scroll_bar_style(
        &self,
        value: COREWEBVIEW2_SCROLLBAR_STYLE,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetScrollBarStyle(value) }
    }
}

/// [`ICoreWebView2ExecuteScriptResult`]
pub type WebView2ExecuteScriptResult = SafeWrapper<ICoreWebView2ExecuteScriptResult>;

impl WebView2ExecuteScriptResult {
    /// [`ICoreWebView2ExecuteScriptResult::Succeeded`]
    #[inline]
    pub fn succeeded(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Succeeded(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ExecuteScriptResult::ResultAsJson`]
    #[inline]
    pub fn result_as_json(&self) -> windows_core::Result<String> {
        let mut jsonresult = windows_core::PWSTR::null();
        unsafe { self.as_ref().ResultAsJson(&mut jsonresult) }?;
        Ok(crate::pwstr::take_pwstr(jsonresult))
    }

    /// [`ICoreWebView2ExecuteScriptResult::TryGetResultAsString`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn try_get_result_as_string(
        &self,
        stringresult: *mut windows_core::PWSTR,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().TryGetResultAsString(stringresult, &mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ExecuteScriptResult::Exception`]
    #[inline]
    pub fn exception(&self) -> windows_core::Result<ICoreWebView2ScriptException> {
        let result = unsafe { self.as_ref().Exception() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2File`]
pub type WebView2File = SafeWrapper<ICoreWebView2File>;

impl WebView2File {
    /// [`ICoreWebView2File::Path`]
    #[inline]
    pub fn path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Path(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2FileSystemHandle`]
pub type WebView2FileSystemHandle = SafeWrapper<ICoreWebView2FileSystemHandle>;

impl WebView2FileSystemHandle {
    /// [`ICoreWebView2FileSystemHandle::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_FILE_SYSTEM_HANDLE_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Kind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2FileSystemHandle::Path`]
    #[inline]
    pub fn path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Path(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2FileSystemHandle::Permission`]
    #[inline]
    pub fn permission(&self) -> windows_core::Result<COREWEBVIEW2_FILE_SYSTEM_HANDLE_PERMISSION> {
        let mut value = Default::default();
        unsafe { self.as_ref().Permission(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2Frame`]
pub type WebView2Frame = SafeWrapper<ICoreWebView2Frame>;

impl WebView2Frame {
    /// [`ICoreWebView2Frame::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut name = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut name) }?;
        Ok(crate::pwstr::take_pwstr(name))
    }

    /// [`ICoreWebView2Frame::add_NameChanged`]
    #[inline]
    pub fn add_name_changed<P0: windows_core::Param<ICoreWebView2FrameNameChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_NameChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame::remove_NameChanged`]
    #[inline]
    pub fn remove_name_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NameChanged(token) }
    }

    /// [`ICoreWebView2Frame::AddHostObjectToScriptWithOrigins`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn add_host_object_to_script_with_origins<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        object: *mut windows_core::VARIANT,
        originscount: u32,
        origins: *const windows_core::PCWSTR,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .AddHostObjectToScriptWithOrigins(name, object, originscount, origins)
        }
    }

    /// [`ICoreWebView2Frame::RemoveHostObjectFromScript`]
    #[inline]
    pub fn remove_host_object_from_script<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveHostObjectFromScript(name) }
    }

    /// [`ICoreWebView2Frame::add_Destroyed`]
    #[inline]
    pub fn add_destroyed<P0: windows_core::Param<ICoreWebView2FrameDestroyedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_Destroyed(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame::remove_Destroyed`]
    #[inline]
    pub fn remove_destroyed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_Destroyed(token) }
    }

    /// [`ICoreWebView2Frame::IsDestroyed`]
    #[inline]
    pub fn is_destroyed(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut destroyed = Default::default();
        unsafe { self.as_ref().IsDestroyed(&mut destroyed) }?;
        Ok(destroyed)
    }
}

/// [`ICoreWebView2Frame2`]
pub type WebView2Frame2 = SafeWrapper<ICoreWebView2Frame2>;

impl WebView2Frame2 {
    /// [`ICoreWebView2Frame2::add_NavigationStarting`]
    #[inline]
    pub fn add_navigation_starting<
        P0: windows_core::Param<ICoreWebView2FrameNavigationStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NavigationStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame2::remove_NavigationStarting`]
    #[inline]
    pub fn remove_navigation_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NavigationStarting(token) }
    }

    /// [`ICoreWebView2Frame2::add_ContentLoading`]
    #[inline]
    pub fn add_content_loading<
        P0: windows_core::Param<ICoreWebView2FrameContentLoadingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_ContentLoading(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame2::remove_ContentLoading`]
    #[inline]
    pub fn remove_content_loading(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ContentLoading(token) }
    }

    /// [`ICoreWebView2Frame2::add_NavigationCompleted`]
    #[inline]
    pub fn add_navigation_completed<
        P0: windows_core::Param<ICoreWebView2FrameNavigationCompletedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NavigationCompleted(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame2::remove_NavigationCompleted`]
    #[inline]
    pub fn remove_navigation_completed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NavigationCompleted(token) }
    }

    /// [`ICoreWebView2Frame2::add_DOMContentLoaded`]
    #[inline]
    pub fn add_dom_content_loaded<
        P0: windows_core::Param<ICoreWebView2FrameDOMContentLoadedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_DOMContentLoaded(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame2::remove_DOMContentLoaded`]
    #[inline]
    pub fn remove_dom_content_loaded(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_DOMContentLoaded(token) }
    }

    /// [`ICoreWebView2Frame2::ExecuteScript`]
    #[inline]
    pub fn execute_script<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ExecuteScriptCompletedHandler>,
    >(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ExecuteScript(javascript, handler) }
    }

    /// [`ICoreWebView2Frame2::PostWebMessageAsJson`]
    #[inline]
    pub fn post_web_message_as_json<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        webmessageasjson: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().PostWebMessageAsJson(webmessageasjson) }
    }

    /// [`ICoreWebView2Frame2::PostWebMessageAsString`]
    #[inline]
    pub fn post_web_message_as_string<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        webmessageasstring: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().PostWebMessageAsString(webmessageasstring) }
    }

    /// [`ICoreWebView2Frame2::add_WebMessageReceived`]
    #[inline]
    pub fn add_web_message_received<
        P0: windows_core::Param<ICoreWebView2FrameWebMessageReceivedEventHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_WebMessageReceived(handler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame2::remove_WebMessageReceived`]
    #[inline]
    pub fn remove_web_message_received(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_WebMessageReceived(token) }
    }
}

/// [`ICoreWebView2Frame3`]
pub type WebView2Frame3 = SafeWrapper<ICoreWebView2Frame3>;

impl WebView2Frame3 {
    /// [`ICoreWebView2Frame3::add_PermissionRequested`]
    #[inline]
    pub fn add_permission_requested<
        P0: windows_core::Param<ICoreWebView2FramePermissionRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_PermissionRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame3::remove_PermissionRequested`]
    #[inline]
    pub fn remove_permission_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_PermissionRequested(token) }
    }
}

/// [`ICoreWebView2Frame4`]
pub type WebView2Frame4 = SafeWrapper<ICoreWebView2Frame4>;

impl WebView2Frame4 {
    /// [`ICoreWebView2Frame4::PostSharedBufferToScript`]
    #[inline]
    pub fn post_shared_buffer_to_script<
        P0: windows_core::Param<ICoreWebView2SharedBuffer>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        sharedbuffer: P0,
        access: COREWEBVIEW2_SHARED_BUFFER_ACCESS,
        additionaldataasjson: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .PostSharedBufferToScript(sharedbuffer, access, additionaldataasjson)
        }
    }
}

/// [`ICoreWebView2Frame5`]
pub type WebView2Frame5 = SafeWrapper<ICoreWebView2Frame5>;

impl WebView2Frame5 {
    /// [`ICoreWebView2Frame5::FrameId`]
    #[inline]
    pub fn frame_id(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().FrameId(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2Frame6`]
pub type WebView2Frame6 = SafeWrapper<ICoreWebView2Frame6>;

impl WebView2Frame6 {
    /// [`ICoreWebView2Frame6::add_ScreenCaptureStarting`]
    #[inline]
    pub fn add_screen_capture_starting<
        P0: windows_core::Param<ICoreWebView2FrameScreenCaptureStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ScreenCaptureStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2Frame6::remove_ScreenCaptureStarting`]
    #[inline]
    pub fn remove_screen_capture_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ScreenCaptureStarting(token) }
    }
}

/// [`ICoreWebView2FrameCreatedEventArgs`]
pub type WebView2FrameCreatedEventArgs = SafeWrapper<ICoreWebView2FrameCreatedEventArgs>;

impl WebView2FrameCreatedEventArgs {
    /// [`ICoreWebView2FrameCreatedEventArgs::Frame`]
    #[inline]
    pub fn frame(&self) -> windows_core::Result<ICoreWebView2Frame> {
        let result = unsafe { self.as_ref().Frame() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2FrameInfo`]
pub type WebView2FrameInfo = SafeWrapper<ICoreWebView2FrameInfo>;

impl WebView2FrameInfo {
    /// [`ICoreWebView2FrameInfo::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2FrameInfo::Source`]
    #[inline]
    pub fn source(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Source(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2FrameInfo2`]
pub type WebView2FrameInfo2 = SafeWrapper<ICoreWebView2FrameInfo2>;

impl WebView2FrameInfo2 {
    /// [`ICoreWebView2FrameInfo2::ParentFrameInfo`]
    #[inline]
    pub fn parent_frame_info(&self) -> windows_core::Result<ICoreWebView2FrameInfo> {
        let result = unsafe { self.as_ref().ParentFrameInfo() }?;
        Ok(result)
    }

    /// [`ICoreWebView2FrameInfo2::FrameId`]
    #[inline]
    pub fn frame_id(&self) -> windows_core::Result<u32> {
        let mut id = Default::default();
        unsafe { self.as_ref().FrameId(&mut id) }?;
        Ok(id)
    }

    /// [`ICoreWebView2FrameInfo2::FrameKind`]
    #[inline]
    pub fn frame_kind(&self) -> windows_core::Result<COREWEBVIEW2_FRAME_KIND> {
        let mut kind = Default::default();
        unsafe { self.as_ref().FrameKind(&mut kind) }?;
        Ok(kind)
    }
}

/// [`ICoreWebView2FrameInfoCollection`]
pub type WebView2FrameInfoCollection = SafeWrapper<ICoreWebView2FrameInfoCollection>;

impl WebView2FrameInfoCollection {
    /// [`ICoreWebView2FrameInfoCollection::GetIterator`]
    #[inline]
    pub fn get_iterator(&self) -> windows_core::Result<ICoreWebView2FrameInfoCollectionIterator> {
        let result = unsafe { self.as_ref().GetIterator() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2FrameInfoCollectionIterator`]
pub type WebView2FrameInfoCollectionIterator =
    SafeWrapper<ICoreWebView2FrameInfoCollectionIterator>;

impl WebView2FrameInfoCollectionIterator {
    /// [`ICoreWebView2FrameInfoCollectionIterator::HasCurrent`]
    #[inline]
    pub fn has_current(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasCurrent(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2FrameInfoCollectionIterator::GetCurrent`]
    #[inline]
    pub fn get_current(&self) -> windows_core::Result<ICoreWebView2FrameInfo> {
        let result = unsafe { self.as_ref().GetCurrent() }?;
        Ok(result)
    }

    /// [`ICoreWebView2FrameInfoCollectionIterator::MoveNext`]
    #[inline]
    pub fn move_next(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().MoveNext(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2HttpHeadersCollectionIterator`]
pub type WebView2HttpHeadersCollectionIterator =
    SafeWrapper<ICoreWebView2HttpHeadersCollectionIterator>;

impl WebView2HttpHeadersCollectionIterator {
    /// [`ICoreWebView2HttpHeadersCollectionIterator::GetCurrentHeader`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn get_current_header(
        &self,
        name: *mut windows_core::PWSTR,
    ) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().GetCurrentHeader(name, &mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2HttpHeadersCollectionIterator::HasCurrentHeader`]
    #[inline]
    pub fn has_current_header(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut hascurrent = Default::default();
        unsafe { self.as_ref().HasCurrentHeader(&mut hascurrent) }?;
        Ok(hascurrent)
    }

    /// [`ICoreWebView2HttpHeadersCollectionIterator::MoveNext`]
    #[inline]
    pub fn move_next(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut hasnext = Default::default();
        unsafe { self.as_ref().MoveNext(&mut hasnext) }?;
        Ok(hasnext)
    }
}

/// [`ICoreWebView2HttpRequestHeaders`]
pub type WebView2HttpRequestHeaders = SafeWrapper<ICoreWebView2HttpRequestHeaders>;

impl WebView2HttpRequestHeaders {
    /// [`ICoreWebView2HttpRequestHeaders::GetHeader`]
    #[inline]
    pub fn get_header<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().GetHeader(name, &mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2HttpRequestHeaders::GetHeaders`]
    #[inline]
    pub fn get_headers<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<ICoreWebView2HttpHeadersCollectionIterator> {
        let result = unsafe { self.as_ref().GetHeaders(name) }?;
        Ok(result)
    }

    /// [`ICoreWebView2HttpRequestHeaders::Contains`]
    #[inline]
    pub fn contains<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Contains(name, &mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2HttpRequestHeaders::SetHeader`]
    #[inline]
    pub fn set_header<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        value: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHeader(name, value) }
    }

    /// [`ICoreWebView2HttpRequestHeaders::RemoveHeader`]
    #[inline]
    pub fn remove_header<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveHeader(name) }
    }

    /// [`ICoreWebView2HttpRequestHeaders::GetIterator`]
    #[inline]
    pub fn get_iterator(&self) -> windows_core::Result<ICoreWebView2HttpHeadersCollectionIterator> {
        let result = unsafe { self.as_ref().GetIterator() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2HttpResponseHeaders`]
pub type WebView2HttpResponseHeaders = SafeWrapper<ICoreWebView2HttpResponseHeaders>;

impl WebView2HttpResponseHeaders {
    /// [`ICoreWebView2HttpResponseHeaders::AppendHeader`]
    #[inline]
    pub fn append_header<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        name: P0,
        value: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().AppendHeader(name, value) }
    }

    /// [`ICoreWebView2HttpResponseHeaders::Contains`]
    #[inline]
    pub fn contains<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Contains(name, &mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2HttpResponseHeaders::GetHeader`]
    #[inline]
    pub fn get_header<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().GetHeader(name, &mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2HttpResponseHeaders::GetHeaders`]
    #[inline]
    pub fn get_headers<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        name: P0,
    ) -> windows_core::Result<ICoreWebView2HttpHeadersCollectionIterator> {
        let result = unsafe { self.as_ref().GetHeaders(name) }?;
        Ok(result)
    }

    /// [`ICoreWebView2HttpResponseHeaders::GetIterator`]
    #[inline]
    pub fn get_iterator(&self) -> windows_core::Result<ICoreWebView2HttpHeadersCollectionIterator> {
        let result = unsafe { self.as_ref().GetIterator() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs`]
pub type WebView2LaunchingExternalUriSchemeEventArgs =
    SafeWrapper<ICoreWebView2LaunchingExternalUriSchemeEventArgs>;

impl WebView2LaunchingExternalUriSchemeEventArgs {
    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::InitiatingOrigin`]
    #[inline]
    pub fn initiating_origin(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().InitiatingOrigin(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::IsUserInitiated`]
    #[inline]
    pub fn is_user_initiated(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsUserInitiated(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Cancel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(value) }
    }

    /// [`ICoreWebView2LaunchingExternalUriSchemeEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2MoveFocusRequestedEventArgs`]
pub type WebView2MoveFocusRequestedEventArgs =
    SafeWrapper<ICoreWebView2MoveFocusRequestedEventArgs>;

impl WebView2MoveFocusRequestedEventArgs {
    /// [`ICoreWebView2MoveFocusRequestedEventArgs::Reason`]
    #[inline]
    pub fn reason(&self) -> windows_core::Result<COREWEBVIEW2_MOVE_FOCUS_REASON> {
        let mut reason = Default::default();
        unsafe { self.as_ref().Reason(&mut reason) }?;
        Ok(reason)
    }

    /// [`ICoreWebView2MoveFocusRequestedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2MoveFocusRequestedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }
}

/// [`ICoreWebView2NavigationCompletedEventArgs`]
pub type WebView2NavigationCompletedEventArgs =
    SafeWrapper<ICoreWebView2NavigationCompletedEventArgs>;

impl WebView2NavigationCompletedEventArgs {
    /// [`ICoreWebView2NavigationCompletedEventArgs::IsSuccess`]
    #[inline]
    pub fn is_success(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut issuccess = Default::default();
        unsafe { self.as_ref().IsSuccess(&mut issuccess) }?;
        Ok(issuccess)
    }

    /// [`ICoreWebView2NavigationCompletedEventArgs::WebErrorStatus`]
    #[inline]
    pub fn web_error_status(&self) -> windows_core::Result<COREWEBVIEW2_WEB_ERROR_STATUS> {
        let mut weberrorstatus = Default::default();
        unsafe { self.as_ref().WebErrorStatus(&mut weberrorstatus) }?;
        Ok(weberrorstatus)
    }

    /// [`ICoreWebView2NavigationCompletedEventArgs::NavigationId`]
    #[inline]
    pub fn navigation_id(&self) -> windows_core::Result<u64> {
        let mut navigationid = Default::default();
        unsafe { self.as_ref().NavigationId(&mut navigationid) }?;
        Ok(navigationid)
    }
}

/// [`ICoreWebView2NavigationCompletedEventArgs2`]
pub type WebView2NavigationCompletedEventArgs2 =
    SafeWrapper<ICoreWebView2NavigationCompletedEventArgs2>;

impl WebView2NavigationCompletedEventArgs2 {
    /// [`ICoreWebView2NavigationCompletedEventArgs2::HttpStatusCode`]
    #[inline]
    pub fn http_status_code(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().HttpStatusCode(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2NavigationStartingEventArgs`]
pub type WebView2NavigationStartingEventArgs =
    SafeWrapper<ICoreWebView2NavigationStartingEventArgs>;

impl WebView2NavigationStartingEventArgs {
    /// [`ICoreWebView2NavigationStartingEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::IsUserInitiated`]
    #[inline]
    pub fn is_user_initiated(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isuserinitiated = Default::default();
        unsafe { self.as_ref().IsUserInitiated(&mut isuserinitiated) }?;
        Ok(isuserinitiated)
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::IsRedirected`]
    #[inline]
    pub fn is_redirected(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isredirected = Default::default();
        unsafe { self.as_ref().IsRedirected(&mut isredirected) }?;
        Ok(isredirected)
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::RequestHeaders`]
    #[inline]
    pub fn request_headers(&self) -> windows_core::Result<ICoreWebView2HttpRequestHeaders> {
        let result = unsafe { self.as_ref().RequestHeaders() }?;
        Ok(result)
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut cancel = Default::default();
        unsafe { self.as_ref().Cancel(&mut cancel) }?;
        Ok(cancel)
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        cancel: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(cancel) }
    }

    /// [`ICoreWebView2NavigationStartingEventArgs::NavigationId`]
    #[inline]
    pub fn navigation_id(&self) -> windows_core::Result<u64> {
        let mut navigationid = Default::default();
        unsafe { self.as_ref().NavigationId(&mut navigationid) }?;
        Ok(navigationid)
    }
}

/// [`ICoreWebView2NavigationStartingEventArgs2`]
pub type WebView2NavigationStartingEventArgs2 =
    SafeWrapper<ICoreWebView2NavigationStartingEventArgs2>;

impl WebView2NavigationStartingEventArgs2 {
    /// [`ICoreWebView2NavigationStartingEventArgs2::AdditionalAllowedFrameAncestors`]
    #[inline]
    pub fn additional_allowed_frame_ancestors(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().AdditionalAllowedFrameAncestors(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2NavigationStartingEventArgs2::SetAdditionalAllowedFrameAncestors`]
    #[inline]
    pub fn set_additional_allowed_frame_ancestors<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAdditionalAllowedFrameAncestors(value) }
    }
}

/// [`ICoreWebView2NavigationStartingEventArgs3`]
pub type WebView2NavigationStartingEventArgs3 =
    SafeWrapper<ICoreWebView2NavigationStartingEventArgs3>;

impl WebView2NavigationStartingEventArgs3 {
    /// [`ICoreWebView2NavigationStartingEventArgs3::NavigationKind`]
    #[inline]
    pub fn navigation_kind(&self) -> windows_core::Result<COREWEBVIEW2_NAVIGATION_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().NavigationKind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2NewWindowRequestedEventArgs`]
pub type WebView2NewWindowRequestedEventArgs =
    SafeWrapper<ICoreWebView2NewWindowRequestedEventArgs>;

impl WebView2NewWindowRequestedEventArgs {
    /// [`ICoreWebView2NewWindowRequestedEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::SetNewWindow`]
    #[inline]
    pub fn set_new_window<P0: windows_core::Param<ICoreWebView2>>(
        &self,
        newwindow: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetNewWindow(newwindow) }
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::NewWindow`]
    #[inline]
    pub fn new_window(&self) -> windows_core::Result<ICoreWebView2> {
        let result = unsafe { self.as_ref().NewWindow() }?;
        Ok(result)
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        handled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(handled) }
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut handled = Default::default();
        unsafe { self.as_ref().Handled(&mut handled) }?;
        Ok(handled)
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::IsUserInitiated`]
    #[inline]
    pub fn is_user_initiated(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isuserinitiated = Default::default();
        unsafe { self.as_ref().IsUserInitiated(&mut isuserinitiated) }?;
        Ok(isuserinitiated)
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }

    /// [`ICoreWebView2NewWindowRequestedEventArgs::WindowFeatures`]
    #[inline]
    pub fn window_features(&self) -> windows_core::Result<ICoreWebView2WindowFeatures> {
        let result = unsafe { self.as_ref().WindowFeatures() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2NewWindowRequestedEventArgs2`]
pub type WebView2NewWindowRequestedEventArgs2 =
    SafeWrapper<ICoreWebView2NewWindowRequestedEventArgs2>;

impl WebView2NewWindowRequestedEventArgs2 {
    /// [`ICoreWebView2NewWindowRequestedEventArgs2::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2NewWindowRequestedEventArgs3`]
pub type WebView2NewWindowRequestedEventArgs3 =
    SafeWrapper<ICoreWebView2NewWindowRequestedEventArgs3>;

impl WebView2NewWindowRequestedEventArgs3 {
    /// [`ICoreWebView2NewWindowRequestedEventArgs3::OriginalSourceFrameInfo`]
    #[inline]
    pub fn original_source_frame_info(&self) -> windows_core::Result<ICoreWebView2FrameInfo> {
        let result = unsafe { self.as_ref().OriginalSourceFrameInfo() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2NonClientRegionChangedEventArgs`]
pub type WebView2NonClientRegionChangedEventArgs =
    SafeWrapper<ICoreWebView2NonClientRegionChangedEventArgs>;

impl WebView2NonClientRegionChangedEventArgs {
    /// [`ICoreWebView2NonClientRegionChangedEventArgs::RegionKind`]
    #[inline]
    pub fn region_kind(&self) -> windows_core::Result<COREWEBVIEW2_NON_CLIENT_REGION_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().RegionKind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2Notification`]
pub type WebView2Notification = SafeWrapper<ICoreWebView2Notification>;

impl WebView2Notification {
    /// [`ICoreWebView2Notification::add_CloseRequested`]
    #[inline]
    pub fn add_close_requested<
        P0: windows_core::Param<ICoreWebView2NotificationCloseRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_CloseRequested(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Notification::remove_CloseRequested`]
    #[inline]
    pub fn remove_close_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_CloseRequested(token) }
    }

    /// [`ICoreWebView2Notification::ReportShown`]
    #[inline]
    pub fn report_shown(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().ReportShown() }
    }

    /// [`ICoreWebView2Notification::ReportClicked`]
    #[inline]
    pub fn report_clicked(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().ReportClicked() }
    }

    /// [`ICoreWebView2Notification::ReportClosed`]
    #[inline]
    pub fn report_closed(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().ReportClosed() }
    }

    /// [`ICoreWebView2Notification::Body`]
    #[inline]
    pub fn body(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Body(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::Direction`]
    #[inline]
    pub fn direction(&self) -> windows_core::Result<COREWEBVIEW2_TEXT_DIRECTION_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Direction(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Notification::Language`]
    #[inline]
    pub fn language(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Language(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::Tag`]
    #[inline]
    pub fn tag(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Tag(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::IconUri`]
    #[inline]
    pub fn icon_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().IconUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::Title`]
    #[inline]
    pub fn title(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Title(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::BadgeUri`]
    #[inline]
    pub fn badge_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().BadgeUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::BodyImageUri`]
    #[inline]
    pub fn body_image_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().BodyImageUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Notification::ShouldRenotify`]
    #[inline]
    pub fn should_renotify(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldRenotify(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Notification::RequiresInteraction`]
    #[inline]
    pub fn requires_interaction(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().RequiresInteraction(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Notification::IsSilent`]
    #[inline]
    pub fn is_silent(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsSilent(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Notification::Timestamp`]
    #[inline]
    pub fn timestamp(&self) -> windows_core::Result<f64> {
        let mut value = Default::default();
        unsafe { self.as_ref().Timestamp(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Notification::GetVibrationPattern`]
    ///
    /// # Safety
    /// This method accesses raw pointer parameters.
    #[inline]
    pub unsafe fn get_vibration_pattern(&self, count: *mut u32) -> windows_core::Result<*mut u64> {
        let mut vibrationpattern = std::ptr::null_mut();
        unsafe {
            self.as_ref()
                .GetVibrationPattern(count, &mut vibrationpattern)
        }?;
        Ok(vibrationpattern)
    }
}

/// [`ICoreWebView2NotificationReceivedEventArgs`]
pub type WebView2NotificationReceivedEventArgs =
    SafeWrapper<ICoreWebView2NotificationReceivedEventArgs>;

impl WebView2NotificationReceivedEventArgs {
    /// [`ICoreWebView2NotificationReceivedEventArgs::SenderOrigin`]
    #[inline]
    pub fn sender_origin(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().SenderOrigin(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2NotificationReceivedEventArgs::Notification`]
    #[inline]
    pub fn notification(&self) -> windows_core::Result<ICoreWebView2Notification> {
        let result = unsafe { self.as_ref().Notification() }?;
        Ok(result)
    }

    /// [`ICoreWebView2NotificationReceivedEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }

    /// [`ICoreWebView2NotificationReceivedEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2NotificationReceivedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ObjectCollection`]
pub type WebView2ObjectCollection = SafeWrapper<ICoreWebView2ObjectCollection>;

impl WebView2ObjectCollection {
    /// [`ICoreWebView2ObjectCollection::RemoveValueAtIndex`]
    #[inline]
    pub fn remove_value_at_index(&self, index: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().RemoveValueAtIndex(index) }
    }

    /// [`ICoreWebView2ObjectCollection::InsertValueAtIndex`]
    #[inline]
    pub fn insert_value_at_index<P0: windows_core::Param<windows_core::IUnknown>>(
        &self,
        index: u32,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().InsertValueAtIndex(index, value) }
    }
}

/// [`ICoreWebView2ObjectCollectionView`]
pub type WebView2ObjectCollectionView = SafeWrapper<ICoreWebView2ObjectCollectionView>;

impl WebView2ObjectCollectionView {
    /// [`ICoreWebView2ObjectCollectionView::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2PermissionRequestedEventArgs`]
pub type WebView2PermissionRequestedEventArgs =
    SafeWrapper<ICoreWebView2PermissionRequestedEventArgs>;

impl WebView2PermissionRequestedEventArgs {
    /// [`ICoreWebView2PermissionRequestedEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs::PermissionKind`]
    #[inline]
    pub fn permission_kind(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_KIND> {
        let mut permissionkind = Default::default();
        unsafe { self.as_ref().PermissionKind(&mut permissionkind) }?;
        Ok(permissionkind)
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs::IsUserInitiated`]
    #[inline]
    pub fn is_user_initiated(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isuserinitiated = Default::default();
        unsafe { self.as_ref().IsUserInitiated(&mut isuserinitiated) }?;
        Ok(isuserinitiated)
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs::State`]
    #[inline]
    pub fn state(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_STATE> {
        let mut state = Default::default();
        unsafe { self.as_ref().State(&mut state) }?;
        Ok(state)
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs::SetState`]
    #[inline]
    pub fn set_state(&self, state: COREWEBVIEW2_PERMISSION_STATE) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetState(state) }
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2PermissionRequestedEventArgs2`]
pub type WebView2PermissionRequestedEventArgs2 =
    SafeWrapper<ICoreWebView2PermissionRequestedEventArgs2>;

impl WebView2PermissionRequestedEventArgs2 {
    /// [`ICoreWebView2PermissionRequestedEventArgs2::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs2::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }
}

/// [`ICoreWebView2PermissionRequestedEventArgs3`]
pub type WebView2PermissionRequestedEventArgs3 =
    SafeWrapper<ICoreWebView2PermissionRequestedEventArgs3>;

impl WebView2PermissionRequestedEventArgs3 {
    /// [`ICoreWebView2PermissionRequestedEventArgs3::SavesInProfile`]
    #[inline]
    pub fn saves_in_profile(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().SavesInProfile(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PermissionRequestedEventArgs3::SetSavesInProfile`]
    #[inline]
    pub fn set_saves_in_profile<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSavesInProfile(value) }
    }
}

/// [`ICoreWebView2PermissionSetting`]
pub type WebView2PermissionSetting = SafeWrapper<ICoreWebView2PermissionSetting>;

impl WebView2PermissionSetting {
    /// [`ICoreWebView2PermissionSetting::PermissionKind`]
    #[inline]
    pub fn permission_kind(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().PermissionKind(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PermissionSetting::PermissionOrigin`]
    #[inline]
    pub fn permission_origin(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().PermissionOrigin(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2PermissionSetting::PermissionState`]
    #[inline]
    pub fn permission_state(&self) -> windows_core::Result<COREWEBVIEW2_PERMISSION_STATE> {
        let mut value = Default::default();
        unsafe { self.as_ref().PermissionState(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2PermissionSettingCollectionView`]
pub type WebView2PermissionSettingCollectionView =
    SafeWrapper<ICoreWebView2PermissionSettingCollectionView>;

impl WebView2PermissionSettingCollectionView {
    /// [`ICoreWebView2PermissionSettingCollectionView::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2PermissionSetting> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }

    /// [`ICoreWebView2PermissionSettingCollectionView::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2PointerInfo`]
pub type WebView2PointerInfo = SafeWrapper<ICoreWebView2PointerInfo>;

impl WebView2PointerInfo {
    /// [`ICoreWebView2PointerInfo::PointerKind`]
    #[inline]
    pub fn pointer_kind(&self) -> windows_core::Result<u32> {
        let mut pointerkind = Default::default();
        unsafe { self.as_ref().PointerKind(&mut pointerkind) }?;
        Ok(pointerkind)
    }

    /// [`ICoreWebView2PointerInfo::SetPointerKind`]
    #[inline]
    pub fn set_pointer_kind(&self, pointerkind: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPointerKind(pointerkind) }
    }

    /// [`ICoreWebView2PointerInfo::PointerId`]
    #[inline]
    pub fn pointer_id(&self) -> windows_core::Result<u32> {
        let mut pointerid = Default::default();
        unsafe { self.as_ref().PointerId(&mut pointerid) }?;
        Ok(pointerid)
    }

    /// [`ICoreWebView2PointerInfo::SetPointerId`]
    #[inline]
    pub fn set_pointer_id(&self, pointerid: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPointerId(pointerid) }
    }

    /// [`ICoreWebView2PointerInfo::FrameId`]
    #[inline]
    pub fn frame_id(&self) -> windows_core::Result<u32> {
        let mut frameid = Default::default();
        unsafe { self.as_ref().FrameId(&mut frameid) }?;
        Ok(frameid)
    }

    /// [`ICoreWebView2PointerInfo::SetFrameId`]
    #[inline]
    pub fn set_frame_id(&self, frameid: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetFrameId(frameid) }
    }

    /// [`ICoreWebView2PointerInfo::PointerFlags`]
    #[inline]
    pub fn pointer_flags(&self) -> windows_core::Result<u32> {
        let mut pointerflags = Default::default();
        unsafe { self.as_ref().PointerFlags(&mut pointerflags) }?;
        Ok(pointerflags)
    }

    /// [`ICoreWebView2PointerInfo::SetPointerFlags`]
    #[inline]
    pub fn set_pointer_flags(&self, pointerflags: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPointerFlags(pointerflags) }
    }

    /// [`ICoreWebView2PointerInfo::PointerDeviceRect`]
    #[inline]
    pub fn pointer_device_rect(&self) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut pointerdevicerect = Default::default();
        unsafe { self.as_ref().PointerDeviceRect(&mut pointerdevicerect) }?;
        Ok(pointerdevicerect)
    }

    /// [`ICoreWebView2PointerInfo::SetPointerDeviceRect`]
    #[inline]
    pub fn set_pointer_device_rect(
        &self,
        pointerdevicerect: windows::Win32::Foundation::RECT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPointerDeviceRect(pointerdevicerect) }
    }

    /// [`ICoreWebView2PointerInfo::DisplayRect`]
    #[inline]
    pub fn display_rect(&self) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut displayrect = Default::default();
        unsafe { self.as_ref().DisplayRect(&mut displayrect) }?;
        Ok(displayrect)
    }

    /// [`ICoreWebView2PointerInfo::SetDisplayRect`]
    #[inline]
    pub fn set_display_rect(
        &self,
        displayrect: windows::Win32::Foundation::RECT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDisplayRect(displayrect) }
    }

    /// [`ICoreWebView2PointerInfo::PixelLocation`]
    #[inline]
    pub fn pixel_location(&self) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut pixellocation = Default::default();
        unsafe { self.as_ref().PixelLocation(&mut pixellocation) }?;
        Ok(pixellocation)
    }

    /// [`ICoreWebView2PointerInfo::SetPixelLocation`]
    #[inline]
    pub fn set_pixel_location(
        &self,
        pixellocation: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPixelLocation(pixellocation) }
    }

    /// [`ICoreWebView2PointerInfo::HimetricLocation`]
    #[inline]
    pub fn himetric_location(&self) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut himetriclocation = Default::default();
        unsafe { self.as_ref().HimetricLocation(&mut himetriclocation) }?;
        Ok(himetriclocation)
    }

    /// [`ICoreWebView2PointerInfo::SetHimetricLocation`]
    #[inline]
    pub fn set_himetric_location(
        &self,
        himetriclocation: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHimetricLocation(himetriclocation) }
    }

    /// [`ICoreWebView2PointerInfo::PixelLocationRaw`]
    #[inline]
    pub fn pixel_location_raw(&self) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut pixellocationraw = Default::default();
        unsafe { self.as_ref().PixelLocationRaw(&mut pixellocationraw) }?;
        Ok(pixellocationraw)
    }

    /// [`ICoreWebView2PointerInfo::SetPixelLocationRaw`]
    #[inline]
    pub fn set_pixel_location_raw(
        &self,
        pixellocationraw: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPixelLocationRaw(pixellocationraw) }
    }

    /// [`ICoreWebView2PointerInfo::HimetricLocationRaw`]
    #[inline]
    pub fn himetric_location_raw(&self) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut himetriclocationraw = Default::default();
        unsafe { self.as_ref().HimetricLocationRaw(&mut himetriclocationraw) }?;
        Ok(himetriclocationraw)
    }

    /// [`ICoreWebView2PointerInfo::SetHimetricLocationRaw`]
    #[inline]
    pub fn set_himetric_location_raw(
        &self,
        himetriclocationraw: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHimetricLocationRaw(himetriclocationraw) }
    }

    /// [`ICoreWebView2PointerInfo::Time`]
    #[inline]
    pub fn time(&self) -> windows_core::Result<u32> {
        let mut time = Default::default();
        unsafe { self.as_ref().Time(&mut time) }?;
        Ok(time)
    }

    /// [`ICoreWebView2PointerInfo::SetTime`]
    #[inline]
    pub fn set_time(&self, time: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTime(time) }
    }

    /// [`ICoreWebView2PointerInfo::HistoryCount`]
    #[inline]
    pub fn history_count(&self) -> windows_core::Result<u32> {
        let mut historycount = Default::default();
        unsafe { self.as_ref().HistoryCount(&mut historycount) }?;
        Ok(historycount)
    }

    /// [`ICoreWebView2PointerInfo::SetHistoryCount`]
    #[inline]
    pub fn set_history_count(&self, historycount: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHistoryCount(historycount) }
    }

    /// [`ICoreWebView2PointerInfo::InputData`]
    #[inline]
    pub fn input_data(&self) -> windows_core::Result<i32> {
        let mut inputdata = Default::default();
        unsafe { self.as_ref().InputData(&mut inputdata) }?;
        Ok(inputdata)
    }

    /// [`ICoreWebView2PointerInfo::SetInputData`]
    #[inline]
    pub fn set_input_data(&self, inputdata: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetInputData(inputdata) }
    }

    /// [`ICoreWebView2PointerInfo::KeyStates`]
    #[inline]
    pub fn key_states(&self) -> windows_core::Result<u32> {
        let mut keystates = Default::default();
        unsafe { self.as_ref().KeyStates(&mut keystates) }?;
        Ok(keystates)
    }

    /// [`ICoreWebView2PointerInfo::SetKeyStates`]
    #[inline]
    pub fn set_key_states(&self, keystates: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetKeyStates(keystates) }
    }

    /// [`ICoreWebView2PointerInfo::PerformanceCount`]
    #[inline]
    pub fn performance_count(&self) -> windows_core::Result<u64> {
        let mut performancecount = Default::default();
        unsafe { self.as_ref().PerformanceCount(&mut performancecount) }?;
        Ok(performancecount)
    }

    /// [`ICoreWebView2PointerInfo::SetPerformanceCount`]
    #[inline]
    pub fn set_performance_count(&self, performancecount: u64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPerformanceCount(performancecount) }
    }

    /// [`ICoreWebView2PointerInfo::ButtonChangeKind`]
    #[inline]
    pub fn button_change_kind(&self) -> windows_core::Result<i32> {
        let mut buttonchangekind = Default::default();
        unsafe { self.as_ref().ButtonChangeKind(&mut buttonchangekind) }?;
        Ok(buttonchangekind)
    }

    /// [`ICoreWebView2PointerInfo::SetButtonChangeKind`]
    #[inline]
    pub fn set_button_change_kind(&self, buttonchangekind: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetButtonChangeKind(buttonchangekind) }
    }

    /// [`ICoreWebView2PointerInfo::PenFlags`]
    #[inline]
    pub fn pen_flags(&self) -> windows_core::Result<u32> {
        let mut penflags = Default::default();
        unsafe { self.as_ref().PenFlags(&mut penflags) }?;
        Ok(penflags)
    }

    /// [`ICoreWebView2PointerInfo::SetPenFlags`]
    #[inline]
    pub fn set_pen_flags(&self, penflags: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenFlags(penflags) }
    }

    /// [`ICoreWebView2PointerInfo::PenMask`]
    #[inline]
    pub fn pen_mask(&self) -> windows_core::Result<u32> {
        let mut penmask = Default::default();
        unsafe { self.as_ref().PenMask(&mut penmask) }?;
        Ok(penmask)
    }

    /// [`ICoreWebView2PointerInfo::SetPenMask`]
    #[inline]
    pub fn set_pen_mask(&self, penmask: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenMask(penmask) }
    }

    /// [`ICoreWebView2PointerInfo::PenPressure`]
    #[inline]
    pub fn pen_pressure(&self) -> windows_core::Result<u32> {
        let mut penpressure = Default::default();
        unsafe { self.as_ref().PenPressure(&mut penpressure) }?;
        Ok(penpressure)
    }

    /// [`ICoreWebView2PointerInfo::SetPenPressure`]
    #[inline]
    pub fn set_pen_pressure(&self, penpressure: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenPressure(penpressure) }
    }

    /// [`ICoreWebView2PointerInfo::PenRotation`]
    #[inline]
    pub fn pen_rotation(&self) -> windows_core::Result<u32> {
        let mut penrotation = Default::default();
        unsafe { self.as_ref().PenRotation(&mut penrotation) }?;
        Ok(penrotation)
    }

    /// [`ICoreWebView2PointerInfo::SetPenRotation`]
    #[inline]
    pub fn set_pen_rotation(&self, penrotation: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenRotation(penrotation) }
    }

    /// [`ICoreWebView2PointerInfo::PenTiltX`]
    #[inline]
    pub fn pen_tilt_x(&self) -> windows_core::Result<i32> {
        let mut pentiltx = Default::default();
        unsafe { self.as_ref().PenTiltX(&mut pentiltx) }?;
        Ok(pentiltx)
    }

    /// [`ICoreWebView2PointerInfo::SetPenTiltX`]
    #[inline]
    pub fn set_pen_tilt_x(&self, pentiltx: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenTiltX(pentiltx) }
    }

    /// [`ICoreWebView2PointerInfo::PenTiltY`]
    #[inline]
    pub fn pen_tilt_y(&self) -> windows_core::Result<i32> {
        let mut pentilty = Default::default();
        unsafe { self.as_ref().PenTiltY(&mut pentilty) }?;
        Ok(pentilty)
    }

    /// [`ICoreWebView2PointerInfo::SetPenTiltY`]
    #[inline]
    pub fn set_pen_tilt_y(&self, pentilty: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPenTiltY(pentilty) }
    }

    /// [`ICoreWebView2PointerInfo::TouchFlags`]
    #[inline]
    pub fn touch_flags(&self) -> windows_core::Result<u32> {
        let mut touchflags = Default::default();
        unsafe { self.as_ref().TouchFlags(&mut touchflags) }?;
        Ok(touchflags)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchFlags`]
    #[inline]
    pub fn set_touch_flags(&self, touchflags: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchFlags(touchflags) }
    }

    /// [`ICoreWebView2PointerInfo::TouchMask`]
    #[inline]
    pub fn touch_mask(&self) -> windows_core::Result<u32> {
        let mut touchmask = Default::default();
        unsafe { self.as_ref().TouchMask(&mut touchmask) }?;
        Ok(touchmask)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchMask`]
    #[inline]
    pub fn set_touch_mask(&self, touchmask: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchMask(touchmask) }
    }

    /// [`ICoreWebView2PointerInfo::TouchContact`]
    #[inline]
    pub fn touch_contact(&self) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut touchcontact = Default::default();
        unsafe { self.as_ref().TouchContact(&mut touchcontact) }?;
        Ok(touchcontact)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchContact`]
    #[inline]
    pub fn set_touch_contact(
        &self,
        touchcontact: windows::Win32::Foundation::RECT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchContact(touchcontact) }
    }

    /// [`ICoreWebView2PointerInfo::TouchContactRaw`]
    #[inline]
    pub fn touch_contact_raw(&self) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut touchcontactraw = Default::default();
        unsafe { self.as_ref().TouchContactRaw(&mut touchcontactraw) }?;
        Ok(touchcontactraw)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchContactRaw`]
    #[inline]
    pub fn set_touch_contact_raw(
        &self,
        touchcontactraw: windows::Win32::Foundation::RECT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchContactRaw(touchcontactraw) }
    }

    /// [`ICoreWebView2PointerInfo::TouchOrientation`]
    #[inline]
    pub fn touch_orientation(&self) -> windows_core::Result<u32> {
        let mut touchorientation = Default::default();
        unsafe { self.as_ref().TouchOrientation(&mut touchorientation) }?;
        Ok(touchorientation)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchOrientation`]
    #[inline]
    pub fn set_touch_orientation(&self, touchorientation: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchOrientation(touchorientation) }
    }

    /// [`ICoreWebView2PointerInfo::TouchPressure`]
    #[inline]
    pub fn touch_pressure(&self) -> windows_core::Result<u32> {
        let mut touchpressure = Default::default();
        unsafe { self.as_ref().TouchPressure(&mut touchpressure) }?;
        Ok(touchpressure)
    }

    /// [`ICoreWebView2PointerInfo::SetTouchPressure`]
    #[inline]
    pub fn set_touch_pressure(&self, touchpressure: u32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetTouchPressure(touchpressure) }
    }
}

/// [`ICoreWebView2PrintSettings`]
pub type WebView2PrintSettings = SafeWrapper<ICoreWebView2PrintSettings>;

impl WebView2PrintSettings {
    /// [`ICoreWebView2PrintSettings::Orientation`]
    #[inline]
    pub fn orientation(&self) -> windows_core::Result<COREWEBVIEW2_PRINT_ORIENTATION> {
        let mut orientation = Default::default();
        unsafe { self.as_ref().Orientation(&mut orientation) }?;
        Ok(orientation)
    }

    /// [`ICoreWebView2PrintSettings::SetOrientation`]
    #[inline]
    pub fn set_orientation(
        &self,
        orientation: COREWEBVIEW2_PRINT_ORIENTATION,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetOrientation(orientation) }
    }

    /// [`ICoreWebView2PrintSettings::ScaleFactor`]
    #[inline]
    pub fn scale_factor(&self) -> windows_core::Result<f64> {
        let mut scalefactor = Default::default();
        unsafe { self.as_ref().ScaleFactor(&mut scalefactor) }?;
        Ok(scalefactor)
    }

    /// [`ICoreWebView2PrintSettings::SetScaleFactor`]
    #[inline]
    pub fn set_scale_factor(&self, scalefactor: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetScaleFactor(scalefactor) }
    }

    /// [`ICoreWebView2PrintSettings::PageWidth`]
    #[inline]
    pub fn page_width(&self) -> windows_core::Result<f64> {
        let mut pagewidth = Default::default();
        unsafe { self.as_ref().PageWidth(&mut pagewidth) }?;
        Ok(pagewidth)
    }

    /// [`ICoreWebView2PrintSettings::SetPageWidth`]
    #[inline]
    pub fn set_page_width(&self, pagewidth: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPageWidth(pagewidth) }
    }

    /// [`ICoreWebView2PrintSettings::PageHeight`]
    #[inline]
    pub fn page_height(&self) -> windows_core::Result<f64> {
        let mut pageheight = Default::default();
        unsafe { self.as_ref().PageHeight(&mut pageheight) }?;
        Ok(pageheight)
    }

    /// [`ICoreWebView2PrintSettings::SetPageHeight`]
    #[inline]
    pub fn set_page_height(&self, pageheight: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPageHeight(pageheight) }
    }

    /// [`ICoreWebView2PrintSettings::MarginTop`]
    #[inline]
    pub fn margin_top(&self) -> windows_core::Result<f64> {
        let mut margintop = Default::default();
        unsafe { self.as_ref().MarginTop(&mut margintop) }?;
        Ok(margintop)
    }

    /// [`ICoreWebView2PrintSettings::SetMarginTop`]
    #[inline]
    pub fn set_margin_top(&self, margintop: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMarginTop(margintop) }
    }

    /// [`ICoreWebView2PrintSettings::MarginBottom`]
    #[inline]
    pub fn margin_bottom(&self) -> windows_core::Result<f64> {
        let mut marginbottom = Default::default();
        unsafe { self.as_ref().MarginBottom(&mut marginbottom) }?;
        Ok(marginbottom)
    }

    /// [`ICoreWebView2PrintSettings::SetMarginBottom`]
    #[inline]
    pub fn set_margin_bottom(&self, marginbottom: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMarginBottom(marginbottom) }
    }

    /// [`ICoreWebView2PrintSettings::MarginLeft`]
    #[inline]
    pub fn margin_left(&self) -> windows_core::Result<f64> {
        let mut marginleft = Default::default();
        unsafe { self.as_ref().MarginLeft(&mut marginleft) }?;
        Ok(marginleft)
    }

    /// [`ICoreWebView2PrintSettings::SetMarginLeft`]
    #[inline]
    pub fn set_margin_left(&self, marginleft: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMarginLeft(marginleft) }
    }

    /// [`ICoreWebView2PrintSettings::MarginRight`]
    #[inline]
    pub fn margin_right(&self) -> windows_core::Result<f64> {
        let mut marginright = Default::default();
        unsafe { self.as_ref().MarginRight(&mut marginright) }?;
        Ok(marginright)
    }

    /// [`ICoreWebView2PrintSettings::SetMarginRight`]
    #[inline]
    pub fn set_margin_right(&self, marginright: f64) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMarginRight(marginright) }
    }

    /// [`ICoreWebView2PrintSettings::ShouldPrintBackgrounds`]
    #[inline]
    pub fn should_print_backgrounds(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut shouldprintbackgrounds = Default::default();
        unsafe {
            self.as_ref()
                .ShouldPrintBackgrounds(&mut shouldprintbackgrounds)
        }?;
        Ok(shouldprintbackgrounds)
    }

    /// [`ICoreWebView2PrintSettings::SetShouldPrintBackgrounds`]
    #[inline]
    pub fn set_should_print_backgrounds<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        shouldprintbackgrounds: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetShouldPrintBackgrounds(shouldprintbackgrounds)
        }
    }

    /// [`ICoreWebView2PrintSettings::ShouldPrintSelectionOnly`]
    #[inline]
    pub fn should_print_selection_only(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut shouldprintselectiononly = Default::default();
        unsafe {
            self.as_ref()
                .ShouldPrintSelectionOnly(&mut shouldprintselectiononly)
        }?;
        Ok(shouldprintselectiononly)
    }

    /// [`ICoreWebView2PrintSettings::SetShouldPrintSelectionOnly`]
    #[inline]
    pub fn set_should_print_selection_only<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        shouldprintselectiononly: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetShouldPrintSelectionOnly(shouldprintselectiononly)
        }
    }

    /// [`ICoreWebView2PrintSettings::ShouldPrintHeaderAndFooter`]
    #[inline]
    pub fn should_print_header_and_footer(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut shouldprintheaderandfooter = Default::default();
        unsafe {
            self.as_ref()
                .ShouldPrintHeaderAndFooter(&mut shouldprintheaderandfooter)
        }?;
        Ok(shouldprintheaderandfooter)
    }

    /// [`ICoreWebView2PrintSettings::SetShouldPrintHeaderAndFooter`]
    #[inline]
    pub fn set_should_print_header_and_footer<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        shouldprintheaderandfooter: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetShouldPrintHeaderAndFooter(shouldprintheaderandfooter)
        }
    }

    /// [`ICoreWebView2PrintSettings::HeaderTitle`]
    #[inline]
    pub fn header_title(&self) -> windows_core::Result<String> {
        let mut headertitle = windows_core::PWSTR::null();
        unsafe { self.as_ref().HeaderTitle(&mut headertitle) }?;
        Ok(crate::pwstr::take_pwstr(headertitle))
    }

    /// [`ICoreWebView2PrintSettings::SetHeaderTitle`]
    #[inline]
    pub fn set_header_title<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        headertitle: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHeaderTitle(headertitle) }
    }

    /// [`ICoreWebView2PrintSettings::FooterUri`]
    #[inline]
    pub fn footer_uri(&self) -> windows_core::Result<String> {
        let mut footeruri = windows_core::PWSTR::null();
        unsafe { self.as_ref().FooterUri(&mut footeruri) }?;
        Ok(crate::pwstr::take_pwstr(footeruri))
    }

    /// [`ICoreWebView2PrintSettings::SetFooterUri`]
    #[inline]
    pub fn set_footer_uri<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        footeruri: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetFooterUri(footeruri) }
    }
}

/// [`ICoreWebView2PrintSettings2`]
pub type WebView2PrintSettings2 = SafeWrapper<ICoreWebView2PrintSettings2>;

impl WebView2PrintSettings2 {
    /// [`ICoreWebView2PrintSettings2::PageRanges`]
    #[inline]
    pub fn page_ranges(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().PageRanges(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2PrintSettings2::SetPageRanges`]
    #[inline]
    pub fn set_page_ranges<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPageRanges(value) }
    }

    /// [`ICoreWebView2PrintSettings2::PagesPerSide`]
    #[inline]
    pub fn pages_per_side(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().PagesPerSide(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetPagesPerSide`]
    #[inline]
    pub fn set_pages_per_side(&self, value: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPagesPerSide(value) }
    }

    /// [`ICoreWebView2PrintSettings2::Copies`]
    #[inline]
    pub fn copies(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Copies(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetCopies`]
    #[inline]
    pub fn set_copies(&self, value: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCopies(value) }
    }

    /// [`ICoreWebView2PrintSettings2::Collation`]
    #[inline]
    pub fn collation(&self) -> windows_core::Result<COREWEBVIEW2_PRINT_COLLATION> {
        let mut value = Default::default();
        unsafe { self.as_ref().Collation(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetCollation`]
    #[inline]
    pub fn set_collation(&self, value: COREWEBVIEW2_PRINT_COLLATION) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCollation(value) }
    }

    /// [`ICoreWebView2PrintSettings2::ColorMode`]
    #[inline]
    pub fn color_mode(&self) -> windows_core::Result<COREWEBVIEW2_PRINT_COLOR_MODE> {
        let mut value = Default::default();
        unsafe { self.as_ref().ColorMode(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetColorMode`]
    #[inline]
    pub fn set_color_mode(&self, value: COREWEBVIEW2_PRINT_COLOR_MODE) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetColorMode(value) }
    }

    /// [`ICoreWebView2PrintSettings2::Duplex`]
    #[inline]
    pub fn duplex(&self) -> windows_core::Result<COREWEBVIEW2_PRINT_DUPLEX> {
        let mut value = Default::default();
        unsafe { self.as_ref().Duplex(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetDuplex`]
    #[inline]
    pub fn set_duplex(&self, value: COREWEBVIEW2_PRINT_DUPLEX) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDuplex(value) }
    }

    /// [`ICoreWebView2PrintSettings2::MediaSize`]
    #[inline]
    pub fn media_size(&self) -> windows_core::Result<COREWEBVIEW2_PRINT_MEDIA_SIZE> {
        let mut value = Default::default();
        unsafe { self.as_ref().MediaSize(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2PrintSettings2::SetMediaSize`]
    #[inline]
    pub fn set_media_size(&self, value: COREWEBVIEW2_PRINT_MEDIA_SIZE) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMediaSize(value) }
    }

    /// [`ICoreWebView2PrintSettings2::PrinterName`]
    #[inline]
    pub fn printer_name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().PrinterName(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2PrintSettings2::SetPrinterName`]
    #[inline]
    pub fn set_printer_name<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPrinterName(value) }
    }
}

/// [`ICoreWebView2ProcessExtendedInfo`]
pub type WebView2ProcessExtendedInfo = SafeWrapper<ICoreWebView2ProcessExtendedInfo>;

impl WebView2ProcessExtendedInfo {
    /// [`ICoreWebView2ProcessExtendedInfo::ProcessInfo`]
    #[inline]
    pub fn process_info(&self) -> windows_core::Result<ICoreWebView2ProcessInfo> {
        let result = unsafe { self.as_ref().ProcessInfo() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ProcessExtendedInfo::AssociatedFrameInfos`]
    #[inline]
    pub fn associated_frame_infos(&self) -> windows_core::Result<ICoreWebView2FrameInfoCollection> {
        let result = unsafe { self.as_ref().AssociatedFrameInfos() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ProcessExtendedInfoCollection`]
pub type WebView2ProcessExtendedInfoCollection =
    SafeWrapper<ICoreWebView2ProcessExtendedInfoCollection>;

impl WebView2ProcessExtendedInfoCollection {
    /// [`ICoreWebView2ProcessExtendedInfoCollection::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ProcessExtendedInfoCollection::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<ICoreWebView2ProcessExtendedInfo> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ProcessFailedEventArgs`]
pub type WebView2ProcessFailedEventArgs = SafeWrapper<ICoreWebView2ProcessFailedEventArgs>;

impl WebView2ProcessFailedEventArgs {
    /// [`ICoreWebView2ProcessFailedEventArgs::ProcessFailedKind`]
    #[inline]
    pub fn process_failed_kind(&self) -> windows_core::Result<COREWEBVIEW2_PROCESS_FAILED_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().ProcessFailedKind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2ProcessFailedEventArgs2`]
pub type WebView2ProcessFailedEventArgs2 = SafeWrapper<ICoreWebView2ProcessFailedEventArgs2>;

impl WebView2ProcessFailedEventArgs2 {
    /// [`ICoreWebView2ProcessFailedEventArgs2::Reason`]
    #[inline]
    pub fn reason(&self) -> windows_core::Result<COREWEBVIEW2_PROCESS_FAILED_REASON> {
        let mut reason = Default::default();
        unsafe { self.as_ref().Reason(&mut reason) }?;
        Ok(reason)
    }

    /// [`ICoreWebView2ProcessFailedEventArgs2::ExitCode`]
    #[inline]
    pub fn exit_code(&self) -> windows_core::Result<i32> {
        let mut exitcode = Default::default();
        unsafe { self.as_ref().ExitCode(&mut exitcode) }?;
        Ok(exitcode)
    }

    /// [`ICoreWebView2ProcessFailedEventArgs2::ProcessDescription`]
    #[inline]
    pub fn process_description(&self) -> windows_core::Result<String> {
        let mut processdescription = windows_core::PWSTR::null();
        unsafe { self.as_ref().ProcessDescription(&mut processdescription) }?;
        Ok(crate::pwstr::take_pwstr(processdescription))
    }

    /// [`ICoreWebView2ProcessFailedEventArgs2::FrameInfosForFailedProcess`]
    #[inline]
    pub fn frame_infos_for_failed_process(
        &self,
    ) -> windows_core::Result<ICoreWebView2FrameInfoCollection> {
        let result = unsafe { self.as_ref().FrameInfosForFailedProcess() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ProcessFailedEventArgs3`]
pub type WebView2ProcessFailedEventArgs3 = SafeWrapper<ICoreWebView2ProcessFailedEventArgs3>;

impl WebView2ProcessFailedEventArgs3 {
    /// [`ICoreWebView2ProcessFailedEventArgs3::FailureSourceModulePath`]
    #[inline]
    pub fn failure_source_module_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FailureSourceModulePath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2ProcessInfo`]
pub type WebView2ProcessInfo = SafeWrapper<ICoreWebView2ProcessInfo>;

impl WebView2ProcessInfo {
    /// [`ICoreWebView2ProcessInfo::ProcessId`]
    #[inline]
    pub fn process_id(&self) -> windows_core::Result<i32> {
        let mut value = Default::default();
        unsafe { self.as_ref().ProcessId(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ProcessInfo::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_PROCESS_KIND> {
        let mut kind = Default::default();
        unsafe { self.as_ref().Kind(&mut kind) }?;
        Ok(kind)
    }
}

/// [`ICoreWebView2ProcessInfoCollection`]
pub type WebView2ProcessInfoCollection = SafeWrapper<ICoreWebView2ProcessInfoCollection>;

impl WebView2ProcessInfoCollection {
    /// [`ICoreWebView2ProcessInfoCollection::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ProcessInfoCollection::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(&self, index: u32) -> windows_core::Result<ICoreWebView2ProcessInfo> {
        let result = unsafe { self.as_ref().GetValueAtIndex(index) }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Profile`]
pub type WebView2Profile = SafeWrapper<ICoreWebView2Profile>;

impl WebView2Profile {
    /// [`ICoreWebView2Profile::ProfileName`]
    #[inline]
    pub fn profile_name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ProfileName(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Profile::IsInPrivateModeEnabled`]
    #[inline]
    pub fn is_in_private_mode_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsInPrivateModeEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Profile::ProfilePath`]
    #[inline]
    pub fn profile_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ProfilePath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Profile::DefaultDownloadFolderPath`]
    #[inline]
    pub fn default_download_folder_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DefaultDownloadFolderPath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Profile::SetDefaultDownloadFolderPath`]
    #[inline]
    pub fn set_default_download_folder_path<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDefaultDownloadFolderPath(value) }
    }

    /// [`ICoreWebView2Profile::PreferredColorScheme`]
    #[inline]
    pub fn preferred_color_scheme(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_PREFERRED_COLOR_SCHEME> {
        let mut value = Default::default();
        unsafe { self.as_ref().PreferredColorScheme(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Profile::SetPreferredColorScheme`]
    #[inline]
    pub fn set_preferred_color_scheme(
        &self,
        value: COREWEBVIEW2_PREFERRED_COLOR_SCHEME,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPreferredColorScheme(value) }
    }
}

/// [`ICoreWebView2Profile2`]
pub type WebView2Profile2 = SafeWrapper<ICoreWebView2Profile2>;

impl WebView2Profile2 {
    /// [`ICoreWebView2Profile2::ClearBrowsingData`]
    #[inline]
    pub fn clear_browsing_data<
        P0: windows_core::Param<ICoreWebView2ClearBrowsingDataCompletedHandler>,
    >(
        &self,
        datakinds: COREWEBVIEW2_BROWSING_DATA_KINDS,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ClearBrowsingData(datakinds, handler) }
    }

    /// [`ICoreWebView2Profile2::ClearBrowsingDataInTimeRange`]
    #[inline]
    pub fn clear_browsing_data_in_time_range<
        P0: windows_core::Param<ICoreWebView2ClearBrowsingDataCompletedHandler>,
    >(
        &self,
        datakinds: COREWEBVIEW2_BROWSING_DATA_KINDS,
        starttime: f64,
        endtime: f64,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .ClearBrowsingDataInTimeRange(datakinds, starttime, endtime, handler)
        }
    }

    /// [`ICoreWebView2Profile2::ClearBrowsingDataAll`]
    #[inline]
    pub fn clear_browsing_data_all<
        P0: windows_core::Param<ICoreWebView2ClearBrowsingDataCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ClearBrowsingDataAll(handler) }
    }
}

/// [`ICoreWebView2Profile3`]
pub type WebView2Profile3 = SafeWrapper<ICoreWebView2Profile3>;

impl WebView2Profile3 {
    /// [`ICoreWebView2Profile3::PreferredTrackingPreventionLevel`]
    #[inline]
    pub fn preferred_tracking_prevention_level(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_TRACKING_PREVENTION_LEVEL> {
        let mut value = Default::default();
        unsafe { self.as_ref().PreferredTrackingPreventionLevel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Profile3::SetPreferredTrackingPreventionLevel`]
    #[inline]
    pub fn set_preferred_tracking_prevention_level(
        &self,
        value: COREWEBVIEW2_TRACKING_PREVENTION_LEVEL,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetPreferredTrackingPreventionLevel(value) }
    }
}

/// [`ICoreWebView2Profile4`]
pub type WebView2Profile4 = SafeWrapper<ICoreWebView2Profile4>;

impl WebView2Profile4 {
    /// [`ICoreWebView2Profile4::SetPermissionState`]
    #[inline]
    pub fn set_permission_state<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2SetPermissionStateCompletedHandler>,
    >(
        &self,
        permissionkind: COREWEBVIEW2_PERMISSION_KIND,
        origin: P0,
        state: COREWEBVIEW2_PERMISSION_STATE,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetPermissionState(permissionkind, origin, state, handler)
        }
    }

    /// [`ICoreWebView2Profile4::GetNonDefaultPermissionSettings`]
    #[inline]
    pub fn get_non_default_permission_settings<
        P0: windows_core::Param<ICoreWebView2GetNonDefaultPermissionSettingsCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetNonDefaultPermissionSettings(handler) }
    }
}

/// [`ICoreWebView2Profile5`]
pub type WebView2Profile5 = SafeWrapper<ICoreWebView2Profile5>;

impl WebView2Profile5 {
    /// [`ICoreWebView2Profile5::CookieManager`]
    #[inline]
    pub fn cookie_manager(&self) -> windows_core::Result<ICoreWebView2CookieManager> {
        let result = unsafe { self.as_ref().CookieManager() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Profile6`]
pub type WebView2Profile6 = SafeWrapper<ICoreWebView2Profile6>;

impl WebView2Profile6 {
    /// [`ICoreWebView2Profile6::IsPasswordAutosaveEnabled`]
    #[inline]
    pub fn is_password_autosave_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsPasswordAutosaveEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Profile6::SetIsPasswordAutosaveEnabled`]
    #[inline]
    pub fn set_is_password_autosave_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsPasswordAutosaveEnabled(value) }
    }

    /// [`ICoreWebView2Profile6::IsGeneralAutofillEnabled`]
    #[inline]
    pub fn is_general_autofill_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsGeneralAutofillEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Profile6::SetIsGeneralAutofillEnabled`]
    #[inline]
    pub fn set_is_general_autofill_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsGeneralAutofillEnabled(value) }
    }
}

/// [`ICoreWebView2Profile7`]
pub type WebView2Profile7 = SafeWrapper<ICoreWebView2Profile7>;

impl WebView2Profile7 {
    /// [`ICoreWebView2Profile7::AddBrowserExtension`]
    #[inline]
    pub fn add_browser_extension<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ProfileAddBrowserExtensionCompletedHandler>,
    >(
        &self,
        extensionfolderpath: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .AddBrowserExtension(extensionfolderpath, handler)
        }
    }

    /// [`ICoreWebView2Profile7::GetBrowserExtensions`]
    #[inline]
    pub fn get_browser_extensions<
        P0: windows_core::Param<ICoreWebView2ProfileGetBrowserExtensionsCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetBrowserExtensions(handler) }
    }
}

/// [`ICoreWebView2Profile8`]
pub type WebView2Profile8 = SafeWrapper<ICoreWebView2Profile8>;

impl WebView2Profile8 {
    /// [`ICoreWebView2Profile8::Delete`]
    #[inline]
    pub fn delete(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Delete() }
    }

    /// [`ICoreWebView2Profile8::add_Deleted`]
    #[inline]
    pub fn add_deleted<P0: windows_core::Param<ICoreWebView2ProfileDeletedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_Deleted(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2Profile8::remove_Deleted`]
    #[inline]
    pub fn remove_deleted(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_Deleted(token) }
    }
}

/// [`ICoreWebView2RegionRectCollectionView`]
pub type WebView2RegionRectCollectionView = SafeWrapper<ICoreWebView2RegionRectCollectionView>;

impl WebView2RegionRectCollectionView {
    /// [`ICoreWebView2RegionRectCollectionView::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2RegionRectCollectionView::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(
        &self,
        index: u32,
    ) -> windows_core::Result<windows::Win32::Foundation::RECT> {
        let mut value = Default::default();
        unsafe { self.as_ref().GetValueAtIndex(index, &mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2SaveAsUIShowingEventArgs`]
pub type WebView2SaveAsUIShowingEventArgs = SafeWrapper<ICoreWebView2SaveAsUIShowingEventArgs>;

impl WebView2SaveAsUIShowingEventArgs {
    /// [`ICoreWebView2SaveAsUIShowingEventArgs::ContentMimeType`]
    #[inline]
    pub fn content_mime_type(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ContentMimeType(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(value) }
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Cancel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SetSuppressDefaultDialog`]
    #[inline]
    pub fn set_suppress_default_dialog<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSuppressDefaultDialog(value) }
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SuppressDefaultDialog`]
    #[inline]
    pub fn suppress_default_dialog(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().SuppressDefaultDialog(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SetSaveAsFilePath`]
    #[inline]
    pub fn set_save_as_file_path<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSaveAsFilePath(value) }
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SaveAsFilePath`]
    #[inline]
    pub fn save_as_file_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().SaveAsFilePath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SetAllowReplace`]
    #[inline]
    pub fn set_allow_replace<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAllowReplace(value) }
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::AllowReplace`]
    #[inline]
    pub fn allow_replace(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().AllowReplace(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::SetKind`]
    #[inline]
    pub fn set_kind(&self, value: COREWEBVIEW2_SAVE_AS_KIND) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetKind(value) }
    }

    /// [`ICoreWebView2SaveAsUIShowingEventArgs::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_SAVE_AS_KIND> {
        let mut value = Default::default();
        unsafe { self.as_ref().Kind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs`]
pub type WebView2SaveFileSecurityCheckStartingEventArgs =
    SafeWrapper<ICoreWebView2SaveFileSecurityCheckStartingEventArgs>;

impl WebView2SaveFileSecurityCheckStartingEventArgs {
    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::CancelSave`]
    #[inline]
    pub fn cancel_save(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().CancelSave(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::SetCancelSave`]
    #[inline]
    pub fn set_cancel_save<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancelSave(value) }
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::DocumentOriginUri`]
    #[inline]
    pub fn document_origin_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().DocumentOriginUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::FileExtension`]
    #[inline]
    pub fn file_extension(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FileExtension(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::FilePath`]
    #[inline]
    pub fn file_path(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FilePath(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::SuppressDefaultPolicy`]
    #[inline]
    pub fn suppress_default_policy(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().SuppressDefaultPolicy(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::SetSuppressDefaultPolicy`]
    #[inline]
    pub fn set_suppress_default_policy<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetSuppressDefaultPolicy(value) }
    }

    /// [`ICoreWebView2SaveFileSecurityCheckStartingEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ScreenCaptureStartingEventArgs`]
pub type WebView2ScreenCaptureStartingEventArgs =
    SafeWrapper<ICoreWebView2ScreenCaptureStartingEventArgs>;

impl WebView2ScreenCaptureStartingEventArgs {
    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::Cancel`]
    #[inline]
    pub fn cancel(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Cancel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::SetCancel`]
    #[inline]
    pub fn set_cancel<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetCancel(value) }
    }

    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::Handled`]
    #[inline]
    pub fn handled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().Handled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::SetHandled`]
    #[inline]
    pub fn set_handled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHandled(value) }
    }

    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::OriginalSourceFrameInfo`]
    #[inline]
    pub fn original_source_frame_info(&self) -> windows_core::Result<ICoreWebView2FrameInfo> {
        let result = unsafe { self.as_ref().OriginalSourceFrameInfo() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ScreenCaptureStartingEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ScriptDialogOpeningEventArgs`]
pub type WebView2ScriptDialogOpeningEventArgs =
    SafeWrapper<ICoreWebView2ScriptDialogOpeningEventArgs>;

impl WebView2ScriptDialogOpeningEventArgs {
    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::Kind`]
    #[inline]
    pub fn kind(&self) -> windows_core::Result<COREWEBVIEW2_SCRIPT_DIALOG_KIND> {
        let mut kind = Default::default();
        unsafe { self.as_ref().Kind(&mut kind) }?;
        Ok(kind)
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::Message`]
    #[inline]
    pub fn message(&self) -> windows_core::Result<String> {
        let mut message = windows_core::PWSTR::null();
        unsafe { self.as_ref().Message(&mut message) }?;
        Ok(crate::pwstr::take_pwstr(message))
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::Accept`]
    #[inline]
    pub fn accept(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Accept() }
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::DefaultText`]
    #[inline]
    pub fn default_text(&self) -> windows_core::Result<String> {
        let mut defaulttext = windows_core::PWSTR::null();
        unsafe { self.as_ref().DefaultText(&mut defaulttext) }?;
        Ok(crate::pwstr::take_pwstr(defaulttext))
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::ResultText`]
    #[inline]
    pub fn result_text(&self) -> windows_core::Result<String> {
        let mut resulttext = windows_core::PWSTR::null();
        unsafe { self.as_ref().ResultText(&mut resulttext) }?;
        Ok(crate::pwstr::take_pwstr(resulttext))
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::SetResultText`]
    #[inline]
    pub fn set_result_text<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        resulttext: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetResultText(resulttext) }
    }

    /// [`ICoreWebView2ScriptDialogOpeningEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2ScriptException`]
pub type WebView2ScriptException = SafeWrapper<ICoreWebView2ScriptException>;

impl WebView2ScriptException {
    /// [`ICoreWebView2ScriptException::LineNumber`]
    #[inline]
    pub fn line_number(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().LineNumber(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ScriptException::ColumnNumber`]
    #[inline]
    pub fn column_number(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().ColumnNumber(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ScriptException::Name`]
    #[inline]
    pub fn name(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Name(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ScriptException::Message`]
    #[inline]
    pub fn message(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Message(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ScriptException::ToJson`]
    #[inline]
    pub fn to_json(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().ToJson(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs`]
pub type WebView2ServerCertificateErrorDetectedEventArgs =
    SafeWrapper<ICoreWebView2ServerCertificateErrorDetectedEventArgs>;

impl WebView2ServerCertificateErrorDetectedEventArgs {
    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::ErrorStatus`]
    #[inline]
    pub fn error_status(&self) -> windows_core::Result<COREWEBVIEW2_WEB_ERROR_STATUS> {
        let mut value = Default::default();
        unsafe { self.as_ref().ErrorStatus(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::RequestUri`]
    #[inline]
    pub fn request_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().RequestUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::ServerCertificate`]
    #[inline]
    pub fn server_certificate(&self) -> windows_core::Result<ICoreWebView2Certificate> {
        let result = unsafe { self.as_ref().ServerCertificate() }?;
        Ok(result)
    }

    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::Action`]
    #[inline]
    pub fn action(&self) -> windows_core::Result<COREWEBVIEW2_SERVER_CERTIFICATE_ERROR_ACTION> {
        let mut value = Default::default();
        unsafe { self.as_ref().Action(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::SetAction`]
    #[inline]
    pub fn set_action(
        &self,
        value: COREWEBVIEW2_SERVER_CERTIFICATE_ERROR_ACTION,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAction(value) }
    }

    /// [`ICoreWebView2ServerCertificateErrorDetectedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2Settings`]
pub type WebView2Settings = SafeWrapper<ICoreWebView2Settings>;

impl WebView2Settings {
    /// [`ICoreWebView2Settings::IsScriptEnabled`]
    #[inline]
    pub fn is_script_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isscriptenabled = Default::default();
        unsafe { self.as_ref().IsScriptEnabled(&mut isscriptenabled) }?;
        Ok(isscriptenabled)
    }

    /// [`ICoreWebView2Settings::SetIsScriptEnabled`]
    #[inline]
    pub fn set_is_script_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        isscriptenabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsScriptEnabled(isscriptenabled) }
    }

    /// [`ICoreWebView2Settings::IsWebMessageEnabled`]
    #[inline]
    pub fn is_web_message_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut iswebmessageenabled = Default::default();
        unsafe { self.as_ref().IsWebMessageEnabled(&mut iswebmessageenabled) }?;
        Ok(iswebmessageenabled)
    }

    /// [`ICoreWebView2Settings::SetIsWebMessageEnabled`]
    #[inline]
    pub fn set_is_web_message_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        iswebmessageenabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsWebMessageEnabled(iswebmessageenabled) }
    }

    /// [`ICoreWebView2Settings::AreDefaultScriptDialogsEnabled`]
    #[inline]
    pub fn are_default_script_dialogs_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut aredefaultscriptdialogsenabled = Default::default();
        unsafe {
            self.as_ref()
                .AreDefaultScriptDialogsEnabled(&mut aredefaultscriptdialogsenabled)
        }?;
        Ok(aredefaultscriptdialogsenabled)
    }

    /// [`ICoreWebView2Settings::SetAreDefaultScriptDialogsEnabled`]
    #[inline]
    pub fn set_are_default_script_dialogs_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        aredefaultscriptdialogsenabled: P0,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetAreDefaultScriptDialogsEnabled(aredefaultscriptdialogsenabled)
        }
    }

    /// [`ICoreWebView2Settings::IsStatusBarEnabled`]
    #[inline]
    pub fn is_status_bar_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut isstatusbarenabled = Default::default();
        unsafe { self.as_ref().IsStatusBarEnabled(&mut isstatusbarenabled) }?;
        Ok(isstatusbarenabled)
    }

    /// [`ICoreWebView2Settings::SetIsStatusBarEnabled`]
    #[inline]
    pub fn set_is_status_bar_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        isstatusbarenabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsStatusBarEnabled(isstatusbarenabled) }
    }

    /// [`ICoreWebView2Settings::AreDevToolsEnabled`]
    #[inline]
    pub fn are_dev_tools_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut aredevtoolsenabled = Default::default();
        unsafe { self.as_ref().AreDevToolsEnabled(&mut aredevtoolsenabled) }?;
        Ok(aredevtoolsenabled)
    }

    /// [`ICoreWebView2Settings::SetAreDevToolsEnabled`]
    #[inline]
    pub fn set_are_dev_tools_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        aredevtoolsenabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAreDevToolsEnabled(aredevtoolsenabled) }
    }

    /// [`ICoreWebView2Settings::AreDefaultContextMenusEnabled`]
    #[inline]
    pub fn are_default_context_menus_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut enabled = Default::default();
        unsafe { self.as_ref().AreDefaultContextMenusEnabled(&mut enabled) }?;
        Ok(enabled)
    }

    /// [`ICoreWebView2Settings::SetAreDefaultContextMenusEnabled`]
    #[inline]
    pub fn set_are_default_context_menus_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        enabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAreDefaultContextMenusEnabled(enabled) }
    }

    /// [`ICoreWebView2Settings::AreHostObjectsAllowed`]
    #[inline]
    pub fn are_host_objects_allowed(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut allowed = Default::default();
        unsafe { self.as_ref().AreHostObjectsAllowed(&mut allowed) }?;
        Ok(allowed)
    }

    /// [`ICoreWebView2Settings::SetAreHostObjectsAllowed`]
    #[inline]
    pub fn set_are_host_objects_allowed<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        allowed: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAreHostObjectsAllowed(allowed) }
    }

    /// [`ICoreWebView2Settings::IsZoomControlEnabled`]
    #[inline]
    pub fn is_zoom_control_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut enabled = Default::default();
        unsafe { self.as_ref().IsZoomControlEnabled(&mut enabled) }?;
        Ok(enabled)
    }

    /// [`ICoreWebView2Settings::SetIsZoomControlEnabled`]
    #[inline]
    pub fn set_is_zoom_control_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        enabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsZoomControlEnabled(enabled) }
    }

    /// [`ICoreWebView2Settings::IsBuiltInErrorPageEnabled`]
    #[inline]
    pub fn is_built_in_error_page_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut enabled = Default::default();
        unsafe { self.as_ref().IsBuiltInErrorPageEnabled(&mut enabled) }?;
        Ok(enabled)
    }

    /// [`ICoreWebView2Settings::SetIsBuiltInErrorPageEnabled`]
    #[inline]
    pub fn set_is_built_in_error_page_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        enabled: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsBuiltInErrorPageEnabled(enabled) }
    }
}

/// [`ICoreWebView2Settings2`]
pub type WebView2Settings2 = SafeWrapper<ICoreWebView2Settings2>;

impl WebView2Settings2 {
    /// [`ICoreWebView2Settings2::UserAgent`]
    #[inline]
    pub fn user_agent(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().UserAgent(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2Settings2::SetUserAgent`]
    #[inline]
    pub fn set_user_agent<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetUserAgent(value) }
    }
}

/// [`ICoreWebView2Settings3`]
pub type WebView2Settings3 = SafeWrapper<ICoreWebView2Settings3>;

impl WebView2Settings3 {
    /// [`ICoreWebView2Settings3::AreBrowserAcceleratorKeysEnabled`]
    #[inline]
    pub fn are_browser_accelerator_keys_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().AreBrowserAcceleratorKeysEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings3::SetAreBrowserAcceleratorKeysEnabled`]
    #[inline]
    pub fn set_are_browser_accelerator_keys_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetAreBrowserAcceleratorKeysEnabled(value) }
    }
}

/// [`ICoreWebView2Settings4`]
pub type WebView2Settings4 = SafeWrapper<ICoreWebView2Settings4>;

impl WebView2Settings4 {
    /// [`ICoreWebView2Settings4::IsPasswordAutosaveEnabled`]
    #[inline]
    pub fn is_password_autosave_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsPasswordAutosaveEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings4::SetIsPasswordAutosaveEnabled`]
    #[inline]
    pub fn set_is_password_autosave_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsPasswordAutosaveEnabled(value) }
    }

    /// [`ICoreWebView2Settings4::IsGeneralAutofillEnabled`]
    #[inline]
    pub fn is_general_autofill_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsGeneralAutofillEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings4::SetIsGeneralAutofillEnabled`]
    #[inline]
    pub fn set_is_general_autofill_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsGeneralAutofillEnabled(value) }
    }
}

/// [`ICoreWebView2Settings5`]
pub type WebView2Settings5 = SafeWrapper<ICoreWebView2Settings5>;

impl WebView2Settings5 {
    /// [`ICoreWebView2Settings5::IsPinchZoomEnabled`]
    #[inline]
    pub fn is_pinch_zoom_enabled(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsPinchZoomEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings5::SetIsPinchZoomEnabled`]
    #[inline]
    pub fn set_is_pinch_zoom_enabled<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsPinchZoomEnabled(value) }
    }
}

/// [`ICoreWebView2Settings6`]
pub type WebView2Settings6 = SafeWrapper<ICoreWebView2Settings6>;

impl WebView2Settings6 {
    /// [`ICoreWebView2Settings6::IsSwipeNavigationEnabled`]
    #[inline]
    pub fn is_swipe_navigation_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsSwipeNavigationEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings6::SetIsSwipeNavigationEnabled`]
    #[inline]
    pub fn set_is_swipe_navigation_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsSwipeNavigationEnabled(value) }
    }
}

/// [`ICoreWebView2Settings7`]
pub type WebView2Settings7 = SafeWrapper<ICoreWebView2Settings7>;

impl WebView2Settings7 {
    /// [`ICoreWebView2Settings7::HiddenPdfToolbarItems`]
    #[inline]
    pub fn hidden_pdf_toolbar_items(&self) -> windows_core::Result<COREWEBVIEW2_PDF_TOOLBAR_ITEMS> {
        let mut value = Default::default();
        unsafe { self.as_ref().HiddenPdfToolbarItems(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings7::SetHiddenPdfToolbarItems`]
    #[inline]
    pub fn set_hidden_pdf_toolbar_items(
        &self,
        value: COREWEBVIEW2_PDF_TOOLBAR_ITEMS,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetHiddenPdfToolbarItems(value) }
    }
}

/// [`ICoreWebView2Settings8`]
pub type WebView2Settings8 = SafeWrapper<ICoreWebView2Settings8>;

impl WebView2Settings8 {
    /// [`ICoreWebView2Settings8::IsReputationCheckingRequired`]
    #[inline]
    pub fn is_reputation_checking_required(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsReputationCheckingRequired(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings8::SetIsReputationCheckingRequired`]
    #[inline]
    pub fn set_is_reputation_checking_required<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsReputationCheckingRequired(value) }
    }
}

/// [`ICoreWebView2Settings9`]
pub type WebView2Settings9 = SafeWrapper<ICoreWebView2Settings9>;

impl WebView2Settings9 {
    /// [`ICoreWebView2Settings9::IsNonClientRegionSupportEnabled`]
    #[inline]
    pub fn is_non_client_region_support_enabled(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsNonClientRegionSupportEnabled(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2Settings9::SetIsNonClientRegionSupportEnabled`]
    #[inline]
    pub fn set_is_non_client_region_support_enabled<
        P0: windows_core::Param<windows::Win32::Foundation::BOOL>,
    >(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsNonClientRegionSupportEnabled(value) }
    }
}

/// [`ICoreWebView2SharedBuffer`]
pub type WebView2SharedBuffer = SafeWrapper<ICoreWebView2SharedBuffer>;

impl WebView2SharedBuffer {
    /// [`ICoreWebView2SharedBuffer::Size`]
    #[inline]
    pub fn size(&self) -> windows_core::Result<u64> {
        let mut value = Default::default();
        unsafe { self.as_ref().Size(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SharedBuffer::Buffer`]
    #[inline]
    pub fn buffer(&self) -> windows_core::Result<*mut u8> {
        let mut value = std::ptr::null_mut();
        unsafe { self.as_ref().Buffer(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SharedBuffer::FileMappingHandle`]
    #[inline]
    pub fn file_mapping_handle(&self) -> windows_core::Result<windows::Win32::Foundation::HANDLE> {
        let mut value = Default::default();
        unsafe { self.as_ref().FileMappingHandle(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2SharedBuffer::Close`]
    #[inline]
    pub fn close(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Close() }
    }
}

/// [`ICoreWebView2SourceChangedEventArgs`]
pub type WebView2SourceChangedEventArgs = SafeWrapper<ICoreWebView2SourceChangedEventArgs>;

impl WebView2SourceChangedEventArgs {
    /// [`ICoreWebView2SourceChangedEventArgs::IsNewDocument`]
    #[inline]
    pub fn is_new_document(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsNewDocument(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2StringCollection`]
pub type WebView2StringCollection = SafeWrapper<ICoreWebView2StringCollection>;

impl WebView2StringCollection {
    /// [`ICoreWebView2StringCollection::Count`]
    #[inline]
    pub fn count(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Count(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2StringCollection::GetValueAtIndex`]
    #[inline]
    pub fn get_value_at_index(&self, index: u32) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().GetValueAtIndex(index, &mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2WebMessageReceivedEventArgs`]
pub type WebView2WebMessageReceivedEventArgs =
    SafeWrapper<ICoreWebView2WebMessageReceivedEventArgs>;

impl WebView2WebMessageReceivedEventArgs {
    /// [`ICoreWebView2WebMessageReceivedEventArgs::Source`]
    #[inline]
    pub fn source(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().Source(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2WebMessageReceivedEventArgs::WebMessageAsJson`]
    #[inline]
    pub fn web_message_as_json(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().WebMessageAsJson(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2WebMessageReceivedEventArgs::TryGetWebMessageAsString`]
    #[inline]
    pub fn try_get_web_message_as_string(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().TryGetWebMessageAsString(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2WebMessageReceivedEventArgs2`]
pub type WebView2WebMessageReceivedEventArgs2 =
    SafeWrapper<ICoreWebView2WebMessageReceivedEventArgs2>;

impl WebView2WebMessageReceivedEventArgs2 {
    /// [`ICoreWebView2WebMessageReceivedEventArgs2::AdditionalObjects`]
    #[inline]
    pub fn additional_objects(&self) -> windows_core::Result<ICoreWebView2ObjectCollectionView> {
        let result = unsafe { self.as_ref().AdditionalObjects() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2WebResourceRequest`]
pub type WebView2WebResourceRequest = SafeWrapper<ICoreWebView2WebResourceRequest>;

impl WebView2WebResourceRequest {
    /// [`ICoreWebView2WebResourceRequest::Uri`]
    #[inline]
    pub fn uri(&self) -> windows_core::Result<String> {
        let mut uri = windows_core::PWSTR::null();
        unsafe { self.as_ref().Uri(&mut uri) }?;
        Ok(crate::pwstr::take_pwstr(uri))
    }

    /// [`ICoreWebView2WebResourceRequest::SetUri`]
    #[inline]
    pub fn set_uri<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        uri: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetUri(uri) }
    }

    /// [`ICoreWebView2WebResourceRequest::Method`]
    #[inline]
    pub fn method(&self) -> windows_core::Result<String> {
        let mut method = windows_core::PWSTR::null();
        unsafe { self.as_ref().Method(&mut method) }?;
        Ok(crate::pwstr::take_pwstr(method))
    }

    /// [`ICoreWebView2WebResourceRequest::SetMethod`]
    #[inline]
    pub fn set_method<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        method: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMethod(method) }
    }

    /// [`ICoreWebView2WebResourceRequest::SetContent`]
    #[inline]
    pub fn set_content<P0: windows_core::Param<windows::Win32::System::Com::IStream>>(
        &self,
        content: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetContent(content) }
    }

    /// [`ICoreWebView2WebResourceRequest::Headers`]
    #[inline]
    pub fn headers(&self) -> windows_core::Result<ICoreWebView2HttpRequestHeaders> {
        let result = unsafe { self.as_ref().Headers() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2WebResourceRequestedEventArgs`]
pub type WebView2WebResourceRequestedEventArgs =
    SafeWrapper<ICoreWebView2WebResourceRequestedEventArgs>;

impl WebView2WebResourceRequestedEventArgs {
    /// [`ICoreWebView2WebResourceRequestedEventArgs::Request`]
    #[inline]
    pub fn request(&self) -> windows_core::Result<ICoreWebView2WebResourceRequest> {
        let result = unsafe { self.as_ref().Request() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceRequestedEventArgs::Response`]
    #[inline]
    pub fn response(&self) -> windows_core::Result<ICoreWebView2WebResourceResponse> {
        let result = unsafe { self.as_ref().Response() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceRequestedEventArgs::SetResponse`]
    #[inline]
    pub fn set_response<P0: windows_core::Param<ICoreWebView2WebResourceResponse>>(
        &self,
        response: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetResponse(response) }
    }

    /// [`ICoreWebView2WebResourceRequestedEventArgs::GetDeferral`]
    #[inline]
    pub fn get_deferral(&self) -> windows_core::Result<ICoreWebView2Deferral> {
        let result = unsafe { self.as_ref().GetDeferral() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceRequestedEventArgs::ResourceContext`]
    #[inline]
    pub fn resource_context(&self) -> windows_core::Result<COREWEBVIEW2_WEB_RESOURCE_CONTEXT> {
        let mut context = Default::default();
        unsafe { self.as_ref().ResourceContext(&mut context) }?;
        Ok(context)
    }
}

/// [`ICoreWebView2WebResourceRequestedEventArgs2`]
pub type WebView2WebResourceRequestedEventArgs2 =
    SafeWrapper<ICoreWebView2WebResourceRequestedEventArgs2>;

impl WebView2WebResourceRequestedEventArgs2 {
    /// [`ICoreWebView2WebResourceRequestedEventArgs2::RequestedSourceKind`]
    #[inline]
    pub fn requested_source_kind(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS> {
        let mut value = Default::default();
        unsafe { self.as_ref().RequestedSourceKind(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2WebResourceResponse`]
pub type WebView2WebResourceResponse = SafeWrapper<ICoreWebView2WebResourceResponse>;

impl WebView2WebResourceResponse {
    /// [`ICoreWebView2WebResourceResponse::SetContent`]
    #[inline]
    pub fn set_content<P0: windows_core::Param<windows::Win32::System::Com::IStream>>(
        &self,
        content: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetContent(content) }
    }

    /// [`ICoreWebView2WebResourceResponse::Headers`]
    #[inline]
    pub fn headers(&self) -> windows_core::Result<ICoreWebView2HttpResponseHeaders> {
        let result = unsafe { self.as_ref().Headers() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceResponse::StatusCode`]
    #[inline]
    pub fn status_code(&self) -> windows_core::Result<i32> {
        let mut statuscode = Default::default();
        unsafe { self.as_ref().StatusCode(&mut statuscode) }?;
        Ok(statuscode)
    }

    /// [`ICoreWebView2WebResourceResponse::SetStatusCode`]
    #[inline]
    pub fn set_status_code(&self, statuscode: i32) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetStatusCode(statuscode) }
    }

    /// [`ICoreWebView2WebResourceResponse::ReasonPhrase`]
    #[inline]
    pub fn reason_phrase(&self) -> windows_core::Result<String> {
        let mut reasonphrase = windows_core::PWSTR::null();
        unsafe { self.as_ref().ReasonPhrase(&mut reasonphrase) }?;
        Ok(crate::pwstr::take_pwstr(reasonphrase))
    }

    /// [`ICoreWebView2WebResourceResponse::SetReasonPhrase`]
    #[inline]
    pub fn set_reason_phrase<P0: windows_core::Param<windows_core::PCWSTR>>(
        &self,
        reasonphrase: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetReasonPhrase(reasonphrase) }
    }
}

/// [`ICoreWebView2WebResourceResponseReceivedEventArgs`]
pub type WebView2WebResourceResponseReceivedEventArgs =
    SafeWrapper<ICoreWebView2WebResourceResponseReceivedEventArgs>;

impl WebView2WebResourceResponseReceivedEventArgs {
    /// [`ICoreWebView2WebResourceResponseReceivedEventArgs::Request`]
    #[inline]
    pub fn request(&self) -> windows_core::Result<ICoreWebView2WebResourceRequest> {
        let result = unsafe { self.as_ref().Request() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceResponseReceivedEventArgs::Response`]
    #[inline]
    pub fn response(&self) -> windows_core::Result<ICoreWebView2WebResourceResponseView> {
        let result = unsafe { self.as_ref().Response() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2WebResourceResponseView`]
pub type WebView2WebResourceResponseView = SafeWrapper<ICoreWebView2WebResourceResponseView>;

impl WebView2WebResourceResponseView {
    /// [`ICoreWebView2WebResourceResponseView::Headers`]
    #[inline]
    pub fn headers(&self) -> windows_core::Result<ICoreWebView2HttpResponseHeaders> {
        let result = unsafe { self.as_ref().Headers() }?;
        Ok(result)
    }

    /// [`ICoreWebView2WebResourceResponseView::StatusCode`]
    #[inline]
    pub fn status_code(&self) -> windows_core::Result<i32> {
        let mut statuscode = Default::default();
        unsafe { self.as_ref().StatusCode(&mut statuscode) }?;
        Ok(statuscode)
    }

    /// [`ICoreWebView2WebResourceResponseView::ReasonPhrase`]
    #[inline]
    pub fn reason_phrase(&self) -> windows_core::Result<String> {
        let mut reasonphrase = windows_core::PWSTR::null();
        unsafe { self.as_ref().ReasonPhrase(&mut reasonphrase) }?;
        Ok(crate::pwstr::take_pwstr(reasonphrase))
    }

    /// [`ICoreWebView2WebResourceResponseView::GetContent`]
    #[inline]
    pub fn get_content<
        P0: windows_core::Param<ICoreWebView2WebResourceResponseViewGetContentCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetContent(handler) }
    }
}

/// [`ICoreWebView2WindowFeatures`]
pub type WebView2WindowFeatures = SafeWrapper<ICoreWebView2WindowFeatures>;

impl WebView2WindowFeatures {
    /// [`ICoreWebView2WindowFeatures::HasPosition`]
    #[inline]
    pub fn has_position(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasPosition(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::HasSize`]
    #[inline]
    pub fn has_size(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().HasSize(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::Left`]
    #[inline]
    pub fn left(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Left(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::Top`]
    #[inline]
    pub fn top(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Top(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::Height`]
    #[inline]
    pub fn height(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Height(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::Width`]
    #[inline]
    pub fn width(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().Width(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::ShouldDisplayMenuBar`]
    #[inline]
    pub fn should_display_menu_bar(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldDisplayMenuBar(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::ShouldDisplayStatus`]
    #[inline]
    pub fn should_display_status(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldDisplayStatus(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::ShouldDisplayToolbar`]
    #[inline]
    pub fn should_display_toolbar(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldDisplayToolbar(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2WindowFeatures::ShouldDisplayScrollBars`]
    #[inline]
    pub fn should_display_scroll_bars(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().ShouldDisplayScrollBars(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2_10`]
pub type WebView2_10 = SafeWrapper<ICoreWebView2_10>;

impl WebView2_10 {
    /// [`ICoreWebView2_10::add_BasicAuthenticationRequested`]
    #[inline]
    pub fn add_basic_authentication_requested<
        P0: windows_core::Param<ICoreWebView2BasicAuthenticationRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_BasicAuthenticationRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_10::remove_BasicAuthenticationRequested`]
    #[inline]
    pub fn remove_basic_authentication_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_BasicAuthenticationRequested(token) }
    }
}

/// [`ICoreWebView2_11`]
pub type WebView2_11 = SafeWrapper<ICoreWebView2_11>;

impl WebView2_11 {
    /// [`ICoreWebView2_11::CallDevToolsProtocolMethodForSession`]
    #[inline]
    pub fn call_dev_tools_protocol_method_for_session<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<ICoreWebView2CallDevToolsProtocolMethodCompletedHandler>,
    >(
        &self,
        sessionid: P0,
        methodname: P1,
        parametersasjson: P2,
        handler: P3,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref().CallDevToolsProtocolMethodForSession(
                sessionid,
                methodname,
                parametersasjson,
                handler,
            )
        }
    }

    /// [`ICoreWebView2_11::add_ContextMenuRequested`]
    #[inline]
    pub fn add_context_menu_requested<
        P0: windows_core::Param<ICoreWebView2ContextMenuRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ContextMenuRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_11::remove_ContextMenuRequested`]
    #[inline]
    pub fn remove_context_menu_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ContextMenuRequested(token) }
    }
}

/// [`ICoreWebView2_12`]
pub type WebView2_12 = SafeWrapper<ICoreWebView2_12>;

impl WebView2_12 {
    /// [`ICoreWebView2_12::add_StatusBarTextChanged`]
    #[inline]
    pub fn add_status_bar_text_changed<
        P0: windows_core::Param<ICoreWebView2StatusBarTextChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_StatusBarTextChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_12::remove_StatusBarTextChanged`]
    #[inline]
    pub fn remove_status_bar_text_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_StatusBarTextChanged(token) }
    }

    /// [`ICoreWebView2_12::StatusBarText`]
    #[inline]
    pub fn status_bar_text(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().StatusBarText(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }
}

/// [`ICoreWebView2_13`]
pub type WebView2_13 = SafeWrapper<ICoreWebView2_13>;

impl WebView2_13 {
    /// [`ICoreWebView2_13::Profile`]
    #[inline]
    pub fn profile(&self) -> windows_core::Result<ICoreWebView2Profile> {
        let result = unsafe { self.as_ref().Profile() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2_14`]
pub type WebView2_14 = SafeWrapper<ICoreWebView2_14>;

impl WebView2_14 {
    /// [`ICoreWebView2_14::add_ServerCertificateErrorDetected`]
    #[inline]
    pub fn add_server_certificate_error_detected<
        P0: windows_core::Param<ICoreWebView2ServerCertificateErrorDetectedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ServerCertificateErrorDetected(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_14::remove_ServerCertificateErrorDetected`]
    #[inline]
    pub fn remove_server_certificate_error_detected(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ServerCertificateErrorDetected(token) }
    }

    /// [`ICoreWebView2_14::ClearServerCertificateErrorActions`]
    #[inline]
    pub fn clear_server_certificate_error_actions<
        P0: windows_core::Param<ICoreWebView2ClearServerCertificateErrorActionsCompletedHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ClearServerCertificateErrorActions(handler) }
    }
}

/// [`ICoreWebView2_15`]
pub type WebView2_15 = SafeWrapper<ICoreWebView2_15>;

impl WebView2_15 {
    /// [`ICoreWebView2_15::add_FaviconChanged`]
    #[inline]
    pub fn add_favicon_changed<P0: windows_core::Param<ICoreWebView2FaviconChangedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_FaviconChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_15::remove_FaviconChanged`]
    #[inline]
    pub fn remove_favicon_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_FaviconChanged(token) }
    }

    /// [`ICoreWebView2_15::FaviconUri`]
    #[inline]
    pub fn favicon_uri(&self) -> windows_core::Result<String> {
        let mut value = windows_core::PWSTR::null();
        unsafe { self.as_ref().FaviconUri(&mut value) }?;
        Ok(crate::pwstr::take_pwstr(value))
    }

    /// [`ICoreWebView2_15::GetFavicon`]
    #[inline]
    pub fn get_favicon<P0: windows_core::Param<ICoreWebView2GetFaviconCompletedHandler>>(
        &self,
        format: COREWEBVIEW2_FAVICON_IMAGE_FORMAT,
        completedhandler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().GetFavicon(format, completedhandler) }
    }
}

/// [`ICoreWebView2_16`]
pub type WebView2_16 = SafeWrapper<ICoreWebView2_16>;

impl WebView2_16 {
    /// [`ICoreWebView2_16::Print`]
    #[inline]
    pub fn print<
        P0: windows_core::Param<ICoreWebView2PrintSettings>,
        P1: windows_core::Param<ICoreWebView2PrintCompletedHandler>,
    >(
        &self,
        printsettings: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().Print(printsettings, handler) }
    }

    /// [`ICoreWebView2_16::ShowPrintUI`]
    #[inline]
    pub fn show_print_ui(
        &self,
        printdialogkind: COREWEBVIEW2_PRINT_DIALOG_KIND,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ShowPrintUI(printdialogkind) }
    }

    /// [`ICoreWebView2_16::PrintToPdfStream`]
    #[inline]
    pub fn print_to_pdf_stream<
        P0: windows_core::Param<ICoreWebView2PrintSettings>,
        P1: windows_core::Param<ICoreWebView2PrintToPdfStreamCompletedHandler>,
    >(
        &self,
        printsettings: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().PrintToPdfStream(printsettings, handler) }
    }
}

/// [`ICoreWebView2_17`]
pub type WebView2_17 = SafeWrapper<ICoreWebView2_17>;

impl WebView2_17 {
    /// [`ICoreWebView2_17::PostSharedBufferToScript`]
    #[inline]
    pub fn post_shared_buffer_to_script<
        P0: windows_core::Param<ICoreWebView2SharedBuffer>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        sharedbuffer: P0,
        access: COREWEBVIEW2_SHARED_BUFFER_ACCESS,
        additionaldataasjson: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .PostSharedBufferToScript(sharedbuffer, access, additionaldataasjson)
        }
    }
}

/// [`ICoreWebView2_18`]
pub type WebView2_18 = SafeWrapper<ICoreWebView2_18>;

impl WebView2_18 {
    /// [`ICoreWebView2_18::add_LaunchingExternalUriScheme`]
    #[inline]
    pub fn add_launching_external_uri_scheme<
        P0: windows_core::Param<ICoreWebView2LaunchingExternalUriSchemeEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_LaunchingExternalUriScheme(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_18::remove_LaunchingExternalUriScheme`]
    #[inline]
    pub fn remove_launching_external_uri_scheme(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_LaunchingExternalUriScheme(token) }
    }
}

/// [`ICoreWebView2_19`]
pub type WebView2_19 = SafeWrapper<ICoreWebView2_19>;

impl WebView2_19 {
    /// [`ICoreWebView2_19::MemoryUsageTargetLevel`]
    #[inline]
    pub fn memory_usage_target_level(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL> {
        let mut value = Default::default();
        unsafe { self.as_ref().MemoryUsageTargetLevel(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2_19::SetMemoryUsageTargetLevel`]
    #[inline]
    pub fn set_memory_usage_target_level(
        &self,
        value: COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetMemoryUsageTargetLevel(value) }
    }
}

/// [`ICoreWebView2_2`]
pub type WebView2_2 = SafeWrapper<ICoreWebView2_2>;

impl WebView2_2 {
    /// [`ICoreWebView2_2::add_WebResourceResponseReceived`]
    #[inline]
    pub fn add_web_resource_response_received<
        P0: windows_core::Param<ICoreWebView2WebResourceResponseReceivedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_WebResourceResponseReceived(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_2::remove_WebResourceResponseReceived`]
    #[inline]
    pub fn remove_web_resource_response_received(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_WebResourceResponseReceived(token) }
    }

    /// [`ICoreWebView2_2::NavigateWithWebResourceRequest`]
    #[inline]
    pub fn navigate_with_web_resource_request<
        P0: windows_core::Param<ICoreWebView2WebResourceRequest>,
    >(
        &self,
        request: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().NavigateWithWebResourceRequest(request) }
    }

    /// [`ICoreWebView2_2::add_DOMContentLoaded`]
    #[inline]
    pub fn add_dom_content_loaded<
        P0: windows_core::Param<ICoreWebView2DOMContentLoadedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_DOMContentLoaded(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_2::remove_DOMContentLoaded`]
    #[inline]
    pub fn remove_dom_content_loaded(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_DOMContentLoaded(token) }
    }

    /// [`ICoreWebView2_2::CookieManager`]
    #[inline]
    pub fn cookie_manager(&self) -> windows_core::Result<ICoreWebView2CookieManager> {
        let result = unsafe { self.as_ref().CookieManager() }?;
        Ok(result)
    }

    /// [`ICoreWebView2_2::Environment`]
    #[inline]
    pub fn environment(&self) -> windows_core::Result<ICoreWebView2Environment> {
        let result = unsafe { self.as_ref().Environment() }?;
        Ok(result)
    }
}

/// [`ICoreWebView2_20`]
pub type WebView2_20 = SafeWrapper<ICoreWebView2_20>;

impl WebView2_20 {
    /// [`ICoreWebView2_20::FrameId`]
    #[inline]
    pub fn frame_id(&self) -> windows_core::Result<u32> {
        let mut value = Default::default();
        unsafe { self.as_ref().FrameId(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2_21`]
pub type WebView2_21 = SafeWrapper<ICoreWebView2_21>;

impl WebView2_21 {
    /// [`ICoreWebView2_21::ExecuteScriptWithResult`]
    #[inline]
    pub fn execute_script_with_result<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ExecuteScriptWithResultCompletedHandler>,
    >(
        &self,
        javascript: P0,
        handler: P1,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ExecuteScriptWithResult(javascript, handler) }
    }
}

/// [`ICoreWebView2_22`]
pub type WebView2_22 = SafeWrapper<ICoreWebView2_22>;

impl WebView2_22 {
    /// [`ICoreWebView2_22::AddWebResourceRequestedFilterWithRequestSourceKinds`]
    #[inline]
    pub fn add_web_resource_requested_filter_with_request_source_kinds<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
        requestsourcekinds: COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .AddWebResourceRequestedFilterWithRequestSourceKinds(
                    uri,
                    resourcecontext,
                    requestsourcekinds,
                )
        }
    }

    /// [`ICoreWebView2_22::RemoveWebResourceRequestedFilterWithRequestSourceKinds`]
    #[inline]
    pub fn remove_web_resource_requested_filter_with_request_source_kinds<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        uri: P0,
        resourcecontext: COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
        requestsourcekinds: COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .RemoveWebResourceRequestedFilterWithRequestSourceKinds(
                    uri,
                    resourcecontext,
                    requestsourcekinds,
                )
        }
    }
}

/// [`ICoreWebView2_23`]
pub type WebView2_23 = SafeWrapper<ICoreWebView2_23>;

impl WebView2_23 {
    /// [`ICoreWebView2_23::PostWebMessageAsJsonWithAdditionalObjects`]
    #[inline]
    pub fn post_web_message_as_json_with_additional_objects<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2ObjectCollectionView>,
    >(
        &self,
        webmessageasjson: P0,
        additionalobjects: P1,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .PostWebMessageAsJsonWithAdditionalObjects(webmessageasjson, additionalobjects)
        }
    }
}

/// [`ICoreWebView2_24`]
pub type WebView2_24 = SafeWrapper<ICoreWebView2_24>;

impl WebView2_24 {
    /// [`ICoreWebView2_24::add_NotificationReceived`]
    #[inline]
    pub fn add_notification_received<
        P0: windows_core::Param<ICoreWebView2NotificationReceivedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_NotificationReceived(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_24::remove_NotificationReceived`]
    #[inline]
    pub fn remove_notification_received(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_NotificationReceived(token) }
    }
}

/// [`ICoreWebView2_25`]
pub type WebView2_25 = SafeWrapper<ICoreWebView2_25>;

impl WebView2_25 {
    /// [`ICoreWebView2_25::add_SaveAsUIShowing`]
    #[inline]
    pub fn add_save_as_ui_showing<
        P0: windows_core::Param<ICoreWebView2SaveAsUIShowingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_SaveAsUIShowing(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_25::remove_SaveAsUIShowing`]
    #[inline]
    pub fn remove_save_as_ui_showing(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_SaveAsUIShowing(token) }
    }

    /// [`ICoreWebView2_25::ShowSaveAsUI`]
    #[inline]
    pub fn show_save_as_ui<P0: windows_core::Param<ICoreWebView2ShowSaveAsUICompletedHandler>>(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ShowSaveAsUI(handler) }
    }
}

/// [`ICoreWebView2_26`]
pub type WebView2_26 = SafeWrapper<ICoreWebView2_26>;

impl WebView2_26 {
    /// [`ICoreWebView2_26::add_SaveFileSecurityCheckStarting`]
    #[inline]
    pub fn add_save_file_security_check_starting<
        P0: windows_core::Param<ICoreWebView2SaveFileSecurityCheckStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_SaveFileSecurityCheckStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_26::remove_SaveFileSecurityCheckStarting`]
    #[inline]
    pub fn remove_save_file_security_check_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_SaveFileSecurityCheckStarting(token) }
    }
}

/// [`ICoreWebView2_27`]
pub type WebView2_27 = SafeWrapper<ICoreWebView2_27>;

impl WebView2_27 {
    /// [`ICoreWebView2_27::add_ScreenCaptureStarting`]
    #[inline]
    pub fn add_screen_capture_starting<
        P0: windows_core::Param<ICoreWebView2ScreenCaptureStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ScreenCaptureStarting(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_27::remove_ScreenCaptureStarting`]
    #[inline]
    pub fn remove_screen_capture_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ScreenCaptureStarting(token) }
    }
}

/// [`ICoreWebView2_3`]
pub type WebView2_3 = SafeWrapper<ICoreWebView2_3>;

impl WebView2_3 {
    /// [`ICoreWebView2_3::TrySuspend`]
    #[inline]
    pub fn try_suspend<P0: windows_core::Param<ICoreWebView2TrySuspendCompletedHandler>>(
        &self,
        handler: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().TrySuspend(handler) }
    }

    /// [`ICoreWebView2_3::Resume`]
    #[inline]
    pub fn resume(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().Resume() }
    }

    /// [`ICoreWebView2_3::IsSuspended`]
    #[inline]
    pub fn is_suspended(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut issuspended = Default::default();
        unsafe { self.as_ref().IsSuspended(&mut issuspended) }?;
        Ok(issuspended)
    }

    /// [`ICoreWebView2_3::SetVirtualHostNameToFolderMapping`]
    #[inline]
    pub fn set_virtual_host_name_to_folder_mapping<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        hostname: P0,
        folderpath: P1,
        accesskind: COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .SetVirtualHostNameToFolderMapping(hostname, folderpath, accesskind)
        }
    }

    /// [`ICoreWebView2_3::ClearVirtualHostNameToFolderMapping`]
    #[inline]
    pub fn clear_virtual_host_name_to_folder_mapping<
        P0: windows_core::Param<windows_core::PCWSTR>,
    >(
        &self,
        hostname: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().ClearVirtualHostNameToFolderMapping(hostname) }
    }
}

/// [`ICoreWebView2_4`]
pub type WebView2_4 = SafeWrapper<ICoreWebView2_4>;

impl WebView2_4 {
    /// [`ICoreWebView2_4::add_FrameCreated`]
    #[inline]
    pub fn add_frame_created<P0: windows_core::Param<ICoreWebView2FrameCreatedEventHandler>>(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_FrameCreated(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_4::remove_FrameCreated`]
    #[inline]
    pub fn remove_frame_created(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_FrameCreated(token) }
    }

    /// [`ICoreWebView2_4::add_DownloadStarting`]
    #[inline]
    pub fn add_download_starting<
        P0: windows_core::Param<ICoreWebView2DownloadStartingEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_DownloadStarting(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_4::remove_DownloadStarting`]
    #[inline]
    pub fn remove_download_starting(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_DownloadStarting(token) }
    }
}

/// [`ICoreWebView2_5`]
pub type WebView2_5 = SafeWrapper<ICoreWebView2_5>;

impl WebView2_5 {
    /// [`ICoreWebView2_5::add_ClientCertificateRequested`]
    #[inline]
    pub fn add_client_certificate_requested<
        P0: windows_core::Param<ICoreWebView2ClientCertificateRequestedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_ClientCertificateRequested(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_5::remove_ClientCertificateRequested`]
    #[inline]
    pub fn remove_client_certificate_requested(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_ClientCertificateRequested(token) }
    }
}

/// [`ICoreWebView2_6`]
pub type WebView2_6 = SafeWrapper<ICoreWebView2_6>;

impl WebView2_6 {
    /// [`ICoreWebView2_6::OpenTaskManagerWindow`]
    #[inline]
    pub fn open_task_manager_window(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().OpenTaskManagerWindow() }
    }
}

/// [`ICoreWebView2_7`]
pub type WebView2_7 = SafeWrapper<ICoreWebView2_7>;

impl WebView2_7 {
    /// [`ICoreWebView2_7::PrintToPdf`]
    #[inline]
    pub fn print_to_pdf<
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICoreWebView2PrintSettings>,
        P2: windows_core::Param<ICoreWebView2PrintToPdfCompletedHandler>,
    >(
        &self,
        resultfilepath: P0,
        printsettings: P1,
        handler: P2,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .PrintToPdf(resultfilepath, printsettings, handler)
        }
    }
}

/// [`ICoreWebView2_8`]
pub type WebView2_8 = SafeWrapper<ICoreWebView2_8>;

impl WebView2_8 {
    /// [`ICoreWebView2_8::add_IsMutedChanged`]
    #[inline]
    pub fn add_is_muted_changed<
        P0: windows_core::Param<ICoreWebView2IsMutedChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe { self.as_ref().add_IsMutedChanged(eventhandler, &mut token) }?;
        Ok(token)
    }

    /// [`ICoreWebView2_8::remove_IsMutedChanged`]
    #[inline]
    pub fn remove_is_muted_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_IsMutedChanged(token) }
    }

    /// [`ICoreWebView2_8::IsMuted`]
    #[inline]
    pub fn is_muted(&self) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsMuted(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2_8::SetIsMuted`]
    #[inline]
    pub fn set_is_muted<P0: windows_core::Param<windows::Win32::Foundation::BOOL>>(
        &self,
        value: P0,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetIsMuted(value) }
    }

    /// [`ICoreWebView2_8::add_IsDocumentPlayingAudioChanged`]
    #[inline]
    pub fn add_is_document_playing_audio_changed<
        P0: windows_core::Param<ICoreWebView2IsDocumentPlayingAudioChangedEventHandler>,
    >(
        &self,
        eventhandler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_IsDocumentPlayingAudioChanged(eventhandler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_8::remove_IsDocumentPlayingAudioChanged`]
    #[inline]
    pub fn remove_is_document_playing_audio_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().remove_IsDocumentPlayingAudioChanged(token) }
    }

    /// [`ICoreWebView2_8::IsDocumentPlayingAudio`]
    #[inline]
    pub fn is_document_playing_audio(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsDocumentPlayingAudio(&mut value) }?;
        Ok(value)
    }
}

/// [`ICoreWebView2_9`]
pub type WebView2_9 = SafeWrapper<ICoreWebView2_9>;

impl WebView2_9 {
    /// [`ICoreWebView2_9::add_IsDefaultDownloadDialogOpenChanged`]
    #[inline]
    pub fn add_is_default_download_dialog_open_changed<
        P0: windows_core::Param<ICoreWebView2IsDefaultDownloadDialogOpenChangedEventHandler>,
    >(
        &self,
        handler: P0,
    ) -> windows_core::Result<windows::Win32::System::WinRT::EventRegistrationToken> {
        let mut token = Default::default();
        unsafe {
            self.as_ref()
                .add_IsDefaultDownloadDialogOpenChanged(handler, &mut token)
        }?;
        Ok(token)
    }

    /// [`ICoreWebView2_9::remove_IsDefaultDownloadDialogOpenChanged`]
    #[inline]
    pub fn remove_is_default_download_dialog_open_changed(
        &self,
        token: windows::Win32::System::WinRT::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        unsafe {
            self.as_ref()
                .remove_IsDefaultDownloadDialogOpenChanged(token)
        }
    }

    /// [`ICoreWebView2_9::IsDefaultDownloadDialogOpen`]
    #[inline]
    pub fn is_default_download_dialog_open(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::BOOL> {
        let mut value = Default::default();
        unsafe { self.as_ref().IsDefaultDownloadDialogOpen(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2_9::OpenDefaultDownloadDialog`]
    #[inline]
    pub fn open_default_download_dialog(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().OpenDefaultDownloadDialog() }
    }

    /// [`ICoreWebView2_9::CloseDefaultDownloadDialog`]
    #[inline]
    pub fn close_default_download_dialog(&self) -> windows_core::Result<()> {
        unsafe { self.as_ref().CloseDefaultDownloadDialog() }
    }

    /// [`ICoreWebView2_9::DefaultDownloadDialogCornerAlignment`]
    #[inline]
    pub fn default_download_dialog_corner_alignment(
        &self,
    ) -> windows_core::Result<COREWEBVIEW2_DEFAULT_DOWNLOAD_DIALOG_CORNER_ALIGNMENT> {
        let mut value = Default::default();
        unsafe {
            self.as_ref()
                .DefaultDownloadDialogCornerAlignment(&mut value)
        }?;
        Ok(value)
    }

    /// [`ICoreWebView2_9::SetDefaultDownloadDialogCornerAlignment`]
    #[inline]
    pub fn set_default_download_dialog_corner_alignment(
        &self,
        value: COREWEBVIEW2_DEFAULT_DOWNLOAD_DIALOG_CORNER_ALIGNMENT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDefaultDownloadDialogCornerAlignment(value) }
    }

    /// [`ICoreWebView2_9::DefaultDownloadDialogMargin`]
    #[inline]
    pub fn default_download_dialog_margin(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::POINT> {
        let mut value = Default::default();
        unsafe { self.as_ref().DefaultDownloadDialogMargin(&mut value) }?;
        Ok(value)
    }

    /// [`ICoreWebView2_9::SetDefaultDownloadDialogMargin`]
    #[inline]
    pub fn set_default_download_dialog_margin(
        &self,
        value: windows::Win32::Foundation::POINT,
    ) -> windows_core::Result<()> {
        unsafe { self.as_ref().SetDefaultDownloadDialogMargin(value) }
    }
}
