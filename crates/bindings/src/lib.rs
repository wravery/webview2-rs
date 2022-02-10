#[allow(non_snake_case)]
pub mod Microsoft {
    #[allow(non_snake_case)]
    pub mod Web {
        #[allow(non_snake_case)]
        pub mod WebView2 {
            #[allow(non_snake_case)]
            pub mod Win32;
        }
    }
}

pub(crate) use ::windows as Windows;

pub mod callback_interfaces;

#[cfg(test)]
mod test {
    use crate::Microsoft::Web::WebView2::Win32::*;

    #[test]
    fn compare_eq() {
        let mut result = 1;
        unsafe { CompareBrowserVersions("1.0.0", "1.0.0", &mut result) }.unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn compare_lt() {
        let mut result = 0;
        unsafe { CompareBrowserVersions("1.0.0", "1.0.1", &mut result) }.unwrap();
        assert_eq!(-1, result);
    }

    #[test]
    fn compare_gt() {
        let mut result = 0;
        unsafe { CompareBrowserVersions("2.0.0", "1.0.1", &mut result) }.unwrap();
        assert_eq!(1, result);
    }
}
