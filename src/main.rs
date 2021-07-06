use std::{string, fs, io};
use std::io::Write;

fn call_dynamic(sf: &str, entry: &str) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(sf)?;
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(entry.as_ref())?;
        Ok(func())
    }
}
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}


fn main()  {
    println!("KCC 1.0 | Entry Point Search Program");
    print!("type a filename here: ");
    io::stdout().flush().expect("Couldn't flush stdout");
    let mut line: String = text_io::read!();
       // <-- here


    println!("\nloading file {}", line);
    if path_exists(&line)
    {
        println!("welcome to file {}. Type an entry point to load an entry point (or symbol). (example: int main() in file: main)", line);
        loop {
            io::stdout().flush().expect("Couldn't flush stdout");
            let mut entry: String = text_io::read!();
            call_dynamic(&line, &entry);
        }
    }

}
