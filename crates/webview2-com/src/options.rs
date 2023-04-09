#![allow(non_snake_case)]
use std::{cell::UnsafeCell, default::Default, ffi::c_void, mem, ptr};

use windows::{
    core::{Error, IUnknown, IUnknown_Vtbl, Result, Vtable, HRESULT, PCWSTR, PWSTR},
    Win32::{
        Foundation::{BOOL, E_POINTER, E_UNEXPECTED, S_OK},
        System::Com::CoTaskMemAlloc,
    },
};

use windows_implement::implement;
use windows_interface::interface;

use crate::{
    pwstr::{pwstr_from_str, string_from_pcwstr},
    Microsoft::Web::WebView2::Win32::{
        ICoreWebView2CustomSchemeRegistration, ICoreWebView2CustomSchemeRegistration_Impl,
        ICoreWebView2EnvironmentOptions, ICoreWebView2EnvironmentOptions2,
        ICoreWebView2EnvironmentOptions2_Impl, ICoreWebView2EnvironmentOptions3,
        ICoreWebView2EnvironmentOptions3_Impl, ICoreWebView2EnvironmentOptions_Impl,
        CORE_WEBVIEW_TARGET_PRODUCT_VERSION,
    },
};

#[implement(
    ICoreWebView2EnvironmentOptions,
    ICoreWebView2EnvironmentOptions2,
    ICoreWebView2EnvironmentOptions3,
    IFixedEnvironmentOptions4
)]
pub struct CoreWebView2EnvironmentOptions {
    additional_browser_arguments: UnsafeCell<String>,
    language: UnsafeCell<String>,
    target_compatible_browser_version: UnsafeCell<String>,
    allow_single_sign_on_using_os_primary_account: UnsafeCell<bool>,
    exclusive_user_data_folder_access: UnsafeCell<bool>,
    is_custom_crash_reporting_enabled: UnsafeCell<bool>,
    scheme_registrations: UnsafeCell<Vec<Option<ICoreWebView2CustomSchemeRegistration>>>,
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
            exclusive_user_data_folder_access: false.into(),
            is_custom_crash_reporting_enabled: false.into(),
            scheme_registrations: Vec::new().into(),
        }
    }
}

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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl ICoreWebView2EnvironmentOptions2_Impl for CoreWebView2EnvironmentOptions {
    fn ExclusiveUserDataFolderAccess(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = (*self.exclusive_user_data_folder_access.get()).into() };
            Ok(())
        }
    }

    fn SetExclusiveUserDataFolderAccess(&self, value: BOOL) -> Result<()> {
        unsafe {
            *self.exclusive_user_data_folder_access.get() = value.into();
        }
        Ok(())
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl ICoreWebView2EnvironmentOptions3_Impl for CoreWebView2EnvironmentOptions {
    fn IsCustomCrashReportingEnabled(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = (*self.is_custom_crash_reporting_enabled.get()).into() };
            Ok(())
        }
    }

    fn SetIsCustomCrashReportingEnabled(&self, value: BOOL) -> Result<()> {
        unsafe {
            *self.is_custom_crash_reporting_enabled.get() = value.into();
        }
        Ok(())
    }
}

/// This is an alternate declaration of the [`crate::Microsoft::Web::WebView2::Win32::ICoreWebView2EnvironmentOptions4`]
/// interface, which matches the parameters for `SetCustomSchemeRegistrations`. The `windows`
/// crate mistakenly interprets the array of interface pointers as a pointer to an out-param,
/// and it converts that into a `Result<Option<ICoreWebView2CustomSchemeRegistration>>`.
#[interface("AC52D13F-0D38-475A-9DCA-876580D6793E")]
pub unsafe trait IFixedEnvironmentOptions4: IUnknown {
    fn GetCustomSchemeRegistrations(
        &self,
        count: *mut u32,
        scheme_registrations: *mut *mut *mut c_void,
    ) -> HRESULT;

    fn SetCustomSchemeRegistrations(
        &self,
        count: u32,
        scheme_registrations: *const *mut c_void,
    ) -> HRESULT;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl IFixedEnvironmentOptions4_Impl for CoreWebView2EnvironmentOptions {
    unsafe fn GetCustomSchemeRegistrations(
        &self,
        count: *mut u32,
        results: *mut *mut *mut c_void,
    ) -> HRESULT {
        if count.is_null() || results.is_null() {
            E_POINTER
        } else {
            let scheme_registrations = &*self.scheme_registrations.get();
            if let Ok(length) = scheme_registrations.len().try_into() {
                *count = length;
                if !scheme_registrations.is_empty() {
                    *results = mem::transmute(CoTaskMemAlloc(
                        mem::size_of::<*mut c_void>() * (*scheme_registrations).len(),
                    ));
                    let results =
                        ptr::slice_from_raw_parts_mut(*results, scheme_registrations.len());
                    for (i, scheme) in scheme_registrations.iter().enumerate() {
                        (*results)[i] = scheme
                            .clone()
                            .map_or(ptr::null_mut(), |scheme| scheme.into_raw())
                    }
                } else {
                    *results = ptr::null_mut();
                }
                S_OK
            } else {
                E_UNEXPECTED
            }
        }
    }

    unsafe fn SetCustomSchemeRegistrations(
        &self,
        count: u32,
        values: *const *mut c_void,
    ) -> HRESULT {
        if let Ok(count) = count.try_into() {
            let scheme_registrations = &mut *self.scheme_registrations.get();
            scheme_registrations.clear();
            scheme_registrations.reserve_exact(count);
            let values = &*ptr::slice_from_raw_parts(values, count);
            for &scheme in values.iter() {
                scheme_registrations.push(if scheme.is_null() {
                    None
                } else {
                    Some(ICoreWebView2CustomSchemeRegistration::from_raw_borrowed(&scheme).clone())
                });
            }
            S_OK
        } else {
            E_UNEXPECTED
        }
    }
}

#[implement(ICoreWebView2CustomSchemeRegistration)]
pub struct CoreWebView2CustomSchemeRegistration {
    scheme_name: String,
    treat_as_secure: UnsafeCell<bool>,
    allowed_origins: UnsafeCell<Vec<String>>,
    has_authority_component: UnsafeCell<bool>,
}

impl CoreWebView2CustomSchemeRegistration {
    pub fn new(scheme_name: String) -> Self {
        Self {
            scheme_name,
            treat_as_secure: false.into(),
            allowed_origins: Vec::new().into(),
            has_authority_component: false.into(),
        }
    }
}

impl ICoreWebView2CustomSchemeRegistration_Impl for CoreWebView2CustomSchemeRegistration {
    fn SchemeName(&self, result: *mut PWSTR) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = pwstr_from_str(&self.scheme_name) };
            Ok(())
        }
    }

    fn TreatAsSecure(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = (*self.treat_as_secure.get()).into() };
            Ok(())
        }
    }

    fn SetTreatAsSecure(&self, value: BOOL) -> Result<()> {
        unsafe {
            *self.treat_as_secure.get() = value.into();
        }
        Ok(())
    }

    fn GetAllowedOrigins(&self, count: *mut u32, results: *mut *mut PWSTR) -> Result<()> {
        unsafe {
            let allowed_origins = &*self.allowed_origins.get();
            *count = allowed_origins
                .len()
                .try_into()
                .map_err(|_| Error::from(E_UNEXPECTED))?;
            if !allowed_origins.is_empty() {
                *results = mem::transmute(CoTaskMemAlloc(
                    mem::size_of::<*mut PWSTR>() * (*allowed_origins).len(),
                ));
                let results = ptr::slice_from_raw_parts_mut(*results, allowed_origins.len());
                for (i, scheme) in allowed_origins.iter().enumerate() {
                    (*results)[i] = pwstr_from_str(scheme);
                }
            } else {
                *results = ptr::null_mut();
            }
        }
        Ok(())
    }

    fn SetAllowedOrigins(&self, count: u32, values: *mut PWSTR) -> Result<()> {
        unsafe {
            let count = count.try_into().map_err(|_| Error::from(E_UNEXPECTED))?;
            let allowed_origins = &mut *self.allowed_origins.get();
            allowed_origins.clear();
            allowed_origins.reserve_exact(count);
            let values = &*ptr::slice_from_raw_parts(values, count);
            for &origin in values.iter() {
                allowed_origins.push(string_from_pcwstr(&PCWSTR(origin.0)));
            }
        }
        Ok(())
    }

    fn HasAuthorityComponent(&self, result: *mut BOOL) -> Result<()> {
        if result.is_null() {
            E_POINTER.ok()
        } else {
            unsafe { *result = (*self.has_authority_component.get()).into() };
            Ok(())
        }
    }

    fn SetHasAuthorityComponent(&self, value: BOOL) -> Result<()> {
        unsafe {
            *self.has_authority_component.get() = value.into();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use windows::{w, Win32::System::Com::CoTaskMemFree};

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

    #[test]
    fn default_exclusive_data_folder() {
        let options: ICoreWebView2EnvironmentOptions2 =
            CoreWebView2EnvironmentOptions::default().into();
        let mut result = BOOL(1);
        unsafe { options.ExclusiveUserDataFolderAccess(&mut result) }.unwrap();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn override_exclusive_data_folder() {
        let options: ICoreWebView2EnvironmentOptions2 =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetExclusiveUserDataFolderAccess(BOOL(1)) }.unwrap();
        let mut result = BOOL(0);
        unsafe { options.ExclusiveUserDataFolderAccess(&mut result) }.unwrap();
        assert_eq!(result.0, 1);
    }

    #[test]
    fn default_custom_crash_reporting() {
        let options: ICoreWebView2EnvironmentOptions3 =
            CoreWebView2EnvironmentOptions::default().into();
        let mut result = BOOL(1);
        unsafe { options.IsCustomCrashReportingEnabled(&mut result) }.unwrap();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn override_custom_crash_reporting() {
        let options: ICoreWebView2EnvironmentOptions3 =
            CoreWebView2EnvironmentOptions::default().into();
        unsafe { options.SetIsCustomCrashReportingEnabled(BOOL(1)) }.unwrap();
        let mut result = BOOL(0);
        unsafe { options.IsCustomCrashReportingEnabled(&mut result) }.unwrap();
        assert_eq!(result.0, 1);
    }

    #[test]
    fn default_scheme_registrations() {
        let options: IFixedEnvironmentOptions4 = CoreWebView2EnvironmentOptions::default().into();
        let mut count = 1;
        let mut scheme_registrations = ptr::null_mut();
        assert!(unsafe {
            options.GetCustomSchemeRegistrations(&mut count, &mut scheme_registrations)
        }
        .is_ok());
        assert_eq!(0, count);
        assert_eq!(ptr::null_mut(), scheme_registrations);
    }

    #[test]
    fn override_scheme_registrations() {
        let options: IFixedEnvironmentOptions4 = CoreWebView2EnvironmentOptions::default().into();
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        assert!(
            unsafe { options.SetCustomSchemeRegistrations(1, &[scheme.as_raw()] as *const _) }
                .is_ok()
        );
        let mut count = 0;
        let mut scheme_registrations = ptr::null_mut();
        assert!(unsafe {
            options.GetCustomSchemeRegistrations(&mut count, &mut scheme_registrations)
        }
        .is_ok());
        assert_eq!(1, count);
        unsafe {
            let scheme_registration =
                ICoreWebView2CustomSchemeRegistration::from_raw(*scheme_registrations);
            assert_eq!(scheme.as_raw(), scheme_registration.as_raw());
            CoTaskMemFree(Some(scheme_registrations as *const _));
        }
    }

    #[test]
    fn scheme_name() {
        const SCHEME_NAME: &str = "custom";
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(SCHEME_NAME.to_string()).into();
        let mut result = PWSTR(ptr::null_mut::<u16>());
        unsafe { scheme.SchemeName(&mut result) }.unwrap();
        let result = take_pwstr(result);
        assert_eq!(result, SCHEME_NAME);
    }

    #[test]
    fn default_treat_as_secure() {
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        let mut result = BOOL(1);
        unsafe { scheme.TreatAsSecure(&mut result) }.unwrap();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn override_treat_as_secure() {
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        unsafe { scheme.SetTreatAsSecure(BOOL(1)) }.unwrap();
        let mut result = BOOL(0);
        unsafe { scheme.TreatAsSecure(&mut result) }.unwrap();
        assert_eq!(result.0, 1);
    }

    #[test]
    fn default_allowed_origins() {
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        let mut count = 1_u32;
        let mut origin = pwstr_from_str("origin");
        let mut results = &mut origin as *mut _;
        let _ = take_pwstr(origin);
        unsafe { scheme.GetAllowedOrigins(&mut count, &mut results) }.unwrap();
        assert_eq!(count, 0);
        assert_eq!(results, ptr::null_mut());
    }

    #[test]
    fn override_allowed_origins() {
        const ORIGIN: &str = "origin";
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        let mut origin = pwstr_from_str(ORIGIN);
        unsafe { scheme.SetAllowedOrigins(1, &mut origin) }.unwrap();

        let mut count = 0_u32;
        let mut results = ptr::null_mut();
        unsafe { scheme.GetAllowedOrigins(&mut count, &mut results) }.unwrap();
        assert_eq!(count, 1);

        assert_ne!(results, ptr::null_mut());
        let mut origin = PWSTR(ptr::null_mut());
        unsafe { mem::swap(&mut origin, &mut *results) };
        let origin = take_pwstr(origin);
        unsafe { CoTaskMemFree(Some(mem::transmute(results))) };
        assert_eq!(origin, ORIGIN);
    }

    #[test]
    fn default_has_authority_component() {
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        let mut result = BOOL(1);
        unsafe { scheme.HasAuthorityComponent(&mut result) }.unwrap();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn override_has_authority_component() {
        let scheme: ICoreWebView2CustomSchemeRegistration =
            CoreWebView2CustomSchemeRegistration::new(String::new()).into();
        unsafe { scheme.SetHasAuthorityComponent(BOOL(1)) }.unwrap();
        let mut result = BOOL(0);
        unsafe { scheme.HasAuthorityComponent(&mut result) }.unwrap();
        assert_eq!(result.0, 1);
    }
}
