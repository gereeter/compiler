use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;

use llvm::context::Context;

#[derive(Copy, Clone)]
pub struct Type<'cid> {
    _context_id: IdRef<'cid>,
    llvm_type: LLVMTypeRef
}

impl<'cid> Type<'cid> {
    pub fn f64(context: &Context<'cid>) -> Type<'cid> {
       Type {
           _context_id: IdRef::new(),
           llvm_type: unsafe { LLVMDoubleTypeInContext(context.as_raw()) }
       }
    }

    pub fn function(params: &[Type<'cid>], ret: Type<'cid>) -> Type<'cid> {
        Type {
            _context_id: IdRef::new(),
            // FIXME: actually enforce that Type and LLVMTypeRef are the same in memory
            llvm_type: unsafe { LLVMFunctionType(ret.llvm_type, params.as_ptr() as *mut LLVMTypeRef, params.len() as u32, 0) }
        }
    }

    pub fn as_raw(&self) -> LLVMTypeRef {
        self.llvm_type
    }
}
