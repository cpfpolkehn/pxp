use std::ffi::{c_void, CString};

use crate::sys::{libphp_sapi_startup, php_module_shutdown, php_module_startup, php_request_shutdown, php_request_startup, sapi_header_struct, sapi_module_struct, sapi_shutdown, sapi_startup, zend_module_entry, ZEND_RESULT_CODE_SUCCESS};
use crate::result::Result;

type SapiModule = sapi_module_struct;

#[derive(Debug)]
pub struct Sapi {
    module: SapiModule,
}

impl Sapi {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        Ok(Self {
            module: SapiModule {
                name: CString::new(name.into())?.into_raw(),
                send_header: Some(sapi_send_header),
                ..Default::default()
            },
        })
    }

    pub fn startup(&mut self) {
        let result = unsafe {
            libphp_sapi_startup();
            sapi_startup(&mut self.module as *mut _);
            php_module_startup(&mut self.module as *mut _, std::ptr::null_mut());
            php_request_startup()
        };

        assert!(result == ZEND_RESULT_CODE_SUCCESS);
    }

    pub fn shutdown(&mut self) {
        unsafe {
            php_request_shutdown(std::ptr::null_mut());
            php_module_shutdown();
            sapi_shutdown();
        }
    }
}

extern "C" fn sapi_send_header(_: *mut sapi_header_struct, _: *mut c_void) {}