// This also links to the C library
#[allow(unused_imports)]
use bindgens;

use std::ffi::CString;

mod ffi {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        dead_code
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern fn my_callback() -> i64 {
    -1337
}

fn main() {
    unsafe {
        ffi::print_uint8(42);
        ffi::print_callback_result(Some(my_callback));

        let circle_name = CString::new("Circly Junior").unwrap();
        let circle = ffi::circle_t {
            radius: 13,
            name: circle_name.as_ptr(),
        };
        ffi::print_circle(circle);

        ffi::call_rust_from_c();
    }
}
