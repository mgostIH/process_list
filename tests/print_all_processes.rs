use process_list::for_each_process;

#[test]
fn print_stuff() {
    env_logger::init();
    for_each_process(|id, name| println!("{} - {}", id, name.display())).unwrap();
}
