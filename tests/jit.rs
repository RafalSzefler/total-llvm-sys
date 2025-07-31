pub use total_llvm_sys::llvm_sys as llvm;

use std::ffi::CStr;
use std::mem;

use llvm::LLVMContext;
use llvm::core::*;
use llvm::execution_engine::*;

struct Jit {
    context: *mut LLVMContext,
    engine: *mut LLVMOpaqueExecutionEngine,
    function: extern "C" fn(u64, u64, u64) -> u64,
}

impl Jit {
    pub fn new() -> Self {
        unsafe {
            // Set up a context, module and builder in that context.
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(b"sum\0".as_ptr() as *const _, context);
            let builder = LLVMCreateBuilderInContext(context);
    
            // get a type for sum function
            let i64t = LLVMInt64TypeInContext(context);
            let mut argts = [i64t, i64t, i64t];
            let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);
    
            // add it to our module
            let function = LLVMAddFunction(module, b"sum\0".as_ptr() as *const _, function_type);
    
            // Create a basic block in the function and set our builder to generate
            // code in it.
            let bb = LLVMAppendBasicBlockInContext(context, function, b"entry\0".as_ptr() as *const _);
    
            LLVMPositionBuilderAtEnd(builder, bb);
    
            // get the function's arguments
            let x = LLVMGetParam(function, 0);
            let y = LLVMGetParam(function, 1);
            let z = LLVMGetParam(function, 2);
    
            let sum = LLVMBuildAdd(builder, x, y, b"sum.1\0".as_ptr() as *const _);
            let sum = LLVMBuildAdd(builder, sum, z, b"sum.2\0".as_ptr() as *const _);
    
            // Emit a `ret i64` into the function to return the computed sum.
            LLVMBuildRet(builder, sum);
    
            // done building
            LLVMDisposeBuilder(builder);
    
            // Dump the module as IR to stdout.
            LLVMDumpModule(module);
    
            // Initialize native target
            LLVMLinkInMCJIT();
            total_llvm_sys::native::initialize_target();
            total_llvm_sys::native::initialize_asm_printer();
    
            // Build an execution engine.
            let engine = {
                let mut ee = mem::MaybeUninit::uninit();
                let mut err = mem::zeroed();
    
                // This moves ownership of the module into the execution engine.
                if LLVMCreateExecutionEngineForModule(ee.as_mut_ptr(), module, &mut err) != 0 {
                    // In case of error, we must avoid using the uninitialized ExecutionEngineRef.
                    assert!(!err.is_null());
                    panic!(
                        "Failed to create execution engine: {:?}",
                        CStr::from_ptr(err)
                    );
                }
    
                ee.assume_init()
            };

            let addr = LLVMGetFunctionAddress(engine, b"sum\0".as_ptr() as *const _);

            Self {
                context,
                engine,
                function: mem::transmute(addr),
            }
        }    
    }

    pub fn call(&self, x: u64, y: u64, z: u64) -> u64 {
        (self.function)(x, y, z)
    }
}

impl Drop for Jit {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.engine);
            LLVMContextDispose(self.context);
        }
    }
}

#[test]
fn test_jit() {
    let jit = Jit::new();

    for a in 0..20 {
        for b in 10..30 {
            for c in 678..693 {
                let result = jit.call(a, b, c);
                assert_eq!(result, a + b + c);
            }
        }
    }
}
