use libc::c_uint;
use ffi;
use foreign_types::ForeignTypeRef;
use std::net::IpAddr;

use cvt;
use error::ErrorStack;

bitflags! {
    pub struct X509CheckFlags: c_uint {
        const X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT;
        const X509_CHECK_FLAG_NO_WILDCARDS = ffi::X509_CHECK_FLAG_NO_WILDCARDS;
        const X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS = ffi::X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS;
        const X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS = ffi::X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS;
        const X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS
            = ffi::X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS;
        /// Requires the `v110` feature and OpenSSL 1.1.0.
        #[cfg(all(feature = "v110", ossl110))]
        const X509_CHECK_FLAG_NEVER_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_NEVER_CHECK_SUBJECT;
    }
}

foreign_type_and_impl_send_sync! {
    type CType = ffi::X509_VERIFY_PARAM;
    fn drop = ffi::X509_VERIFY_PARAM_free;

    pub struct X509VerifyParam;
    pub struct X509VerifyParamRef;
}

impl X509VerifyParamRef {
    pub fn set_hostflags(&mut self, hostflags: X509CheckFlags) {
        unsafe {
            ffi::X509_VERIFY_PARAM_set_hostflags(self.as_ptr(), hostflags.bits);
        }
    }

    pub fn set_host(&mut self, host: &str) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::X509_VERIFY_PARAM_set1_host(
                self.as_ptr(),
                host.as_ptr() as *const _,
                host.len(),
            )).map(|_| ())
        }
    }

    pub fn set_ip(&mut self, ip: IpAddr) -> Result<(), ErrorStack> {
        unsafe {
            let mut buf = [0; 16];
            let len = match ip {
                IpAddr::V4(addr) => {
                    buf[..4].copy_from_slice(&addr.octets());
                    4
                }
                IpAddr::V6(addr) => {
                    buf.copy_from_slice(&addr.octets());
                    16
                }
            };
            cvt(ffi::X509_VERIFY_PARAM_set1_ip(
                self.as_ptr(),
                buf.as_ptr() as *const _,
                len,
            )).map(|_| ())
        }
    }
}
