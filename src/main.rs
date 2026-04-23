mod macho;
mod parser;
use std::fs::File;
use std::error::Error;
use std::ptr;
use std::env;

fn main() -> Result<(), Box<dyn Error>>
{
    let fname = env::args().nth(1).unwrap();

    let mut f = File::open(fname)?;
    let f_contents = parser::read_entire_file(&mut f)?;
    let hdr = parser::parse_mach_header(&f_contents);

    println!("Executable type: {}", get_field!(hdr, file_type));

    let load_comms = parser::read_load_commands(&f_contents, hdr.load_commands_n);

    println!("First segment name: {}", String::from_utf8_lossy(&load_comms[0].segment_name));

    Ok(())
}
