use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;

use libc::size_t;

use crate::bindings::root::std::string;

cpp! {{
    #include <string>

    struct struct_with_strings {
        int32_t index;
        std::string first_name;
        std::string last_name;
    };
}}

#[repr(C)]
pub struct String(string);

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c_str().to_string_lossy())
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.c_str().to_string_lossy())
    }
}

impl String {
    pub fn len(&self) -> size_t {
        unsafe {
            cpp!([self as "const std::string*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    pub fn c_str(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(cpp!([self as "const std::string*"]
                  -> *const c_char as "const char*" {
                return self->c_str();
            }))
        }
    }

    pub fn assign<S: AsRef<CStr>>(&mut self, s: S) {
        let s = s.as_ref();
        let ptr = s.as_ptr();
        unsafe {
            cpp!([self as "std::string*", ptr as "const char*"] {
                self->assign(ptr);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[repr(C)]
    struct StructWithStrings {
        index: i32,
        first_name: String,
        last_name: String,
    }

    #[test]
    fn unittest_struct_with_strings() {
        let x = unsafe {
            cpp!([] -> &mut StructWithStrings as "struct_with_strings*" {
                static struct_with_strings x{23, "boncheol", "gu"};
                return &x;
            })
        };
        assert_eq!(x.index, 23);
        assert_eq!(x.first_name.c_str().to_string_lossy(), "boncheol");
        assert_eq!(x.last_name.c_str().to_string_lossy(), "gu");

        x.first_name.assign(CString::new("junmo").unwrap());
        assert_eq!(x.first_name.c_str().to_string_lossy(), "junmo");
    }
}
