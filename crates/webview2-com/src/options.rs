use std::default::Default;

use windows::{
    core::Result,
    Win32::Foundation::{BOOL, PWSTR},
};

use windows_implement::implement;

use crate::{
    pwstr::{pwstr_from_str, string_from_pwstr},
    Microsoft,
};

#[implement(Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions)]
pub struct CoreWebView2EnvironmentOptions {
    additional_browser_arguments: String,
    language: String,
    target_compatible_browser_version: String,
    allow_single_sign_on_using_os_primary_account: bool,
}

impl Default for CoreWebView2EnvironmentOptions {
    fn default() -> Self {
        Self {
            additional_browser_arguments: String::new(),
            language: String::new(),
            target_compatible_browser_version:
                Microsoft::Web::WebView2::Win32::CORE_WEBVIEW_TARGET_PRODUCT_VERSION.into(),
            allow_single_sign_on_using_os_primary_account: false,
        }
    }
}

#[allow(non_snake_case)]
impl Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions_Impl
    for CoreWebView2EnvironmentOptions
{
    fn AdditionalBrowserArguments(&mut self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.additional_browser_arguments) };
        Ok(())
    }

    fn SetAdditionalBrowserArguments(&mut self, value: PWSTR) -> Result<()> {
        self.additional_browser_arguments = string_from_pwstr(&value);
        Ok(())
    }

    fn Language(&mut self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.language) };
        Ok(())
    }

    fn SetLanguage(&mut self, value: PWSTR) -> Result<()> {
        self.language = string_from_pwstr(&value);
        Ok(())
    }

    fn TargetCompatibleBrowserVersion(&mut self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.target_compatible_browser_version) };
        Ok(())
    }

    fn SetTargetCompatibleBrowserVersion(&mut self, value: PWSTR) -> Result<()> {
        self.target_compatible_browser_version = string_from_pwstr(&value);
        Ok(())
    }

    fn AllowSingleSignOnUsingOSPrimaryAccount(&mut self, result: *mut BOOL) -> Result<()> {
        unsafe { *result = self.allow_single_sign_on_using_os_primary_account.into() };
        Ok(())
    }

    fn SetAllowSingleSignOnUsingOSPrimaryAccount(&mut self, value: BOOL) -> Result<()> {
        self.allow_single_sign_on_using_os_primary_account = value.into();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use crate::{
        pwstr::take_pwstr, Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions,
    };

    use super::*;

    #[test]
    fn additional_arguments() {
        const ADDITIONAL_ARGUMENTS: &str = "FakeArguments";
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetAdditionalBrowserArguments(pwstr_from_str(ADDITIONAL_ARGUMENTS)) }
            .unwrap();
        let mut result = PWSTR(ptr::null_mut());
        unsafe { options.AdditionalBrowserArguments(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, ADDITIONAL_ARGUMENTS);
    }

    #[test]
    fn override_language() {
        const OVERRIDE_LANGUAGE: &str = "FakeLanguage";
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetLanguage(pwstr_from_str(OVERRIDE_LANGUAGE)) }.unwrap();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.Language(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, OVERRIDE_LANGUAGE);
    }

    #[test]
    fn default_version() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.TargetCompatibleBrowserVersion(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(
            &result,
            Microsoft::Web::WebView2::Win32::CORE_WEBVIEW_TARGET_PRODUCT_VERSION
        );
    }

    #[test]
    fn override_version() {
        const OVERRIDE_VERSION: &str = "FakeVersion";
        assert_ne!(
            OVERRIDE_VERSION,
            Microsoft::Web::WebView2::Win32::CORE_WEBVIEW_TARGET_PRODUCT_VERSION
        );
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetTargetCompatibleBrowserVersion(pwstr_from_str(OVERRIDE_VERSION)) }
            .unwrap();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.TargetCompatibleBrowserVersion(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, OVERRIDE_VERSION);
    }

    #[test]
    fn default_allow_sso() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        let mut result = BOOL(1);
        unsafe { options.AllowSingleSignOnUsingOSPrimaryAccount(&mut result) }.unwrap();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn override_allow_sso() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetAllowSingleSignOnUsingOSPrimaryAccount(BOOL(1)) }.unwrap();
        let mut result = BOOL(0);
        unsafe { options.AllowSingleSignOnUsingOSPrimaryAccount(&mut result) }.unwrap();
        assert_eq!(result.0, 1);
    }
}
