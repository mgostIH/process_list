mod processes;

use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::CloseHandle;

pub use processes::for_each_process;
#[repr(transparent)]
pub struct RAIIHandle(pub HANDLE);

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
