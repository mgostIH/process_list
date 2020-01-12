# Process Listing
This crate exposes a `for_each_processes` function to deal sequentially with every process open in the operating system.

## Example
Printing every process to `stdout`
```Rust
use process_list::for_each_process;
fn print_processes(id : u32, name : &str) {
    println!("Id: {} --- Name: {}", id, name);
}
for_each_process(print_processes).unwrap();
```

# Support
For now only Windows is supported, but it should be simple enough to port on other operating systems. 

It's not a priority but pull requests are well accepted.
