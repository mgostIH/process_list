# Functionalities
This crate exposes functionalities for dealing with processes and modules loaded inside them in a streaming manner.

## Process Listing
This crate exposes a `for_each_process` function to deal sequentially with every process open in the operating system.

### Example
Printing every process to `stdout`
```Rust
use std::path::{Path, PathBuf};
use process_list::for_each_process;
fn print_processes(id : u32, name : &Path) {
    println!("Id: {} --- Name: {}", id, name.display());
}

for_each_process(print_processes).unwrap();
```

## Modules Listing
This crate exposes a `for_each_module` function to deal sequentially with every module loaded in a process.

### Example
Printing every module loaded in the current process to `stdout`
```Rust
use process_list::for_each_module;
use std::process;

fn print_stuff() {
    env_logger::init();
    for_each_module(process::id(), |(address, size), name| {
        println!("{:016X} - {} \t {}", address, size, name.display())
    })
    .unwrap();
}

```

# Features
You can enable the `log` feature of this crate in order to get logging from it.

# Support
For now only Windows is supported, but it should be simple enough to port on other operating systems. 

It's not a priority but pull requests are well accepted.

