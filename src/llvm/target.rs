use std::ffi::CStr;
use std::mem;

pub use llvm_sys::target_machine::{LLVMCodeGenOptLevel, LLVMRelocMode, LLVMCodeModel};
use llvm_sys::target_machine::{LLVMTargetRef, LLVMGetTargetFromTriple, LLVMGetDefaultTargetTriple};
use llvm_sys::target_machine::{LLVMTargetMachineRef, LLVMCreateTargetMachine, LLVMDisposeTargetMachine};

use llvm_sys::target::{LLVMTargetDataRef, LLVMCopyStringRepOfTargetData, LLVMDisposeTargetData, LLVMCreateTargetData};
use llvm_sys::target_machine::LLVMGetTargetMachineData;

pub use llvm_sys::target_machine::LLVMCodeGenFileType;
use llvm_sys::target_machine::LLVMTargetMachineEmitToFile;

use ffi::MallocCStr;
use owned::{Owned, DropInPlace};

use llvm::module::Module;

pub fn default_triple() -> Owned<MallocCStr> {
    unsafe {
        MallocCStr::from_raw(LLVMGetDefaultTargetTriple())
    }
}

pub struct Target {
    _priv: ()
}

impl Target {
    pub fn from_triple(triple: &CStr) -> Result<&'static Target, Owned<MallocCStr>> {
        unsafe {
            let mut target_ptr = mem::uninitialized();
            let mut err_ptr = mem::uninitialized();
            if LLVMGetTargetFromTriple(triple.as_ptr(), &mut target_ptr, &mut err_ptr) == 0 {
                Ok(&*(target_ptr as *mut Target))
            } else {
                Err(MallocCStr::from_raw(err_ptr))
            }
        }
    }

    pub fn as_raw(&self) -> LLVMTargetRef {
        self as *const Target as LLVMTargetRef
    }
}

pub struct TargetMachine {
    _priv: ()
}

impl DropInPlace for TargetMachine {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeTargetMachine(self.as_raw());
    }
}

impl TargetMachine {
    pub fn new(target: &Target, triple: &CStr, cpu: &CStr, features: &CStr, opt_level: LLVMCodeGenOptLevel, reloc_mode: LLVMRelocMode, code_model: LLVMCodeModel) -> Owned<TargetMachine> {
        unsafe {
            Owned::from_raw(LLVMCreateTargetMachine(target.as_raw(), triple.as_ptr(), cpu.as_ptr(), features.as_ptr(), opt_level, reloc_mode, code_model) as *mut TargetMachine)
        }
    }

    pub fn emit_module_to_file<'cid, 'mid, 'context>(&self, module: &Module<'cid, 'mid, 'context>, filename: &CStr, codegen: LLVMCodeGenFileType) -> Result<(), Owned<MallocCStr>> {
        unsafe {
            let mut err_ptr = mem::uninitialized();
            if LLVMTargetMachineEmitToFile(self.as_raw(), module.as_raw(), filename.as_ptr() as *mut _, codegen, &mut err_ptr) == 0 {
                Ok(())
            } else {
                Err(MallocCStr::from_raw(err_ptr))
            }
        }
    }

    pub fn data_layout(&self) -> &DataLayout {
        unsafe {
            // TODO(3.9): Replace with LLVMCreateTargetDataLayout (returning Owned)
            &*(LLVMGetTargetMachineData(self.as_raw()) as *mut DataLayout)
        }
    }

    pub fn as_raw(&self) -> LLVMTargetMachineRef {
        self as *const TargetMachine as LLVMTargetMachineRef
    }
}

pub struct DataLayout {
    _priv: ()
}

impl DropInPlace for DataLayout {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeTargetData(self.as_raw())
    }
}

impl DataLayout {
    pub fn from_string(desc: &CStr) -> Owned<DataLayout> {
        unsafe {
            Owned::from_raw(LLVMCreateTargetData(desc.as_ptr()) as *mut DataLayout)
        }
    }

    pub fn as_string(&self) -> Owned<MallocCStr> {
        unsafe {
            MallocCStr::from_raw(LLVMCopyStringRepOfTargetData(self.as_raw()))
        }
    }

    pub fn as_raw(&self) -> LLVMTargetDataRef {
        self as *const DataLayout as LLVMTargetDataRef
    }
}
