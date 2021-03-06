pub use self::context::Context;
pub use self::module::{Module, ModuleBuilder};
pub use self::function::{Function, FunctionLabel, FunctionBuilder};
pub use self::builder::{Builder, PositionedBuilder};
pub use self::block::{BasicBlock, Label};
pub use self::value::Value;
pub use self::phi::Phi;
pub use self::alloca::Alloca;
pub use self::constant::Constant;
pub use self::global::Global;
pub use self::ty::{Type, FunctionType, IntegerType, PointerType, ArrayType};
pub use self::target::{Target, TargetMachine, DataLayout};
pub use self::pass_manager::{FunctionPassManager, InitializedFunctionPassManager};

pub use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate};

pub mod init;
pub mod context;
pub mod module;
pub mod function;
pub mod block;
pub mod builder;
pub mod value;
pub mod phi;
pub mod alloca;
pub mod constant;
pub mod ty;
pub mod target;
pub mod pass_manager;
pub mod global;

//
// TODO: Error Checking
//   This is currently a safety hole.
//
// TODO: Reduce the number of lifetimes.
//   It isn't completely clear that this is possible, but this is a first attempt.
//
// TODO: Docs
//
// TODO: See what safety guarentees LLVM actually needs.
//   The official LLVM docs aren't very clear on, e.g., when different contexts/modules/function/builders
//   can be mixed, or what things can outlive other things.
//
// TODO: Consider wrapping the C++ API in another way.
//   The official wrapper is useful, since it requires no effort and is maintained by LLVM. However,
//   it has a number of annoying issues. For example, given the similarity between LLVM's Twine and Rust's
//   &str, there should be no reason to use CStr.
//
// TODO: Missing functionality
//
