mod modules;
mod processes;
pub use modules::for_each_module;
pub use processes::for_each_process;

use std::str::{from_utf8, Utf8Error};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::HANDLE;

#[repr(transparent)]
pub(self) struct RAIIHandle(pub HANDLE);

impl RAIIHandle {
    pub fn new(handle: HANDLE) -> RAIIHandle {
        RAIIHandle(handle)
    }
}

impl Drop for RAIIHandle {
    fn drop(&mut self) {
        debug!("Calling CloseHandle from the RAIIHandle's drop.");
        // This never gives problem except when running under a debugger.
        unsafe { CloseHandle(self.0) };
    }
}

// This is basically from_utf8 with a "transmute" from &[i8] to &[u8]
pub(self) fn get_winstring<'a>(data: &[i8]) -> Result<&'a str, Utf8Error> {
    let name: &'a [u8] = unsafe { std::slice::from_raw_parts(data.as_ptr().cast(), data.len()) };
    from_utf8(name)
}
