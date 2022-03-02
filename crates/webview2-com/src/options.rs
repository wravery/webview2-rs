use std::{default::Default, sync::Mutex};

use windows::{
    core::{Result, PCWSTR, PWSTR},
    Win32::Foundation::{BOOL, E_POINTER},
};

use windows_implement::implement;

use crate::{
    pwstr::{pwstr_from_str, string_from_pcwstr},
    Microsoft,
};

#[implement(Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions)]
pub struct CoreWebView2EnvironmentOptions {
    additional_browser_arguments: Mutex<String>,
    language: Mutex<String>,
    target_compatible_browser_version: Mutex<String>,
    allow_single_sign_on_using_os_primary_account: Mutex<bool>,
}

impl Default for CoreWebView2EnvironmentOptions {
    fn default() -> Self {
        Self {
            additional_browser_arguments: Mutex::new(String::new()),
            language: Mutex::new(String::new()),
            target_compatible_browser_version: Mutex::new(
                Microsoft::Web::WebView2::Win32::CORE_WEBVIEW_TARGET_PRODUCT_VERSION.into(),
            ),
            allow_single_sign_on_using_os_primary_account: Mutex::new(false),
        }
    }
}

#[allow(non_snake_case)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions_Impl
    for CoreWebView2EnvironmentOptions
{
    fn AdditionalBrowserArguments(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = pwstr_from_str(
                    self.additional_browser_arguments
                        .lock()
                        .expect("lock additional_browser_arguments")
                        .as_str(),
                )
            };
            Ok(())
        }
    }

    fn SetAdditionalBrowserArguments(&self, value: &PCWSTR) -> Result<()> {
        *self
            .additional_browser_arguments
            .lock()
            .expect("lock additional_browser_arguments") = string_from_pcwstr(value);
        Ok(())
    }

    fn Language(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = pwstr_from_str(self.language.lock().expect("lock language").as_str())
            };
            Ok(())
        }
    }

    fn SetLanguage(&self, value: &PCWSTR) -> Result<()> {
        *self.language.lock().expect("lock language") = string_from_pcwstr(value);
        Ok(())
    }

    fn TargetCompatibleBrowserVersion(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = pwstr_from_str(
                    self.target_compatible_browser_version
                        .lock()
                        .expect("lock target_compatible_browser_version")
                        .as_str(),
                )
            };
            Ok(())
        }
    }

    fn SetTargetCompatibleBrowserVersion(&self, value: &PCWSTR) -> Result<()> {
        *self
            .target_compatible_browser_version
            .lock()
            .expect("lock target_compatible_browser_version") = string_from_pcwstr(value);
        Ok(())
    }

    fn AllowSingleSignOnUsingOSPrimaryAccount(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe {
                *result = (*self
                    .allow_single_sign_on_using_os_primary_account
                    .lock()
                    .expect("lock allow_single_sign_on_using_os_primary_account"))
                .into()
            };
            Ok(())
        }
    }

    fn SetAllowSingleSignOnUsingOSPrimaryAccount(&self, value: BOOL) -> Result<()> {
        *self
            .allow_single_sign_on_using_os_primary_account
            .lock()
            .expect("lock allow_single_sign_on_using_os_primary_account") = value.into();
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
        unsafe {
            options.SetAdditionalBrowserArguments(&PCWSTR(pwstr_from_str(ADDITIONAL_ARGUMENTS).0))
        }
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
        unsafe { options.SetLanguage(&PCWSTR(pwstr_from_str(OVERRIDE_LANGUAGE).0)) }.unwrap();
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
        unsafe {
            options.SetTargetCompatibleBrowserVersion(&PCWSTR(pwstr_from_str(OVERRIDE_VERSION).0))
        }
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
