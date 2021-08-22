use crate::{
    pwstr::{pwstr_from_str, string_from_pwstr},
    Microsoft,
    Windows::{
        self,
        Win32::Foundation::{BOOL, PWSTR},
    },
};
use windows::*;

#[implement(Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions)]
pub struct CoreWebView2EnvironmentOptions {
    additional_browser_arguments: String,
    language: String,
    target_compatible_browser_version: String,
    allow_single_sign_on_using_os_primary_account: bool,
}

#[allow(non_snake_case)]
impl CoreWebView2EnvironmentOptions {
    pub fn new() -> Self {
        pub const DESIRED_VERSION: &str = "89.0.0.0";

        Self {
            additional_browser_arguments: String::new(),
            language: String::new(),
            target_compatible_browser_version: DESIRED_VERSION.into(),
            allow_single_sign_on_using_os_primary_account: false,
        }
    }

    pub fn AdditionalBrowserArguments(&self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.additional_browser_arguments) };
        Ok(())
    }

    pub fn SetAdditionalBrowserArguments(&mut self, value: PWSTR) -> Result<()> {
        self.additional_browser_arguments = string_from_pwstr(value);
        Ok(())
    }

    pub fn Language(&self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.language) };
        Ok(())
    }

    pub fn SetLanguage(&mut self, value: PWSTR) -> Result<()> {
        self.language = string_from_pwstr(value);
        Ok(())
    }

    pub fn TargetCompatibleBrowserVersion(&self, result: *mut PWSTR) -> Result<()> {
        unsafe { *result = pwstr_from_str(&self.target_compatible_browser_version) };
        Ok(())
    }

    pub fn SetTargetCompatibleBrowserVersion(&mut self, value: PWSTR) -> Result<()> {
        self.target_compatible_browser_version = string_from_pwstr(value);
        Ok(())
    }

    pub fn AllowSingleSignOnUsingOSPrimaryAccount(&self, result: *mut BOOL) -> Result<()> {
        unsafe { *result = self.allow_single_sign_on_using_os_primary_account.into() };
        Ok(())
    }

    pub fn SetAllowSingleSignOnUsingOSPrimaryAccount(&mut self, value: BOOL) -> Result<()> {
        self.allow_single_sign_on_using_os_primary_account = value.into();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use super::*;
    use crate::pwstr::take_pwstr;

    #[test]
    fn default_version() {
        let options = CoreWebView2EnvironmentOptions::new();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        options.TargetCompatibleBrowserVersion(&mut result).unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, "89.0.0.0");
    }

    #[test]
    fn override_version() {
        const OVERRIDE_VERSION: &str = "90.0.0.0";
        let mut options = CoreWebView2EnvironmentOptions::new();
        options.SetTargetCompatibleBrowserVersion(pwstr_from_str(OVERRIDE_VERSION)).unwrap();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        options.TargetCompatibleBrowserVersion(&mut result).unwrap();
        let result = take_pwstr(result);
        assert_eq!(&result, OVERRIDE_VERSION);
    }
}
