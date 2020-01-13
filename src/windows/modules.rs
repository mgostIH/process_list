use super::RAIIHandle;
use std::io;
use std::path::Path;
use winapi::shared::minwindef::TRUE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE,
};

/// Computes a function for each module found in the process with id `process_id`.
///
/// The function `f` takes the module base address with its size and it's name as parameters and can do whatever needed.
///
/// Modules that have an invalid UTF-8 name are ignored and logged in warn level (May change in the future)
///
/// # Returns
/// This function returns the error if any of the internal WinAPI fails.
///
/// # Examples
/// Printing every module of the current executable
/// ```
/// use process_list::for_each_module;
/// use std::process;
///
/// fn print_stuff() {
///     env_logger::init();
///     for_each_module(process::id(), |(address, size), name| {
///         println!("{:016X} - {} \t {}", address, size, name.display())
///     })
///     .unwrap();
/// }
///
/// ```

pub fn for_each_module<F>(process_id: u32, mut f: F) -> io::Result<()>
where
    F: FnMut((usize, usize), &Path),
{
    // Safe, we need to interface with WinAPI, there's not particular preconditions for the input.
    let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id) };
    debug!(
        "Called CreateToolHelp32Snapshot with return value {}",
        handle as usize
    );
    if handle == INVALID_HANDLE_VALUE {
        let error = io::Error::last_os_error();
        error!("CreateToolHelp32Snapshot failed with error {:?}", error);
        return Err(error);
    }
    let _guard = RAIIHandle::new(handle); // We don't actually use this but we want to call CloseHandle when we are done

    debug!("Handle is {}", handle as usize);
    // Safe because it's a WINAPI type, using MaybeUninit would be hard because we need to write on its dwSize field.
    let mut pe32: MODULEENTRY32 = unsafe { std::mem::zeroed() };
    // We must initialize dwSize here.
    pe32.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
    debug!("dwSize is {}", pe32.dwSize);
    let v = unsafe { Module32First(handle, &mut pe32) };
    if v != TRUE {
        let error = io::Error::last_os_error();
        error!(
            "Module32First returned {}, failed with error {:?}",
            v, error
        );
        return Err(error);
    }

    match get_module_data(&pe32) {
        Ok((address_size, name)) => f(address_size, name),
        Err(address) => warn!(
            "The module with address {:016X} didn't have an UTF8 valid name.",
            address
        ),
    }

    // Cleans back the storage we used to store the module name.
    trace!("Cleaning back module name.");
    pe32.szExePath
        .iter_mut()
        .take_while(|c| **c != 0)
        .for_each(|c| *c = 0);

    // This is safe because we are calling a WinAPI with the right parameters.
    while unsafe { Module32Next(handle, &mut pe32) } != 0 {
        match get_module_data(&pe32) {
            Ok((address_size, name)) => f(address_size, name),
            Err(address) => warn!(
                "The module with address {:016X} didn't have an UTF8 valid name.",
                address
            ),
        }
        // Cleans back the storage we used to store the module name.
        trace!("Cleaning back module name.");
        pe32.szExePath
            .iter_mut()
            .take_while(|c| **c != 0)
            .for_each(|c| *c = 0);
    }
    // No need to call CloseHandle as it's dealt by the RAIIHandle.
    Ok(())
}

fn get_module_data(module: &MODULEENTRY32) -> Result<((usize, usize), &Path), usize> {
    let address = module.modBaseAddr as usize;
    let size = module.modBaseSize as usize;
    let name = super::get_winstring(&module.szExePath).map_err(|_| address)?;
    trace!(
        "get_module_data: address = {:016X}, name = {}",
        address,
        name
    );
    Ok(((address, size), Path::new(name)))
}
