use process_list::for_each_module;
use std::process;
#[test]
fn print_stuff() {
    env_logger::init();
    for_each_module(process::id(), |(address, size), name| {
        println!("{:016X} - {} \t {}", address, size, name.display())
    })
    .unwrap();
}
