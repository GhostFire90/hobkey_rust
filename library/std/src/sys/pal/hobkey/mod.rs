use crate::io::ErrorKind;

pub(crate) mod start;
pub(crate) mod alloc;


#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unsupported/env.rs"]
pub mod env;

#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;

#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/time.rs"]
pub mod time;

pub fn unsupported<T>() -> crate::io::Result<T>{
    Err(unsupported_err())
}
pub fn unsupported_err() -> crate::io::Error{
    crate::io::const_error!(
        crate::io::ErrorKind::Unsupported,
        "This isnt ready yet Persephone!",
    )
}
pub fn abort_internal() -> !{
    loop{   }
}

// PANIC STUFF
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn __rust_abort(){
    abort_internal();
}

pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe:u8){

}
pub unsafe fn cleanup(){}

#[inline]
pub(crate) fn is_interrupted(_errno: i32) -> bool{
    // TODO Actual interrupt check
    false 
}

pub fn decode_error_kind(_errno : i32) -> ErrorKind{
    ErrorKind::Other
}