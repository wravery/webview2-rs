#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal fn $($function:tt)*) => (
        #[cfg_attr(
            target_env = "msvc",
            link(name = "WebView2LoaderStatic", kind = "static")
        )]
        #[cfg_attr(
            not(target_env = "msvc"),
            link(name = "WebView2Loader.dll")
        )]
        extern $abi {
            pub fn $($function)*;
        }
    )
}

mod bindings;

#[allow(non_snake_case)]
pub mod Microsoft {
    pub mod Web {
        pub mod WebView2 {
            pub mod Win32 {
                pub use crate::bindings::*;
            }
        }
    }
}

pub mod declared_interfaces;

#[cfg(test)]
mod test {
    use windows_core::w;

    use crate::Microsoft::Web::WebView2::Win32::*;

    #[test]
    fn compare_eq() {
        let mut result = 1;
        unsafe { CompareBrowserVersions(w!("1.0.0"), w!("1.0.0"), &mut result) }.unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn compare_lt() {
        let mut result = 0;
        unsafe { CompareBrowserVersions(w!("1.0.0"), w!("1.0.1"), &mut result) }.unwrap();
        assert_eq!(-1, result);
    }

    #[test]
    fn compare_gt() {
        let mut result = 0;
        unsafe { CompareBrowserVersions(w!("2.0.0"), w!("1.0.1"), &mut result) }.unwrap();
        assert_eq!(1, result);
    }
}
