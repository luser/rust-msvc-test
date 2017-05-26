extern crate gcc;

fn main() {
    let tool = gcc::Config::new()
        .opt_level(1)
        .host("x86_64-pc-windows-msvc")
        .target("x86_64-pc-windows-msvc")
        .debug(false)
        .get_compiler();
    println!("64-bit MSVC path: {:?}", tool.path());
    let tool = gcc::Config::new()
        .opt_level(1)
        .host("x86_64-pc-windows-msvc")
        .target("i686-pc-windows-msvc")
        .debug(false)
        .get_compiler();
    println!("32-bit MSVC path: {:?}", tool.path());
}
