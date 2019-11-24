use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;
use inheritance::DerivesFrom;
use opaque::Opaque;

pub struct Value<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>,
    _opaque: Opaque
}
unsafe impl<'cid, 'mid, 'fid> DerivesFrom<Value<'cid, 'mid, 'fid>> for Value<'cid, 'mid, 'fid> { }

impl<'cid, 'mid, 'fid> Value<'cid, 'mid, 'fid> {
    // FIXME: Should this require a mutable reference?
    pub fn set_name(&self, name: &str) {
        unsafe {
            LLVMSetValueName2(self.as_raw(), name.as_ptr() as *const std::os::raw::c_char, name.len());
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Value as *mut Value as LLVMValueRef
    }
}
