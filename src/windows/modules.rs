use std::io;


pub fn for_each_module<F>(process_id : u32, mut f: F) -> io::Result<()>
where
    F: FnMut(u32, &str),
{
    unimplemented!()
}