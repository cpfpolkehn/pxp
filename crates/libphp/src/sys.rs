#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::ffi::c_void;

#[link(name = "libphp")]
extern "C" {
    
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));