use std::{cell::UnsafeCell, default::Default};

use windows::{
    core::{Result, PCWSTR, PWSTR},
    Win32::Foundation::{BOOL, E_POINTER},
};

use windows_implement::implement;

use crate::{
    pwstr::{pwstr_from_str, string_from_pcwstr},
    Microsoft::Web::WebView2::Win32::{
        ICoreWebView2EnvironmentOptions, ICoreWebView2EnvironmentOptions_Impl,
        CORE_WEBVIEW_TARGET_PRODUCT_VERSION,
    },
};

#[implement(ICoreWebView2EnvironmentOptions)]
pub struct CoreWebView2EnvironmentOptions {
    additional_browser_arguments: UnsafeCell<String>,
    language: UnsafeCell<String>,
    target_compatible_browser_version: UnsafeCell<String>,
    allow_single_sign_on_using_os_primary_account: UnsafeCell<bool>,
}

impl Default for CoreWebView2EnvironmentOptions {
    fn default() -> Self {
        Self {
            additional_browser_arguments: String::new().into(),
            language: String::new().into(),
            target_compatible_browser_version: unsafe {
                CORE_WEBVIEW_TARGET_PRODUCT_VERSION.to_string()
            }
            .unwrap_or_default()
            .into(),
            allow_single_sign_on_using_os_primary_account: false.into(),
        }
    }
}

#[allow(non_snake_case)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl ICoreWebView2EnvironmentOptions_Impl for CoreWebView2EnvironmentOptions {
    fn AdditionalBrowserArguments(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = pwstr_from_str((*self.additional_browser_arguments.get()).as_str())
            };
            Ok(())
        }
    }

    fn SetAdditionalBrowserArguments(&self, value: &PCWSTR) -> Result<()> {
        unsafe { *self.additional_browser_arguments.get() = string_from_pcwstr(value) };
        Ok(())
    }

    fn Language(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = pwstr_from_str((*self.language.get()).as_str()) };
            Ok(())
        }
    }

    fn SetLanguage(&self, value: &PCWSTR) -> Result<()> {
        unsafe { *self.language.get() = string_from_pcwstr(value) };
        Ok(())
    }

    fn TargetCompatibleBrowserVersion(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = pwstr_from_str((*self.target_compatible_browser_version.get()).as_str())
            };
            Ok(())
        }
    }

    fn SetTargetCompatibleBrowserVersion(&self, value: &PCWSTR) -> Result<()> {
        unsafe { *self.target_compatible_browser_version.get() = string_from_pcwstr(value) };
        Ok(())
    }

    fn AllowSingleSignOnUsingOSPrimaryAccount(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = (*self.allow_single_sign_on_using_os_primary_account.get()).into() };
            Ok(())
        }
    }

    fn SetAllowSingleSignOnUsingOSPrimaryAccount(&self, value: BOOL) -> Result<()> {
        unsafe {
            *self.allow_single_sign_on_using_os_primary_account.get() = value.into();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use windows::w;

    use crate::{
        pwstr::take_pwstr,
        Microsoft::Web::WebView2::Win32::{
            ICoreWebView2EnvironmentOptions, CORE_WEBVIEW_TARGET_PRODUCT_VERSION,
        },
    };

    use super::*;

    #[test]
    fn additional_arguments() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetAdditionalBrowserArguments(w!("FakeArguments")) }.unwrap();
        let mut result = PWSTR(ptr::null_mut());
        unsafe { options.AdditionalBrowserArguments(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, "FakeArguments");
    }

    #[test]
    fn override_language() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetLanguage(w!("FakeLanguage")) }.unwrap();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.Language(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, "FakeLanguage");
    }

    #[test]
    fn default_version() {
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.TargetCompatibleBrowserVersion(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(
            result,
            unsafe { CORE_WEBVIEW_TARGET_PRODUCT_VERSION.to_string() }.unwrap()
        );
    }

    #[test]
    fn override_version() {
        assert_ne!(
            "FakeVersion",
            unsafe { CORE_WEBVIEW_TARGET_PRODUCT_VERSION.to_string() }
                .unwrap()
                .as_str()
        );
        let options: ICoreWebView2EnvironmentOptions =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetTargetCompatibleBrowserVersion(w!("FakeVersion")) }.unwrap();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { options.TargetCompatibleBrowserVersion(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, "FakeVersion");
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
