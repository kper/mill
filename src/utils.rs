use libc::c_char;
use llvm_sys::core::LLVMCreateMessage;
use std::ffi::CStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Eq)]
pub struct LLVMString {
    pub(crate) ptr: *const c_char,
}
impl LLVMString {
    #[allow(dead_code)]
    pub(crate) unsafe fn new(ptr: *const c_char) -> Self {
        LLVMString { ptr }
    }

    #[allow(dead_code)]
    pub(crate) fn create_from_c_str(string: &CStr) -> LLVMString {
        unsafe { LLVMString::new(LLVMCreateMessage(string.as_ptr() as *const _)) }
    }

    #[allow(dead_code)]
    pub(crate) fn create_from_str(string: &str) -> LLVMString {
        debug_assert_eq!(string.as_bytes()[string.as_bytes().len() - 1], 0);

        unsafe { LLVMString::new(LLVMCreateMessage(string.as_ptr() as *const _)) }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        (*self).to_string_lossy().into_owned()
    }
}

impl Deref for LLVMString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl Debug for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}

impl Display for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}

impl PartialEq for LLVMString {
    fn eq(&self, other: &LLVMString) -> bool {
        **self == **other
    }
}
