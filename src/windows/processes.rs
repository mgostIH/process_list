use std::io;
use winapi::shared::minwindef::TRUE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use super::RAIIHandle;

/// Computes a function for each process found, returns an error if any of the WinAPI failed.
///
/// The function `f` takes the process id and it's name as parameters and can do whatever needed.
///
/// Processes that have an invalid UTF-8 name are ignored and logged in warn level (May change in the future)
///
/// # Examples
/// Printing every process to `stdout`
/// ```
/// use process_list::for_each_process;
/// fn print_processes(id : u32, name : &str) {
///     println!("Id: {} --- Name: {}", id, name);
/// }
///
/// for_each_process(print_processes).unwrap();
/// ```
///
/// # Examples
/// Getting all the processes into a `Vec`
/// ```
/// use process_list::for_each_process;
/// let mut data : Vec<(u32, String)> = Vec::new();
/// for_each_process(|id, name| data.push( (id, name.to_string()) )).unwrap();
/// // Now `data` holds all the current processes id-name pairs.
/// ```
pub fn for_each_process<F>(mut f: F) -> io::Result<()>
where
    F: FnMut(u32, &str),
{
    // Safe, we need to interface with WinAPI, there's not particular preconditions for the input.
    let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
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
    let mut pe32: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    // We must initialize dwSize here.
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
    debug!("dwSize is {}", pe32.dwSize);
    let v = unsafe { Process32First(handle, &mut pe32) };
    if v != TRUE {
        let error = io::Error::last_os_error();
        error!(
            "Process32First returned {}, failed with error {:?}",
            v, error
        );
        return Err(error);
    }

    match get_process_data(&pe32) {
        Ok((id, name)) => f(id, name),
        // Don't change the underscore from id
        Err(_id) => warn!("The process with id {} didn't have an UTF8 valid name.", _id),
    }

    // Cleans back the storage we used to store the process name.
    trace!("Cleaning back process name.");
    pe32.szExeFile
        .iter_mut()
        .take_while(|c| **c != 0)
        .for_each(|c| *c = 0);

    // This is safe because we are calling a WinAPI with the right parameters.
    while unsafe { Process32Next(handle, &mut pe32) } != 0 {
        match get_process_data(&pe32) {
            Ok((id, name)) => f(id, name),
            Err(_id) => warn!("The process with id {} didn't have an UTF8 valid name.", _id),
        }
        // Cleans back the storage we used to store the process name.
        trace!("Cleaning back process name.");
        pe32.szExeFile
            .iter_mut()
            .take_while(|c| **c != 0)
            .for_each(|c| *c = 0);
    }
    // No need to call CloseHandle as it's dealt by the RAIIHandle.
    Ok(())
}

// We use an explicit lifetime because we are changing a pointer to i8 to a pointer to u8.
fn get_process_data<'a>(process: &'a PROCESSENTRY32) -> Result<(u32, &'a str), u32> {
    let id = process.th32ProcessID;
    let name = super::get_winstring(&process.szExeFile).map_err(|_| id)?;
    trace!("get_process_data: id = {}, name = {}", id, name);
    Ok((id, name))
}