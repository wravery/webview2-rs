use std::{mem, ptr};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{Globalization::lstrlenW, System::Com},
};

/// Copy a [`PCWSTR`] from an input param to a [`String`].
pub fn string_from_pcwstr(source: &PCWSTR) -> String {
    if source.0.is_null() {
        String::new()
    } else {
        let len = unsafe { lstrlenW(source) };

        if len > 0 {
            unsafe {
                let buffer = ptr::slice_from_raw_parts(source.0, len as usize);
                String::from_utf16_lossy(&*buffer)
            }
        } else {
            String::new()
        }
    }
}

/// Copy a [`PWSTR`] allocated with [`Com::CoTaskMemAlloc`] from an input param to a [`String`]
/// and free the original buffer with [`Com::CoTaskMemFree`].
pub fn take_pwstr(source: PWSTR) -> String {
    let result = string_from_pcwstr(&PCWSTR(source.0));

    if !source.0.is_null() {
        unsafe {
            Com::CoTaskMemFree(mem::transmute(source.0));
        }
    }

    result
}

/// Allocate a [`PWSTR`] with [`Com::CoTaskMemAlloc`] and copy a [`&str`] into it.
pub fn pwstr_from_str(source: &str) -> PWSTR {
    match source {
        "" => PWSTR(ptr::null_mut::<u16>()),
        value => {
            let encoded: Vec<_> = value.encode_utf16().chain(std::iter::once(0)).collect();

            unsafe {
                let mut buffer =
                    Com::CoTaskMemAlloc(encoded.len() * mem::size_of::<u16>()) as *mut u16;
                let result = PWSTR(buffer);

                for char in encoded {
                    *buffer = char;
                    buffer = buffer.add(1);
                }

                result
            }
        }
    }
}
