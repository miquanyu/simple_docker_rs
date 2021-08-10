use std::ffi::{CStr, CString};
use std::os::raw::{c_char};

pub fn parse_one_string_to_cstr<'a>(str: &'a String) -> &'a CStr {
    let cstr;
    let c_string = CString::new(str.to_owned()).unwrap();
    unsafe {
        let c_world: *const c_char = c_string.into_raw() as *const c_char;
        cstr = CStr::from_ptr(c_world);
    }
    return cstr
}

pub fn parse_more_string_to_cstr<'a>(str_s: &'a [String]) ->  Vec<&'a CStr> {
    str_s.iter().map(|str| {
        parse_one_string_to_cstr(str)
    }).collect::<Vec<&CStr>>()
}